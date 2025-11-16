//! ChaosExperiment CRD definition

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Chaos scenario type
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum ScenarioType {
    PodKiller,
    CpuStress,
    NetworkDelay,
}

/// Experiment specification
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "chaos.io",
    version = "v1",
    kind = "ChaosExperiment",
    namespaced,
    status = "ChaosExperimentStatus"
)]
pub struct ChaosExperimentSpec {
    /// Scenario type
    pub scenario: ScenarioType,
    
    /// Duration in seconds
    pub duration: u64,
    
    /// Target namespace (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_namespace: Option<String>,
}

/// Experiment phase
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Default)]
pub enum Phase {
    #[default]
    Pending,
    Running,
    Succeeded,
    Failed,
}

/// Experiment status
#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema, Default)]
pub struct ChaosExperimentStatus {
    #[serde(default)]
    pub phase: Phase,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
