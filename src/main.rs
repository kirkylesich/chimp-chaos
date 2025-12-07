mod crd;
mod reconciler;

use crd::ChaosExperiment;
use futures::StreamExt;
use kube::{
    api::Api,
    runtime::{controller::Action, watcher::Config, Controller},
    Client,
};
use reconciler::{error_policy, reconcile, Context};
use std::sync::Arc;
use tracing::{error, info};

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
        .run(
            |exp: Arc<ChaosExperiment>, ctx| async move {
                reconcile(exp, ctx)
                    .await
                    .map(Action::requeue)
                    .map_err(Box::new)
            },
            |exp, err, _ctx| error_policy(exp, err.as_ref()),
            context,
        )
        .for_each(|res| async move {
            match res {
                Ok(o) => info!("Reconciled: {:?}", o),
                Err(e) => error!("Reconcile failed: {}", e),
            }
        })
        .await;

    Ok(())
}
