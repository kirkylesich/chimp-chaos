#![forbid(unsafe_code)]
#![deny(warnings)]
#![warn(clippy::pedantic)]

use async_trait::async_trait;
use domain::errors::DomainError;
use domain::ports::AgentClient;

#[derive(Default, Clone)]
pub struct GrpcAgents;

#[async_trait]
impl AgentClient for GrpcAgents {
    async fn start_load(&self, _spec: &domain::models::ChaosExperimentSpec) -> Result<(), DomainError> { Ok(()) }
    async fn stop_load(&self, _spec: &domain::models::ChaosExperimentSpec) -> Result<(), DomainError> { Ok(()) }
}

