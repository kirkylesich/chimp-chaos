#![forbid(unsafe_code)]
#![deny(warnings)]
#![warn(clippy::pedantic)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExperimentRef {
    pub namespace: String,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChaosExperimentSpec {
    pub targets: Vec<Target>,
    pub mode: Mode,
    pub duration_seconds: u32,
    pub load_profile: LoadProfile,
    pub mesh_fault: Option<String>,
    pub agent_fault: Option<String>,
    pub labels: HashMap<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Target {
    pub service: String,
    pub r#match: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mode {
    Mesh,
    Agent,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoadProfile {
    pub r#type: String,
    pub rps: Option<u32>,
    pub connections: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChaosReport {
    pub experiment_ref: ExperimentRef,
    pub windows: Windows,
    pub results: Vec<String>,
    pub topology: Topology,
    pub artifacts: Artifacts,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Windows {
    pub baseline: String,
    pub failure: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Topology {
    pub nodes: Vec<String>,
    pub edges: Vec<(String, String)>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Artifacts {
    pub json: String,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Phase {
    Pending,
    Running,
    Completed,
    Failed,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChaosExperimentStatus {
    pub phase: Phase,
    pub started_at: Option<i64>,
    pub ends_at: Option<i64>,
    pub baseline_window: Option<String>,
    pub failure_window: Option<String>,
    pub istio_patched: bool,
    pub snapshot: Option<String>,
}
