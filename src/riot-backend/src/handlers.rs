use std::fs;

use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

use crate::models::RegisterForm;

// ROUTES

// users
#[post("/users/register")]
async fn user_register(form: web::Form<RegisterForm>) -> impl Responder {
    // TODO: insert to DB
    HttpResponse::Ok().body(format!("{:?}", form))
}

#[post("/users/login")]
async fn user_login() -> impl Responder {
    HttpResponse::Ok().body("Login!")
}

#[get("/devices")]
async fn all_devices() -> impl Responder {
    HttpResponse::Ok().body("ALL DEVICE!")
}

// devices
#[utoipa::path(
        get,
        context_path = "/api",
        path = "/devices/{did}",
        responses(
            (status = 200, description = "Device info", body = Device),
            (status = NOT_FOUND, description = "Device was not found")
        ),
        params()
    )]
#[get("/devices/{did}")]
async fn device_info(path: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body(format!("*get did={}!", path.into_inner()))
}

#[delete("/devices/{did}")]
async fn del_device(path: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body(format!("-del did={}!", path.into_inner()))
}

#[put("/devices/{did}")]
async fn upd_device_info(path: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body(format!("+-upd did={}!", path.into_inner()))
}

#[get("/devices/{did}/records")]
async fn device_records(path: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body(format!("get rec@did={}!", path.into_inner()))
}

#[post("/devices/{did}/records")]
async fn upd_device_records(path: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body(format!("+rec did={}!", path.into_inner()))
}

// sites
#[get("/sites")]
async fn all_sites() -> impl Responder {
    HttpResponse::Ok().body("ALL SITES!")
}

#[get("/sites/{sid}")]
async fn site_info(path: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body(format!("*get sid={}!", path.into_inner()))
}

#[delete("/sites/{sid}")]
async fn del_site(path: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body(format!("-del sid={}!", path.into_inner()))
}

#[put("/sites/{sid}")]
async fn upd_site_info(path: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body(format!("+-upd sid={}!", path.into_inner()))
}

#[get("/sites/{sid}/devices")]
async fn site_devices(path: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body(format!("get site@sid={}!", path.into_inner()))
}

#[post("/sites/{sid}/devices")]
async fn upd_site_devices(path: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body(format!("+site sid={}!", path.into_inner()))
}

// Pipes

#[get("/ws")]
async fn ws_socket() -> impl Responder {
    HttpResponse::Ok().body("WS!")
}

#[put("/mqtt/{did}")]
async fn mqtt_sub(path: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body(format!("sub mqtt did={}!", path.into_inner()))
}

#[delete("/mqtt/{did}")]
async fn mqtt_unsub(path: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body(format!("unsub mqtt did={}!", path.into_inner()))
}

// HTTP Codes
pub async fn notfound_404() -> HttpResponse {
    let content =
        fs::read_to_string("./public/404.html").unwrap_or_else(|_| "Page not found".to_string());
    HttpResponse::NotFound().body(content)
}
