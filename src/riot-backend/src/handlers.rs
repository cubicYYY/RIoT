use std::fs;

use actix_web::{
    delete, get, post, put,
    web::{self},
    HttpResponse, Responder, ResponseError,
};
use chrono::Utc;
use diesel::result::{DatabaseErrorKind, Error as DieselErr};
use log::{debug, error, info, Record};
use validator::Validate;

use crate::{
    errors::{ErrorMessage, HttpError},
    middlewares::{AuthenticatedUser, RequireAuth},
    models::{LoginForm, RecordFormDb, RecordFormWeb, RegisterForm, Response, User, UserPrivilege},
    schema::record::timestamp,
    utils::password::verify,
    AppState,
};

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
async fn healthchecker() -> impl Responder {
    HttpResponse::Ok().content_type("text/plain").body("Ok")
}

// account reg/login
#[utoipa::path(
    post,
    context_path = "/api",
    path = "/accounts/register",
    tag = "User",
    request_body(
        content = RegisterForm,
        description = "Register form", 
        example = json!({"username": "yyysama", "email": "egPerson@example.com","password": "pass.!w0rd"})
    ),
    responses(
        (status = 200, description = "Success", body = Response),
        (status = 400, description = "Bad input", body = Response),
        (status = 409, description = "Duplicate user / conflicted identity / user exists", body = Response),
        (status = 500, description = "Internal error, contact website admin", body = Response)
    ),
    params(),
    security(
        ("jwt_header" = []),
        ("jwt_cookie" = [])
    )
)]
#[post("/accounts/register")]
async fn user_register(form: web::Form<RegisterForm>, app: web::Data<AppState>) -> impl Responder {
    if let Err(e) = form.0.validate() {
        info!("Illegal input detected: {:?}", e);
        return HttpError::new(e.to_string(), 400).error_response();
    }

    let RegisterForm {
        username: username_,
        email: email_,
        password: password_,
    } = &*form;

    match app
        .register_user(
            &username_,
            &email_,
            &password_,
            UserPrivilege::Normal as u32,
        )
        .await
    {
        Ok(_) => HttpResponse::Ok().json(Response {
            status: "ok",
            message: "".into(),
        }),
        Err(DieselErr::DatabaseError(DatabaseErrorKind::UniqueViolation, _msg)) => {
            HttpError::new(ErrorMessage::UserExist, 409).error_response()
        }
        Err(e) => {
            error!("{:?}", e);
            HttpError::new(ErrorMessage::ServerError, 500).error_response()
        }
    }
}

#[utoipa::path(
    post,
    context_path = "/api",
    path = "/accounts/login",
    tag = "User",
    request_body(
        content = LoginForm,
        description = "Login form", 
        example = json!({"username": "yyysama", "password": "pass.!w0rd"})
    ),
    responses(
        (status = 200, description = "Success and return user id in message, set the token Cookie", body = Response),
        (status = 403, description = "Failed: wrong credentials or suspended/non-valid account ", body = Response),
        (status = 500, description = "Internal error, contact website admin", body = Response)
    ),
    params(),
    security(
        ("jwt_header" = []),
        ("jwt_cookie" = [])
    )
)]
#[post("/accounts/login")]
async fn user_login(form: web::Form<LoginForm>, app: web::Data<AppState>) -> impl Responder {
    let LoginForm {
        account: account_,
        password: password_,
    } = form.0;

    // ! NOTE: We MUST perform this hash comparison using a special function provided in the library, otherwise
    // ! it can be vulnerable to time-based attacks.

    match app.get_user_by_username_or_email(&account_).await {
        Ok(user) => {
            debug!(
                "{:?}, provided={:?}--{:?}",
                user,
                account_.as_bytes(),
                password_.as_bytes()
            );
            if verify(&user.password, password_.as_bytes()) {
                let jwt_cookie = app.get_jwt_cookie(user.id);
                if user.activated {
                    HttpResponse::Ok().cookie(jwt_cookie).json(Response {
                        status: "ok",
                        message: user.id.to_string(),
                    })
                } else {
                    HttpError::permission_denied(ErrorMessage::UserNotActivated).error_response()
                }
            } else {
                HttpError::permission_denied(ErrorMessage::WrongCredentials).error_response()
            }
        }
        Err(DieselErr::NotFound) => {
            HttpError::permission_denied(ErrorMessage::WrongCredentials).error_response()
        }
        Err(e) => {
            error!("{:?}", e);
            HttpError::server_error(ErrorMessage::ServerError).error_response()
        }
    }
}

#[utoipa::path(
    get,
    context_path = "/api",
    path = "/whoami",
    tag = "User",
    responses(
        (status = 200, description = "User struct", body = User),
        (status = 202, description = "Not logged in", body = Response),
    ),
    params(),
    security(
        ("jwt_header" = []),
        ("jwt_cookie" = [])
    )
)]
#[get("/whoami", wrap = "RequireAuth::no_auth()")]
async fn whoami(cur_user: Option<AuthenticatedUser>) -> impl Responder {
    match cur_user {
        Some(user) => HttpResponse::Ok().json(&*user),
        None => HttpResponse::Accepted().json(Response {
            status: "ok",
            message: "You are not logged-in.".into(),
        }),
    }
}

// devices

#[utoipa::path(
    get,
    context_path = "/api",
    path = "/devices",
    responses(
        (status = 200, description = "Devices", body = Vec<Device>),
        (status = 403, description = "Permission denied", body = Response),
        (status = 500, description = "Internal error, contact website admin", body = Response)
    ),
    params(),
    security(
        ("jwt_header" = []),
        ("jwt_cookie" = [])
    )
)]
#[get(
    "/devices",
    wrap = "RequireAuth::with_priv_level(UserPrivilege::Normal as u32)"
)]
async fn owned_devices(cur_user: AuthenticatedUser, app: web::Data<AppState>) -> impl Responder {
    let devices = app.get_owned_devices(cur_user.id).await;
    match devices {
        Ok(devices) => HttpResponse::Ok().json(devices),
        Err(e) => {
            error!("{:?}", e);
            HttpError::server_error(ErrorMessage::ServerError).error_response()
        }
    }
}

#[utoipa::path(
        get,
        context_path = "/api",
        path = "/devices/{did}",
        responses(
            (status = 200, description = "Device info", body = Device),
            (status = NOT_FOUND, description = "Device was not found")
        ),
        security(
            ("jwt_header" = []),
            ("jwt_cookie" = [])
        )
    )]
#[get(
    "/devices/{did}",
    wrap = "RequireAuth::with_priv_level(UserPrivilege::Normal as u32)"
)]
async fn device_info(
    path: web::Path<u64>,
    app: web::Data<AppState>,
    cur_user: AuthenticatedUser,
) -> impl Responder {
    let did = path.into_inner();
    match app.get_device_by_id(did).await {
        Ok(device) => {
            if device.uid == cur_user.id {
                HttpResponse::Ok().json(device)
            } else {
                HttpError::not_found(ErrorMessage::DeviceNotFound).error_response()
            }
        }
        Err(DieselErr::NotFound) => {
            HttpError::not_found(ErrorMessage::DeviceNotFound).error_response()
        }
        Err(e) => {
            error!("{:?}", e);
            HttpError::server_error(ErrorMessage::ServerError).error_response()
        }
    }
}

#[utoipa::path(
    delete,
    context_path = "/api",
    path = "/devices/{did}",
    responses(
        (status = 200, description = "Delete success", body = Response),
        (status = 401, description = "Unauthorized", body = Response),
        (status = 404, description = "Device was not found or the device is not yours \
        and you do not have enough privilege to delete it", body = Response),
        (status = 500, description = "Internal error, contact website admin", body = Response)
    ),
    security(
        ("jwt_header" = []),
        ("jwt_cookie" = [])
    )
)]
#[delete(
    "/devices/{did}",
    wrap = "RequireAuth::with_priv_level(UserPrivilege::Normal as u32)"
)]
async fn del_device(
    path: web::Path<u64>,
    app: web::Data<AppState>,
    cur_user: AuthenticatedUser,
) -> impl Responder {
    let did = path.into_inner();
    match app
        .update_device_status(did, false, Some(cur_user.id))
        .await
    {
        Ok(1) => HttpResponse::Ok().json(Response {
            status: "ok",
            message: "".into(),
        }),
        Ok(_) => HttpError::not_found(ErrorMessage::DeviceNotFound).error_response(),
        Err(e) => {
            error!("{:?}", e);
            HttpError::server_error(ErrorMessage::ServerError).error_response()
        }
    }
}

#[put(
    "/devices/{did}",
    wrap = "RequireAuth::with_priv_level(UserPrivilege::Normal as u32)"
)]
async fn upd_device_info(
    path: web::Path<u64>,
    app: web::Data<AppState>,
    cur_user: AuthenticatedUser,
) -> impl Responder {
    unimplemented!();
    HttpResponse::Ok().body(format!("+-upd did={}!", path.into_inner()))
}

#[utoipa::path(
    get,
    context_path = "/api",
    path = "/devices/{did}/records",
    responses(
        (status = 200, description = "Delete success", body = Vec<Record>),
        (status = 401, description = "Unauthorized", body = Response),
        (status = 404, description = "Device was not found or the device is not yours \
        and you do not have enough privilege to delete it", body = Response),
        (status = 500, description = "Internal error, contact website admin", body = Response)
    ),
    security(
        ("jwt_header" = []),
        ("jwt_cookie" = [])
    )
)]
#[get(
    "/devices/{did}/records",
    wrap = "RequireAuth::with_priv_level(UserPrivilege::Normal as u32)"
)]
async fn device_records(
    path: web::Path<u64>,
    app: web::Data<AppState>,
    cur_user: AuthenticatedUser,
) -> impl Responder {
    let did = path.into_inner();
    if Ok(true) == app.device_belongs_to(did, cur_user.id).await {
    } else {
        return HttpError::not_found(ErrorMessage::DeviceNotFound).error_response();
    }
    let records = app.get_device_records(did).await;
    match records {
        Ok(records) => HttpResponse::Ok().json(records),
        Err(e) => {
            error!("{:?}", e);
            HttpError::server_error(ErrorMessage::ServerError).error_response()
        }
    }
}

#[utoipa::path(
    post,
    context_path = "/api",
    path = "/devices/{did}/records",
    responses(
        (status = 200, description = "Insert record success", body = Response),
        (status = 401, description = "Unauthorized", body = Response),
        (status = 404, description = "Device was not found or the device is not yours \
        and you do not have enough privilege to delete it", body = Response),
        (status = 500, description = "Internal error, contact website admin", body = Response)
    ),
    security(
        ("jwt_header" = []),
        ("jwt_cookie" = [])
    )
)]
#[post(
    "/devices/{did}/records",
    wrap = "RequireAuth::with_priv_level(UserPrivilege::Normal as u32)"
)]
async fn insert_device_records(
    path: web::Path<u64>,
    app: web::Data<AppState>,
    form: web::Form<RecordFormWeb>,
    cur_user: AuthenticatedUser,
) -> impl Responder {
    let did = path.into_inner();
    if Ok(true) == app.device_belongs_to(did, cur_user.id).await {
    } else {
        return HttpError::not_found(ErrorMessage::DeviceNotFound).error_response();
    }
    let actix_web::web::Form(RecordFormWeb {
        payload: payload_,
        latitude: latitude_,
        longitude: longitude_,
        timestamp: timestamp_,
    }) = form;

    // If timestamp is not set, fill it with server time
    let timestamp_ = timestamp_.unwrap_or(Utc::now().naive_utc());

    let db_form = RecordFormDb {
        payload: payload_,
        latitude: latitude_,
        longitude: longitude_,
        timestamp: timestamp_,
    };

    match app.add_device_records(&db_form).await {
        Ok(1) => HttpResponse::Ok().json(Response {
            status: "ok",
            message: "".into(),
        }),
        Ok(_) => {
            error!("This should never happened!");
            HttpError::server_error(ErrorMessage::ServerError).error_response()
        }
        Err(e) => {
            error!("{:?}", e);
            HttpError::server_error(ErrorMessage::ServerError).error_response()
        }
    }
}

// sites
#[get("/sites")]
async fn owned_sites() -> impl Responder {
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
