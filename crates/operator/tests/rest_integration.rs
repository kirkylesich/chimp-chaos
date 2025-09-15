#![forbid(unsafe_code)]
#![deny(warnings)]
#![warn(clippy::pedantic)]

use actix_web::{test, App};
use operator::http;

#[actix_web::test]
async fn healthz_works() {
    let app = test::init_service(App::new().configure(http::configure)).await;
    let req = test::TestRequest::get().uri("/healthz").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

