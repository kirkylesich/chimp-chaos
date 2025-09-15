#![forbid(unsafe_code)]
#![deny(warnings)]
#![warn(clippy::pedantic)]

use async_trait::async_trait;
use domain::errors::DomainError;
use domain::ports::{AgentClient, Clock, MeshAdapter, MetricsBackend, ReportSink, TraceBackend};

/// Нулевые реализации портов для безопасного локального запуска без кластера

#[derive(Default, Clone)]
pub struct NoopMesh;

#[async_trait]
impl MeshAdapter for NoopMesh {
    async fn apply_fault(
        &self,
        _spec: &domain::models::ChaosExperimentSpec,
    ) -> Result<(), DomainError> {
        Ok(())
    }
    async fn rollback_fault(
        &self,
        _spec: &domain::models::ChaosExperimentSpec,
    ) -> Result<(), DomainError> {
        Ok(())
    }
}

#[derive(Default, Clone)]
pub struct NoopAgents;

#[async_trait]
impl AgentClient for NoopAgents {
    async fn start_load(
        &self,
        _spec: &domain::models::ChaosExperimentSpec,
    ) -> Result<(), DomainError> {
        Ok(())
    }
    async fn stop_load(
        &self,
        _spec: &domain::models::ChaosExperimentSpec,
    ) -> Result<(), DomainError> {
        Ok(())
    }
}

#[derive(Default, Clone)]
pub struct NoopMetrics;

#[async_trait]
impl MetricsBackend for NoopMetrics {
    async fn fetch_value(&self, _query: &str, _window: &str) -> Result<f64, DomainError> {
        Ok(1.0)
    }
}

#[derive(Default, Clone)]
pub struct NoopTrace;

#[async_trait]
impl TraceBackend for NoopTrace {
    async fn fetch_edges(&self, _window: &str) -> Result<Vec<domain::models::Edge>, DomainError> {
        Ok(Vec::new())
    }
}

#[derive(Default, Clone)]
pub struct NoopSink;

#[async_trait]
impl ReportSink for NoopSink {
    async fn store_report(&self, _report: &domain::models::ChaosReport) -> Result<(), DomainError> {
        Ok(())
    }
}

#[derive(Default, Clone)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> chrono::DateTime<chrono::Utc> {
        chrono::Utc::now()
    }
}
