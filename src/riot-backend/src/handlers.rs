use std::fs;

use actix_web::{
    delete, get, post, put,
    web::{self},
    HttpRequest, HttpResponse, Responder, ResponseError,
};
use chrono::NaiveDateTime;
use diesel::{
    insert_into,
    result::{DatabaseErrorKind, Error},
    ExpressionMethods,
};
use diesel_async::RunQueryDsl;
use log::error;

use crate::{
    errors::{ErrorMessage, HttpError},
    models::{RegisterForm, Response, UserPrivilege},
    schema::user::{self, dsl::*},
    AppState,
};

// ROUTES

// users
#[post("/users/register")]
async fn user_register(form: web::Form<RegisterForm>, req: HttpRequest) -> impl Responder {
    let app = req
        .app_data::<web::Data<AppState>>()
        .expect("Internal error: invalid app state");
    let mut conn = app.db.pool.get().await.unwrap();
    let RegisterForm {
        username: username_,
        email: email_,
        password: password_,
    } = form.0;

    match insert_into(user::table)
        .values((
            username.eq(username_),
            email.eq(email_),
            password.eq(password_), // TODO: password hash
            privilege.eq(UserPrivilege::Normal as u32),
            since.eq(NaiveDateTime::MIN), // TODO: time record
        ))
        .execute(&mut conn)
        .await
    {
        Ok(_) => HttpResponse::Ok().json(Response {
            status: "ok",
            message: "".into(),
        }),
        Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _msg)) => {
            HttpError::new(ErrorMessage::UserExist, 409).error_response()
        }
        Err(e) => {
            error!("{:?}", e);
            HttpError::new(ErrorMessage::ServerError, 500).error_response()
        }
    }
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
