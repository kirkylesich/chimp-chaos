#![forbid(unsafe_code)]
#![deny(warnings)]
#![warn(clippy::pedantic)]

use async_trait::async_trait;
use domain::errors::DomainError;
use domain::ports::ReportSink;

#[derive(Default, Clone)]
pub struct FsSink;

#[async_trait]
impl ReportSink for FsSink {
    async fn store_report(&self, report: &domain::models::ChaosReport) -> Result<(), DomainError> {
        let _ = serde_json::to_string(report).map_err(|e| DomainError::message(&e.to_string()))?;
        Ok(())
    }
}
