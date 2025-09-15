#![forbid(unsafe_code)]
#![deny(warnings)]
#![warn(clippy::pedantic)]

use async_trait::async_trait;
use domain::errors::DomainError;
use domain::models::Edge;
use domain::ports::TraceBackend;

#[derive(Default, Clone)]
pub struct TempoTrace;

#[async_trait]
impl TraceBackend for TempoTrace {
    async fn fetch_edges(&self, _window: &str) -> Result<Vec<Edge>, DomainError> {
        // Если переменная окружения TEMPO_BASE отсутствует — вернуть пустой список
        if std::env::var("TEMPO_BASE").ok().is_none() { return Ok(Vec::new()); }
        Ok(Vec::new())
    }
}

