use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::models::{ChaosExperimentSpec, ChaosReport, Edge};

/// Mesh control port (e.g., Istio)
#[async_trait]
pub trait MeshAdapter: Send + Sync {
    async fn apply_fault(&self, spec: &ChaosExperimentSpec) -> Result<(), crate::errors::DomainError>;
    async fn rollback_fault(&self, spec: &ChaosExperimentSpec) -> Result<(), crate::errors::DomainError>;
}

/// Node agents port
#[async_trait]
pub trait AgentClient: Send + Sync {
    async fn start_load(&self, spec: &ChaosExperimentSpec) -> Result<(), crate::errors::DomainError>;
    async fn stop_load(&self, spec: &ChaosExperimentSpec) -> Result<(), crate::errors::DomainError>;
}

/// Metrics backend port
#[async_trait]
pub trait MetricsBackend: Send + Sync {
    async fn fetch_value(&self, query: &str, window: &str) -> Result<f64, crate::errors::DomainError>;
}

/// Tracing backend port
#[async_trait]
pub trait TraceBackend: Send + Sync {
    async fn fetch_edges(&self, window: &str) -> Result<Vec<Edge>, crate::errors::DomainError>;
}

/// Report sink port
#[async_trait]
pub trait ReportSink: Send + Sync {
    async fn store_report(&self, report: &ChaosReport) -> Result<(), crate::errors::DomainError>;
}

/// Clock port
pub trait Clock: Send + Sync {
    fn now(&self) -> DateTime<Utc>;
}

