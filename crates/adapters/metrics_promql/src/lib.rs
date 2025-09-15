#![forbid(unsafe_code)]
#![deny(warnings)]
#![warn(clippy::pedantic)]

use async_trait::async_trait;
use domain::errors::DomainError;
use domain::ports::MetricsBackend;

#[derive(Default, Clone)]
pub struct PromQl;

#[async_trait]
impl MetricsBackend for PromQl {
    async fn fetch_value(&self, _query: &str, _window: &str) -> Result<f64, DomainError> {
        Ok(1.0)
    }
}

