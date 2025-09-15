use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ChaosExperiment.spec
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChaosExperimentSpec {
    pub targets: Vec<Target>,
    pub mode: Mode,
    pub duration_seconds: u32,
    pub load_profile: LoadProfile,
    pub mesh_fault: Option<MeshFault>,
    pub agent_fault: Option<AgentFault>,
    pub labels: HashMap<String, String>,
}

/// Target service selection
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Target {
    pub service: String,
    #[serde(rename = "match")]
    pub r#match: Option<Match>,
}

/// HTTP match rules
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Match {
    pub http: Option<Vec<HttpMatch>>,
}

/// HTTP prefix
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HttpMatch {
    pub prefix: Option<String>,
}

/// Experiment mode
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Mode {
    Agent,
    Mesh,
    Mixed,
}

/// Load profile
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LoadProfile {
    #[serde(rename = "type")]
    pub r#type: String,
    pub rps: Option<u32>,
    pub connections: Option<u32>,
}

/// Mesh-level fault injection
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MeshFault {
    pub delay_ms: Option<u32>,
    pub delay_percent: Option<u8>,
    pub abort_http: Option<u16>,
    pub abort_percent: Option<u8>,
}

/// Agent fault parameters
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AgentFault {
    pub timeout_ms: Option<u32>,
    pub jitter_ms: Option<u32>,
    pub packet_loss_percent: Option<u8>,
}

/// ChaosExperiment.status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChaosExperimentStatus {
    pub phase: Phase,
    pub started_at: Option<DateTime<Utc>>,
    pub ends_at: Option<DateTime<Utc>>,
    pub baseline_window: Option<String>,
    pub failure_window: Option<String>,
    pub istio_patched: bool,
    pub snapshot: Option<Snapshot>,
}

/// Experiment phase
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Phase {
    Pending,
    Running,
    Stopping,
    Completed,
    Error,
}

/// Snapshot of original objects
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Snapshot {
    pub virtual_service_original: Option<serde_json::Value>,
}

/// Call graph edge
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Edge {
    pub from: String,
    pub to: String,
}

/// Reference to experiment
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExperimentRef {
    pub namespace: String,
    pub name: String,
}

/// Observation windows
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Windows {
    pub baseline: String,
    pub failure: String,
}

/// Per-service results
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ServiceResult {
    pub name: String,
    pub rps_base: f64,
    pub rps_fail: f64,
    pub p95_base_ms: f64,
    pub p95_fail_ms: f64,
    pub impact_score: f64,
}

/// Topology
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Topology {
    pub nodes: Vec<String>,
    pub edges: Vec<Edge>,
}

/// Extra artifacts
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Artifacts {
    pub json: String,
}

/// ChaosReport
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChaosReport {
    pub experiment_ref: ExperimentRef,
    pub windows: Windows,
    pub results: Vec<ServiceResult>,
    pub topology: Topology,
    pub artifacts: Artifacts,
}
