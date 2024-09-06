use actix_web::{get, Responder};
use actix_web::web::Data;
use crate::AppState;

#[get("/appname")]
pub(crate) async fn app_name(app_state: Data<AppState>) -> impl Responder {
    app_state.app_name.clone()
}

#[get("\req")]
pub(crate) async fn req_counter(app_state: Data<AppState>) -> impl Responder {
    let mut req_counter = app_state.req_counter.lock().unwrap();
    *req_counter += 1;
    format!("Requests sent: {}", req_counter)
}