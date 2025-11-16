mod crd;

use crd::{ChaosExperiment, Phase};
use futures::StreamExt;
use kube::{
    api::{Api, Patch, PatchParams},
    runtime::{controller::Action, watcher::Config, Controller},
    Client, ResourceExt,
};
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;
use tracing::{error, info, warn};

#[derive(Debug, Error)]
enum ReconcileError {
    #[error("Kubernetes error: {0}")]
    KubeError(#[from] kube::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

/// Reconcile context
struct Context {
    client: Client,
}

/// Main reconcile logic
async fn reconcile(
    experiment: Arc<ChaosExperiment>,
    ctx: Arc<Context>,
) -> Result<Action, ReconcileError> {
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
            return Ok(Action::requeue(Duration::from_secs(300)));
        }
        Phase::Failed => {
            warn!("Experiment {} is in failed state", name);
            return Ok(Action::requeue(Duration::from_secs(300)));
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

    Ok(Action::requeue(Duration::from_secs(10)))
}

/// Error policy handler
fn error_policy(
    _experiment: Arc<ChaosExperiment>,
    error: &ReconcileError,
    _ctx: Arc<Context>,
) -> Action {
    error!("Reconcile error: {}", error);
    Action::requeue(Duration::from_secs(30))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    info!("Starting Chaos Operator");

    let client = Client::try_default().await?;
    info!("Connected to Kubernetes cluster");

    let context = Arc::new(Context {
        client: client.clone(),
    });

    let experiments: Api<ChaosExperiment> = Api::all(client);

    info!("Watching for ChaosExperiment resources");

    Controller::new(experiments, Config::default())
        .run(reconcile, error_policy, context)
        .for_each(|res| async move {
            match res {
                Ok(o) => info!("Reconciled: {:?}", o),
                Err(e) => error!("Reconcile failed: {}", e),
            }
        })
        .await;

    Ok(())
}
