use actix_web::{HttpResponse, post};
use actix_web::web::Json;
use actix_web::get;
use serde::Deserialize;
use log::debug;


#[derive(Deserialize, Debug)]
struct Test {
    field1: String,
    field2: u32
}


// #[post("/register")]
// pub(crate) async fn json_test() -> HttpResponse {
//     HttpResponse::Ok().finish()
// }

#[post("/json/test")]
pub(crate) async fn json_test(json: Json<Test>) -> HttpResponse {
   debug!("{:?}", json);
    HttpResponse::Ok().finish()
}

#[get("/json/time")]
pub(crate) async fn json_time() -> HttpResponse {
    let current_utc = chrono::Utc::now();
    HttpResponse::Ok().json(current_utc)
}

