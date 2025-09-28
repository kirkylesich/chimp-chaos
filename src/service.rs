#![forbid(unsafe_code)]
#![deny(warnings)]
#![warn(clippy::pedantic)]

use crate::models::{ChaosExperimentSpec, ChaosExperimentStatus, ExperimentRef, Phase};

pub async fn reconcile(_spec: &ChaosExperimentSpec, _exp: &ExperimentRef) -> ChaosExperimentStatus {
    ChaosExperimentStatus {
        phase: Phase::Running,
        started_at: Some(chrono::Utc::now().timestamp()),
        ends_at: None,
        baseline_window: None,
        failure_window: None,
        istio_patched: false,
        snapshot: None,
    }
}
