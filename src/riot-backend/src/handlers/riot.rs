use std::fs;

use actix_web::get;
use actix_web::HttpResponse;
use actix_web::Responder;

// ROUTES
// health checker
#[utoipa::path(
    get,
    context_path = "/api",
    tag = "RIoT",
    path = "/healthchecker",
    responses(
        (status = 200, description = "Always 'Ok'", body = String),
    ),
    params(),
)]
#[get("/healthchecker")]
pub(crate) async fn healthchecker() -> impl Responder {
    HttpResponse::Ok().content_type("text/plain").body("Ok")
}

// HTTP Codes
pub async fn notfound_404() -> HttpResponse {
    let content =
        fs::read_to_string("./public/404.html").unwrap_or_else(|_| "Page not found".to_string());
    HttpResponse::NotFound().body(content)
}
