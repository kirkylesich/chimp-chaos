#![forbid(unsafe_code)]
#![deny(warnings)]
#![warn(clippy::pedantic)]

use crate::models::{
    ChaosExperimentSpec, ChaosExperimentStatus, ExperimentRef, LoadProfile, Mode, Phase, Target,
};
use crate::service::reconcile;
use crate::state::SharedState;
use actix_web::{get, post, web, HttpResponse, Responder};

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(healthz)
        .service(status)
        .service(report)
        .service(stop)
        .service(apply_experiment)
        .service(create_report)
        .service(run_experiment);
}

#[get("/healthz")]
async fn healthz() -> impl Responder {
    HttpResponse::Ok().body("ok")
}

#[get("/experiments/{ns}/{name}/status")]
async fn status(
    path: web::Path<(String, String)>,
    state: web::Data<SharedState>,
) -> impl Responder {
    let (ns, name) = path.into_inner();
    let current = state
        .get_status(&ns, &name)
        .unwrap_or(ChaosExperimentStatus {
            phase: Phase::Pending,
            started_at: None,
            ends_at: None,
            baseline_window: None,
            failure_window: None,
            istio_patched: false,
            snapshot: None,
        });
    let body = serde_json::to_string(&current).unwrap_or_else(|_| "{}".to_string());
    HttpResponse::Ok()
        .content_type("application/json")
        .body(body)
}

#[get("/experiments/{ns}/{name}/report")]
async fn report(
    path: web::Path<(String, String)>,
    state: web::Data<SharedState>,
) -> impl Responder {
    let (ns, name) = path.into_inner();
    if let Some(rep) = state.get_report(&ns, &name) {
        let body = serde_json::to_string(&rep).unwrap_or_else(|_| "{}".to_string());
        HttpResponse::Ok()
            .content_type("application/json")
            .body(body)
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[get("/experiments/{ns}/{name}/stop")]
async fn stop(path: web::Path<(String, String)>, _state: web::Data<SharedState>) -> impl Responder {
    let _ = path.into_inner();
    HttpResponse::Ok()
        .content_type("application/json")
        .body("{\"stopped\":true}")
}

#[get("/experiments/{ns}/{name}/apply")]
async fn apply_experiment(
    path: web::Path<(String, String)>,
    state: web::Data<SharedState>,
) -> impl Responder {
    let (ns, name) = path.into_inner();
    state.set_status(
        &ns,
        &name,
        ChaosExperimentStatus {
            phase: Phase::Running,
            started_at: None,
            ends_at: None,
            baseline_window: None,
            failure_window: None,
            istio_patched: false,
            snapshot: None,
        },
    );
    HttpResponse::Ok().finish()
}

#[get("/experiments/{ns}/{name}/create-report")]
async fn create_report(
    path: web::Path<(String, String)>,
    state: web::Data<SharedState>,
) -> impl Responder {
    let (ns, name) = path.into_inner();
    let chaos_report = crate::models::ChaosReport {
        experiment_ref: crate::models::ExperimentRef {
            namespace: ns.clone(),
            name: name.clone(),
        },
        windows: crate::models::Windows {
            baseline: "b".into(),
            failure: "f".into(),
        },
        results: Vec::new(),
        topology: crate::models::Topology {
            nodes: Vec::new(),
            edges: Vec::new(),
        },
        artifacts: crate::models::Artifacts { json: "{}".into() },
    };
    state.set_report(&ns, &name, chaos_report);
    HttpResponse::Ok().finish()
}

#[post("/experiments/{ns}/{name}/run")]
async fn run_experiment(
    path: web::Path<(String, String)>,
    state: web::Data<SharedState>,
) -> impl Responder {
    let (ns, name) = path.into_inner();
    let exp_ref = ExperimentRef {
        namespace: ns.clone(),
        name: name.clone(),
    };
    let spec = ChaosExperimentSpec {
        targets: vec![Target {
            service: name.clone(),
            r#match: None,
        }],
        mode: Mode::Mesh,
        duration_seconds: 1,
        load_profile: LoadProfile {
            r#type: "none".into(),
            rps: None,
            connections: None,
        },
        mesh_fault: None,
        agent_fault: None,
        labels: std::collections::HashMap::new(),
    };
    let exp_status = reconcile(&spec, &exp_ref).await;
    state.set_status(&ns, &name, exp_status);
    HttpResponse::Ok().finish()
}
