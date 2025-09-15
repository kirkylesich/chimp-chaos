use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::errors::DomainError;
use crate::models::{ChaosExperimentSpec, ChaosReport, Edge};
use crate::ports::{AgentClient, Clock, MeshAdapter, MetricsBackend, ReportSink, TraceBackend};

#[derive(Default, Clone)]
pub struct TestClock;

impl Clock for TestClock {
    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}

#[derive(Default, Clone)]
pub struct NoopMesh;

#[async_trait]
impl MeshAdapter for NoopMesh {
    async fn apply_fault(&self, _spec: &ChaosExperimentSpec) -> Result<(), DomainError> {
        Ok(())
    }
    async fn rollback_fault(&self, _spec: &ChaosExperimentSpec) -> Result<(), DomainError> {
        Ok(())
    }
}

#[derive(Default, Clone)]
pub struct NoopAgent;

#[async_trait]
impl AgentClient for NoopAgent {
    async fn start_load(&self, _spec: &ChaosExperimentSpec) -> Result<(), DomainError> {
        Ok(())
    }
    async fn stop_load(&self, _spec: &ChaosExperimentSpec) -> Result<(), DomainError> {
        Ok(())
    }
}

#[derive(Default, Clone)]
pub struct ConstMetrics(pub f64);

#[async_trait]
impl MetricsBackend for ConstMetrics {
    async fn fetch_value(&self, _query: &str, _window: &str) -> Result<f64, DomainError> {
        Ok(self.0)
    }
}

#[derive(Default, Clone)]
pub struct NoopTrace;

#[async_trait]
impl TraceBackend for NoopTrace {
    async fn fetch_edges(&self, _window: &str) -> Result<Vec<Edge>, DomainError> {
        Ok(Vec::new())
    }
}

#[derive(Default, Clone)]
pub struct MemoryReport;

#[async_trait]
impl ReportSink for MemoryReport {
    async fn store_report(&self, _report: &ChaosReport) -> Result<(), DomainError> {
        Ok(())
    }
}

