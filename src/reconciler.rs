//! Reconciliation logic for ChaosExperiment

use crate::crd::{ChaosExperiment, Phase};
use kube::{
    api::{Api, Patch, PatchParams},
    Client, ResourceExt,
};
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;
use tracing::{error, info, warn};

#[derive(Debug, Error)]
pub enum ReconcileError {
    #[error("Kubernetes error: {0}")]
    KubeError(#[from] kube::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

pub struct Context {
    pub client: Client,
}

pub async fn reconcile(
    experiment: Arc<ChaosExperiment>,
    ctx: Arc<Context>,
) -> Result<Duration, ReconcileError> {
    let name = experiment.name_any();
    let namespace = experiment
        .namespace()
        .unwrap_or_else(|| "default".to_string());

    info!("Reconciling ChaosExperiment: {}/{}", namespace, name);

    let api: Api<ChaosExperiment> = Api::namespaced(ctx.client.clone(), &namespace);

    let current_phase = experiment
        .status
        .as_ref()
        .map(|s| s.phase.clone())
        .unwrap_or_default();

    info!(
        "Experiment: {}, Scenario: {:?}, Duration: {}s, Phase: {:?}",
        name, experiment.spec.scenario, experiment.spec.duration, current_phase
    );

    let new_phase = match current_phase {
        Phase::Pending => {
            info!("Starting experiment: {}", name);
            Phase::Running
        }
        Phase::Running => {
            info!("Experiment {} is running, marking as succeeded", name);
            Phase::Succeeded
        }
        Phase::Succeeded => {
            info!("Experiment {} already succeeded", name);
            return Ok(Duration::from_secs(300));
        }
        Phase::Failed => {
            warn!("Experiment {} is in failed state", name);
            return Ok(Duration::from_secs(300));
        }
    };

    let status = serde_json::json!({
        "status": {
            "phase": new_phase,
            "message": format!("Experiment is in {:?} phase", new_phase)
        }
    });

    let patch = Patch::Merge(&status);
    let ps = PatchParams::default();

    match api.patch_status(&name, &ps, &patch).await {
        Ok(_) => {
            info!("Status updated for {}: {:?}", name, new_phase);
        }
        Err(e) => {
            error!("Failed to update status for {}: {}", name, e);
            return Err(ReconcileError::KubeError(e));
        }
    }

    Ok(Duration::from_secs(10))
}

pub fn error_policy(
    _experiment: Arc<ChaosExperiment>,
    error: &ReconcileError,
) -> kube::runtime::controller::Action {
    error!("Reconcile error: {}", error);
    kube::runtime::controller::Action::requeue(Duration::from_secs(30))
}
