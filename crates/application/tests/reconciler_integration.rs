#![forbid(unsafe_code)]
#![deny(warnings)]
#![warn(clippy::pedantic)]

use application::usecases::Reconciler;
use domain::models::{ChaosExperimentSpec, ChaosExperimentStatus, ExperimentRef, LoadProfile, Mode, Target, Phase};
use domain::tests_support::{ConstMetrics, MemoryReport, NoopAgent, NoopMesh, NoopTrace, TestClock};
use std::collections::HashMap;

#[tokio::test]
async fn reconciler_happy_path() {
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
    assert!(matches!(status.phase, Phase::Running | Phase::Completed));
}

