#![forbid(unsafe_code)]
#![deny(warnings)]
#![warn(clippy::pedantic)]

use chrono::{DateTime, Utc};
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, CustomResource, Deserialize, Serialize, JsonSchema)]
#[kube(
    group = "chaos.k8s.local",
    version = "v1alpha1",
    kind = "ChaosExperiment",
    namespaced
)]
#[kube(status = "ChaosExperimentStatus")]
pub struct ChaosExperimentSpec {
    pub targets: Vec<Target>,
    pub mode: Mode,
    pub duration_seconds: u32,
    pub load_profile: LoadProfile,
    pub mesh_fault: Option<MeshFault>,
    pub agent_fault: Option<AgentFault>,
    pub labels: std::collections::HashMap<String, String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct Target {
    pub service: String,
    #[serde(rename = "match")]
    pub r#match: Option<Match>,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct Match {
    pub http: Option<Vec<HttpMatch>>,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct HttpMatch {
    pub prefix: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub enum Mode {
    Agent,
    Mesh,
    Mixed,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct LoadProfile {
    #[serde(rename = "type")]
    pub r#type: String,
    pub rps: Option<u32>,
    pub connections: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct MeshFault {
    pub delay_ms: Option<u32>,
    pub delay_percent: Option<u8>,
    pub abort_http: Option<u16>,
    pub abort_percent: Option<u8>,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct AgentFault {
    pub timeout_ms: Option<u32>,
    pub jitter_ms: Option<u32>,
    pub packet_loss_percent: Option<u8>,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct ChaosExperimentStatus {
    pub phase: Phase,
    pub started_at: Option<DateTime<Utc>>,
    pub ends_at: Option<DateTime<Utc>>,
    pub baseline_window: Option<String>,
    pub failure_window: Option<String>,
    pub istio_patched: bool,
    pub snapshot: Option<Snapshot>,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub enum Phase {
    Pending,
    Running,
    Stopping,
    Completed,
    Error,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct Snapshot {
    pub virtual_service_original: Option<serde_json::Value>,
}

#[derive(Clone, Debug, CustomResource, Deserialize, Serialize, JsonSchema)]
#[kube(
    group = "chaos.k8s.local",
    version = "v1alpha1",
    kind = "ChaosReport",
    namespaced
)]
pub struct ChaosReportSpec {
    pub experiment_ref: ExperimentRef,
    pub windows: Windows,
    pub results: Vec<ServiceResult>,
    pub topology: Topology,
    pub artifacts: Artifacts,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct ExperimentRef {
    pub namespace: String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct Windows {
    pub baseline: String,
    pub failure: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct ServiceResult {
    pub name: String,
    pub rps_base: f64,
    pub rps_fail: f64,
    pub p95_base_ms: f64,
    pub p95_fail_ms: f64,
    pub impact_score: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct Topology {
    pub nodes: Vec<String>,
    pub edges: Vec<Edge>,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct Edge {
    pub from: String,
    pub to: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct Artifacts {
    pub json: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_serialize_specs() {
        let _ = serde_json::to_string(&LoadProfile {
            r#type: "none".into(),
            rps: None,
            connections: None,
        })
        .unwrap();
    }
}
