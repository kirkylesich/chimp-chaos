#![forbid(unsafe_code)]
#![deny(warnings)]
#![warn(clippy::pedantic)]

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct TargetSpec {
    pub service: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#match: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ModeSpec {
    Mesh,
    Agent,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct LoadProfileSpec {
    pub r#type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rps: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connections: Option<u32>,
}

#[derive(CustomResource, Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[kube(
    group = "chaos.chimp.io",
    version = "v1alpha1",
    kind = "ChaosExperiment",
    plural = "chaosexperiments",
    namespaced,
    status = "ChaosExperimentStatus",
    printcolumn = r#"{"name":"Phase","type":"string","description":"Current phase","jsonPath":".status.phase"}"#
)]
pub struct ChaosExperimentSpec {
    pub targets: Vec<TargetSpec>,
    pub mode: ModeSpec,
    pub duration_seconds: u32,
    pub load_profile: LoadProfileSpec,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mesh_fault: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_fault: Option<String>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub labels: HashMap<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PhaseStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, Default)]
pub struct ChaosExperimentStatus {
    pub phase: Option<PhaseStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ends_at: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub baseline_window: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure_window: Option<String>,
    #[serde(default)]
    pub istio_patched: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<String>,
}
