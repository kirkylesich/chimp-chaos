#![forbid(unsafe_code)]
#![deny(warnings)]
#![warn(clippy::pedantic)]

use async_trait::async_trait;
use domain::errors::DomainError;
use domain::ports::MeshAdapter;

#[derive(Default, Clone)]
pub struct IstioMesh;

#[async_trait]
impl MeshAdapter for IstioMesh {
    async fn apply_fault(
        &self,
        _spec: &domain::models::ChaosExperimentSpec,
    ) -> Result<(), DomainError> {
        Ok(())
    }
    async fn rollback_fault(
        &self,
        _spec: &domain::models::ChaosExperimentSpec,
    ) -> Result<(), DomainError> {
        Ok(())
    }
}
