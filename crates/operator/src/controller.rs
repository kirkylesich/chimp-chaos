#![forbid(unsafe_code)]
#![deny(warnings)]
#![warn(clippy::pedantic)]

use crate::App;
use tracing::info;

pub async fn run(_app: App) -> anyhow::Result<()> {
    // Заготовка контроллера, реальный контроллер будет в дальнейшем
    info!("controller initialized");
    Ok(())
}

