use actix_web::{get, HttpResponse, web};

#[get("/{path}")]
pub(crate) async fn single_path(path: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().body(format!("You looked for {}", path))
}

#[get("/{path1}/{path2}")]
pub(crate) async fn multiple_paths(path: web::Path<(String, String)>) -> HttpResponse {
    let (path1, path2) = path.into_inner();

    HttpResponse::Ok().body(format!("You looked for {}/{}", path1, path2))
}