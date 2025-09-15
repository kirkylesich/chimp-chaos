#![forbid(unsafe_code)]
#![deny(warnings)]
#![warn(clippy::pedantic)]

use actix_web::{App as ActixApp, HttpServer};
use operator::{http, build_app};
// use std::net::SocketAddr;
use tracing::info;
// removed duplicate build_app import
use operator::controller;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();
    let bind_str = config_layered::SETTINGS
        .http_bind
        .as_deref()
        .unwrap_or("0.0.0.0:8080");
    // Инициализируем DI и запускаем контроллер (пока заглушка)
    let app = build_app();
    let controller_app = app.clone();
    tokio::spawn(async move {
        let _ = controller::run(controller_app).await;
    });
    info!(bind = %bind_str, "starting http server");
    let http_app = app.clone();
    HttpServer::new(move || {
        let data_app = http_app.clone();
        ActixApp::new()
            .app_data(actix_web::web::Data::new(data_app))
            .app_data(actix_web::web::Data::new(operator::state::SharedState::new()))
            .configure(http::configure)
    })
    .bind(bind_str)?
    .run()
    .await?;
    Ok(())
}

fn init_tracing() {
    let fmt = tracing_subscriber::fmt().with_env_filter(tracing_subscriber::EnvFilter::from_default_env());
    fmt.json().init();
}

#[cfg(test)]
mod tests {
    #[test]
    fn smoke() {
        assert!(true);
    }
}

