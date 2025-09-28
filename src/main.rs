#![forbid(unsafe_code)]
#![deny(warnings)]
#![warn(clippy::pedantic)]

mod crd;
mod http;
mod models;
mod service;
mod state;

use actix_web::{App, HttpServer};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();
    // touch types so they are referenced
    let _ = (
        std::any::TypeId::of::<crd::ChaosExperiment>(),
        std::any::TypeId::of::<crd::ChaosExperimentSpec>(),
    );
    let bind = std::env::var("HTTP_BIND").unwrap_or_else(|_| "0.0.0.0:8080".into());
    info!(bind=%bind, "starting http server");
    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(state::SharedState::new()))
            .configure(http::configure)
    })
    .bind(bind)?
    .run()
    .await?;
    Ok(())
}

fn init_tracing() {
    let fmt = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env());
    fmt.json().init();
}
