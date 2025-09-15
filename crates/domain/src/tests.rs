use super::models::{ChaosExperimentSpec, LoadProfile, Mode, Target};
use std::collections::HashMap;

#[test]
fn create_minimal_spec() {
    let spec = ChaosExperimentSpec {
        targets: vec![Target {
            service: "svc".to_string(),
            r#match: None,
        }],
        mode: Mode::Mesh,
        duration_seconds: 1,
        load_profile: LoadProfile {
            r#type: "none".to_string(),
            rps: None,
            connections: None,
        },
        mesh_fault: None,
        agent_fault: None,
        labels: HashMap::new(),
    };
    assert_eq!(spec.targets.len(), 1);
}
