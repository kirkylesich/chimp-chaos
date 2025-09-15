use chrono::Duration;
use domain::errors::DomainError;
use domain::models::{Artifacts, ChaosExperimentSpec, ChaosExperimentStatus, ChaosReport, Edge, ExperimentRef, Phase, ServiceResult, Topology, Windows};
use domain::ports::{AgentClient, Clock, MeshAdapter, MetricsBackend, ReportSink, TraceBackend};
use tracing::instrument;

/// Reconcile use-case service
pub struct Reconciler<'a> {
    pub mesh: &'a dyn MeshAdapter,
    pub agents: &'a dyn AgentClient,
    pub metrics: &'a dyn MetricsBackend,
    pub traces: &'a dyn TraceBackend,
    pub sink: &'a dyn ReportSink,
    pub clock: &'a dyn Clock,
}

impl<'a> Reconciler<'a> {
    pub fn new(
        mesh: &'a dyn MeshAdapter,
        agents: &'a dyn AgentClient,
        metrics: &'a dyn MetricsBackend,
        traces: &'a dyn TraceBackend,
        sink: &'a dyn ReportSink,
        clock: &'a dyn Clock,
    ) -> Self {
        Self { mesh, agents, metrics, traces, sink, clock }
    }

    #[instrument(skip_all, fields(ns = %exp_ref.namespace, experiment = %exp_ref.name))]
    pub async fn reconcile(
        &self,
        spec: &ChaosExperimentSpec,
        status: &mut ChaosExperimentStatus,
        exp_ref: &ExperimentRef,
    ) -> Result<(), DomainError> {
        match status.phase {
            Phase::Pending => {
                self.validate_spec(spec)?;
                self.compute_windows(spec, status)?;
                status.started_at = Some(self.clock.now());
                status.ends_at = Some(self.clock.now() + Duration::seconds(spec.duration_seconds as i64));
                status.phase = Phase::Running;
            }
            Phase::Running => {
                self.maybe_apply_mesh(spec, status).await?;
                self.maybe_start_agents(spec).await?;
                if let Some(ends_at) = status.ends_at {
                    if self.clock.now() >= ends_at {
                        self.finalize(spec, status, exp_ref).await?;
                    }
                }
            }
            Phase::Stopping => {
                self.finalize(spec, status, exp_ref).await?;
            }
            Phase::Completed | Phase::Error => {}
        }
        Ok(())
    }

    fn validate_spec(&self, spec: &ChaosExperimentSpec) -> Result<(), DomainError> {
        if spec.targets.is_empty() {
            return Err(DomainError::message("targets must not be empty"));
        }
        if spec.duration_seconds == 0 {
            return Err(DomainError::message("durationSeconds must be > 0"));
        }
        Ok(())
    }

    fn compute_windows(&self, spec: &ChaosExperimentSpec, status: &mut ChaosExperimentStatus) -> Result<(), DomainError> {
        // Baseline window: [-2*duration, -duration], failure: [now, now+duration]
        let d = spec.duration_seconds as i64;
        let baseline = format!("now-{}s_to_now-{}s", 2 * d, d);
        let failure = format!("now_to_now+{}s", d);
        status.baseline_window = Some(baseline);
        status.failure_window = Some(failure);
        Ok(())
    }

    async fn maybe_apply_mesh(&self, spec: &ChaosExperimentSpec, status: &mut ChaosExperimentStatus) -> Result<(), DomainError> {
        use domain::models::Mode::*;
        match spec.mode {
            Mesh | Mixed => {
                if !status.istio_patched {
                    self.mesh.apply_fault(spec).await?;
                    status.istio_patched = true;
                }
            }
            Agent => {}
        }
        Ok(())
    }

    async fn maybe_start_agents(&self, spec: &ChaosExperimentSpec) -> Result<(), DomainError> {
        use domain::models::Mode::*;
        match spec.mode {
            Agent | Mixed => self.agents.start_load(spec).await,
            Mesh => Ok(()),
        }
    }

    async fn finalize(&self, spec: &ChaosExperimentSpec, status: &mut ChaosExperimentStatus, exp_ref: &ExperimentRef) -> Result<(), DomainError> {
        use domain::models::Mode::*;
        match spec.mode {
            Mesh | Mixed => {
                if status.istio_patched {
                    self.mesh.rollback_fault(spec).await?;
                    status.istio_patched = false;
                }
            }
            Agent => {}
        }
        match spec.mode {
            Agent | Mixed => self.agents.stop_load(spec).await?,
            Mesh => {}
        }
        let (base_win, fail_win) = match (&status.baseline_window, &status.failure_window) {
            (Some(b), Some(f)) => (b.as_str(), f.as_str()),
            _ => return Err(DomainError::message("windows not computed")),
        };
        let rps_base = self.metrics.fetch_value("rps_base", base_win).await?;
        let rps_fail = self.metrics.fetch_value("rps_fail", fail_win).await?;
        let p95_base = self.metrics.fetch_value("p95_base", base_win).await?;
        let p95_fail = self.metrics.fetch_value("p95_fail", fail_win).await?;
        let impact = compute_impact_score(rps_base, rps_fail, p95_base, p95_fail)?;
        let edges = self.traces.fetch_edges(fail_win).await.unwrap_or_default();
        let report = ChaosReport {
            experiment_ref: exp_ref.clone(),
            windows: Windows { baseline: base_win.to_string(), failure: fail_win.to_string() },
            results: vec![ServiceResult { name: first_target_service(spec), rps_base, rps_fail, p95_base_ms: p95_base, p95_fail_ms: p95_fail, impact_score: impact }],
            topology: Topology { nodes: collect_nodes(&edges), edges },
            artifacts: Artifacts { json: "{}".to_string() },
        };
        self.sink.store_report(&report).await?;
        status.phase = Phase::Completed;
        Ok(())
    }
}

fn first_target_service(spec: &ChaosExperimentSpec) -> String {
    spec.targets.get(0).map(|t| t.service.clone()).unwrap_or_else(|| "unknown".to_string())
}

fn collect_nodes(edges: &[Edge]) -> Vec<String> {
    let mut nodes = Vec::new();
    for e in edges {
        if !nodes.contains(&e.from) { nodes.push(e.from.clone()); }
        if !nodes.contains(&e.to) { nodes.push(e.to.clone()); }
    }
    nodes
}

/// ImpactScore формула из ТЗ
pub fn compute_impact_score(rps_base: f64, rps_fail: f64, p95_base_ms: f64, p95_fail_ms: f64) -> Result<f64, DomainError> {
    if rps_base <= 0.0 || p95_base_ms <= 0.0 {
        return Err(DomainError::message("baseline values must be > 0"));
    }
    let rps_part = ((rps_base - rps_fail) / rps_base) * 100.0;
    let lat_part = ((p95_fail_ms - p95_base_ms) / p95_base_ms) * 100.0;
    Ok(rps_part + lat_part)
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::models::{LoadProfile, Mode, Target};
    use domain::tests_support::{ConstMetrics, MemoryReport, NoopAgent, NoopMesh, NoopTrace, TestClock};
    use std::collections::HashMap;

    #[tokio::test]
    async fn impact_score_positive() {
        let score = compute_impact_score(100.0, 50.0, 200.0, 400.0).unwrap();
        assert!(score > 0.0);
    }

    #[tokio::test]
    async fn reconcile_minimal() {
        let mesh = NoopMesh::default();
        let agents = NoopAgent::default();
        let metrics = ConstMetrics(100.0);
        let traces = NoopTrace::default();
        let sink = MemoryReport::default();
        let clock = TestClock::default();
        let mut status = ChaosExperimentStatus { phase: Phase::Pending, started_at: None, ends_at: None, baseline_window: None, failure_window: None, istio_patched: false, snapshot: None };
        let spec = ChaosExperimentSpec { targets: vec![Target { service: "svc".into(), r#match: None }], mode: Mode::Mesh, duration_seconds: 1, load_profile: LoadProfile { r#type: "none".into(), rps: None, connections: None }, mesh_fault: None, agent_fault: None, labels: HashMap::new() };
        let exp_ref = ExperimentRef { namespace: "ns".into(), name: "name".into() };
        let rec = Reconciler::new(&mesh, &agents, &metrics, &traces, &sink, &clock);
        rec.reconcile(&spec, &mut status, &exp_ref).await.unwrap();
        // After first reconcile we either enter Running or immediately Completed
        assert!(matches!(status.phase, Phase::Running | Phase::Completed));
    }
}

