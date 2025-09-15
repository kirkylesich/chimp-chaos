#![forbid(unsafe_code)]
#![deny(warnings)]
#![warn(clippy::pedantic)]

use std::sync::Arc;

use crate::state::SharedStateRef;
use adapters_k8s::{NoopAgents, NoopMesh, NoopMetrics, NoopSink, NoopTrace, SystemClock};
use domain::ports::{AgentClient, Clock, MeshAdapter, MetricsBackend, ReportSink, TraceBackend};

#[derive(Clone)]
pub struct App {
    pub mesh: Arc<dyn MeshAdapter>,
    pub agents: Arc<dyn AgentClient>,
    pub metrics: Arc<dyn MetricsBackend>,
    pub traces: Arc<dyn TraceBackend>,
    pub sink: Arc<dyn ReportSink>,
    pub clock: Arc<dyn Clock>,
    pub state: SharedStateRef,
}

pub fn build_app() -> App {
    App {
        mesh: Arc::new(NoopMesh::default()),
        agents: Arc::new(NoopAgents::default()),
        metrics: Arc::new(NoopMetrics::default()),
        traces: Arc::new(NoopTrace::default()),
        sink: Arc::new(NoopSink::default()),
        clock: Arc::new(SystemClock::default()),
        state: std::sync::Arc::new(crate::state::SharedState::new()),
    }
}

pub mod controller;
pub mod http;
pub mod state;
