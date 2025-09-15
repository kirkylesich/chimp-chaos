#![forbid(unsafe_code)]
#![deny(warnings)]
#![warn(clippy::pedantic)]

use actix_web::{get, post, web, HttpResponse, Responder};
use crate::state::SharedState;
use application::usecases::Reconciler;
use domain::models::{ExperimentRef, LoadProfile, Mode, Target};
use domain::models::{ChaosExperimentStatus, Phase};

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
async fn healthz() -> impl Responder { HttpResponse::Ok().body("ok") }

#[get("/experiments/{ns}/{name}/status")]
async fn status(path: web::Path<(String, String)>, state: web::Data<SharedState>) -> impl Responder {
    let (ns, name) = path.into_inner();
    let current = state.get_status(&ns, &name).unwrap_or(ChaosExperimentStatus {
        phase: Phase::Pending,
        started_at: None,
        ends_at: None,
        baseline_window: None,
        failure_window: None,
        istio_patched: false,
        snapshot: None,
    });
    let body = serde_json::to_string(&current).unwrap_or_else(|_| "{}".to_string());
    HttpResponse::Ok().content_type("application/json").body(body)
}

#[get("/experiments/{ns}/{name}/report")]
async fn report(path: web::Path<(String, String)>, state: web::Data<SharedState>) -> impl Responder {
    let (ns, name) = path.into_inner();
    if let Some(rep) = state.get_report(&ns, &name) {
        let body = serde_json::to_string(&rep).unwrap_or_else(|_| "{}".to_string());
        HttpResponse::Ok().content_type("application/json").body(body)
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[get("/experiments/{ns}/{name}/stop")]
async fn stop(path: web::Path<(String, String)>, _state: web::Data<SharedState>) -> impl Responder {
    let _ = path.into_inner();
    // В базовой заглушке просто возвращаем 200
    HttpResponse::Ok().content_type("application/json").body("{\"stopped\":true}")
}

#[get("/experiments/{ns}/{name}/apply")]
async fn apply_experiment(path: web::Path<(String, String)>, state: web::Data<SharedState>) -> impl Responder {
    let (ns, name) = path.into_inner();
    state.set_status(&ns, &name, ChaosExperimentStatus { phase: Phase::Running, started_at: None, ends_at: None, baseline_window: None, failure_window: None, istio_patched: false, snapshot: None });
    HttpResponse::Ok().finish()
}

#[get("/experiments/{ns}/{name}/create-report")]
async fn create_report(path: web::Path<(String, String)>, state: web::Data<SharedState>) -> impl Responder {
    let (ns, name) = path.into_inner();
    let chaos_report = domain::models::ChaosReport {
        experiment_ref: domain::models::ExperimentRef { namespace: ns.clone(), name: name.clone() },
        windows: domain::models::Windows { baseline: "b".into(), failure: "f".into() },
        results: Vec::new(),
        topology: domain::models::Topology { nodes: Vec::new(), edges: Vec::new() },
        artifacts: domain::models::Artifacts { json: "{}".into() },
    };
    state.set_report(&ns, &name, chaos_report);
    HttpResponse::Ok().finish()
}

#[post("/experiments/{ns}/{name}/run")]
async fn run_experiment(path: web::Path<(String, String)>, app: web::Data<crate::App>, state: web::Data<SharedState>) -> impl Responder {
    let (ns, name) = path.into_inner();
    let exp_ref = ExperimentRef { namespace: ns.clone(), name: name.clone() };
    let spec = domain::models::ChaosExperimentSpec {
        targets: vec![Target { service: name.clone(), r#match: None }],
        mode: Mode::Mesh,
        duration_seconds: 1,
        load_profile: LoadProfile { r#type: "none".into(), rps: None, connections: None },
        mesh_fault: None,
        agent_fault: None,
        labels: std::collections::HashMap::new(),
    };
    let mut exp_status = domain::models::ChaosExperimentStatus { phase: Phase::Pending, started_at: None, ends_at: None, baseline_window: None, failure_window: None, istio_patched: false, snapshot: None };
    let rec = Reconciler::new(app.mesh.as_ref(), app.agents.as_ref(), app.metrics.as_ref(), app.traces.as_ref(), app.sink.as_ref(), app.clock.as_ref());
    let _ = rec.reconcile(&spec, &mut exp_status, &exp_ref).await;
    state.set_status(&ns, &name, exp_status);
    HttpResponse::Ok().finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handlers_exist() {
        let _ = healthz;
        let _ = status;
        let _ = report;
        let _ = stop;
    }
}

