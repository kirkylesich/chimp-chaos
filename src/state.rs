#![forbid(unsafe_code)]
#![deny(warnings)]
#![warn(clippy::pedantic)]

use crate::models::{ChaosExperimentStatus, ChaosReport};
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Default)]
pub struct SharedState {
    statuses: Mutex<HashMap<(String, String), ChaosExperimentStatus>>,
    reports: Mutex<HashMap<(String, String), ChaosReport>>,
}

impl SharedState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_status(&self, ns: &str, name: &str, status: ChaosExperimentStatus) {
        let mut g = self.statuses.lock().expect("status lock");
        g.insert((ns.to_string(), name.to_string()), status);
    }

    pub fn get_status(&self, ns: &str, name: &str) -> Option<ChaosExperimentStatus> {
        let g = self.statuses.lock().expect("status lock");
        g.get(&(ns.to_string(), name.to_string())).cloned()
    }

    pub fn set_report(&self, ns: &str, name: &str, report: ChaosReport) {
        let mut g = self.reports.lock().expect("report lock");
        g.insert((ns.to_string(), name.to_string()), report);
    }

    pub fn get_report(&self, ns: &str, name: &str) -> Option<ChaosReport> {
        let g = self.reports.lock().expect("report lock");
        g.get(&(ns.to_string(), name.to_string())).cloned()
    }
}
