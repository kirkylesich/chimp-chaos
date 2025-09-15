#![forbid(unsafe_code)]
#![deny(warnings)]
#![warn(clippy::pedantic)]

use actix_web::{body::to_bytes, test, App as ActixApp};
use operator::{build_app, http, state::SharedState};

#[actix_web::test]
async fn healthz_ok() {
    let app = test::init_service(
        ActixApp::new()
            .app_data(actix_web::web::Data::new(build_app()))
            .app_data(actix_web::web::Data::new(SharedState::new()))
            .configure(http::configure),
    )
    .await;
    let req = test::TestRequest::get().uri("/healthz").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn report_not_found_then_create() {
    let app = test::init_service(
        ActixApp::new()
            .app_data(actix_web::web::Data::new(build_app()))
            .app_data(actix_web::web::Data::new(SharedState::new()))
            .configure(http::configure),
    )
    .await;

    // Not found before create
    let req = test::TestRequest::get().uri("/experiments/ns/exp/report").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);

    // Create report
    let req = test::TestRequest::get().uri("/experiments/ns/exp/create-report").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Now exists
    let req = test::TestRequest::get().uri("/experiments/ns/exp/report").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let body = to_bytes(resp.into_body()).await.unwrap();
    assert!(std::str::from_utf8(&body).unwrap().contains("\"experiment_ref\""));
}

#[actix_web::test]
async fn apply_and_status_running() {
    let app = test::init_service(
        ActixApp::new()
            .app_data(actix_web::web::Data::new(build_app()))
            .app_data(actix_web::web::Data::new(SharedState::new()))
            .configure(http::configure),
    )
    .await;

    let req = test::TestRequest::get().uri("/experiments/ns/exp/apply").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let req = test::TestRequest::get().uri("/experiments/ns/exp/status").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let body = to_bytes(resp.into_body()).await.unwrap();
    let s = std::str::from_utf8(&body).unwrap();
    assert!(s.contains("\"phase\":\"Running\""));
}

#[actix_web::test]
async fn run_sets_status() {
    let app = test::init_service(
        ActixApp::new()
            .app_data(actix_web::web::Data::new(build_app()))
            .app_data(actix_web::web::Data::new(SharedState::new()))
            .configure(http::configure),
    )
    .await;

    let req = test::TestRequest::post().uri("/experiments/ns/exp/run").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let req = test::TestRequest::get().uri("/experiments/ns/exp/status").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let body = to_bytes(resp.into_body()).await.unwrap();
    let s = std::str::from_utf8(&body).unwrap();
    assert!(s.contains("\"phase\":"));
}


