use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use app_state::{app_name, req_counter};
use log::info;
use actix_web::{get, Responder};
use actix_web::web::Data;
use path::{multiple_paths, single_path};
use std::sync::Mutex;
use json::{json_test, json_time};

mod app_state;
mod json;
mod path;
pub(crate) struct AppState {
    app_name: String,
    req_counter: Mutex<u32>
}


#[get("/")]
async fn hello() -> impl Responder {
    "Hello!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    

    let app_state = Data::new(AppState {
        app_name: "test".to_string(),
        req_counter: Mutex::new(0)
    });

    info!("Succesfully started server");

    HttpServer::new(move || {
        App::new()
        .wrap(Logger::default())
        .app_data(app_state.clone())
        .service(hello)
        .service(app_name)
        .service(req_counter)
        .service(json_test)
        .service(json_time)
        .service(single_path)
        .service(multiple_paths)
    }).bind("0.0.0.0:8080")
    .unwrap().run().await


    
}
