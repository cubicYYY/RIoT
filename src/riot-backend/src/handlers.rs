use std::fs;

use actix_web::{
    delete, get, post, put,
    web::{self},
    HttpResponse, Responder, ResponseError,
};
use diesel::result::{DatabaseErrorKind, Error as DieselErr};
use log::{debug, error, info};
use validator::Validate;

use crate::{
    errors::{ErrorMessage, HttpError},
    middlewares::{AuthenticatedUser, RequireAuth},
    models::{LoginForm, RegisterForm, Response, User, UserPrivilege},
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
    } = form.0;

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
                    HttpError::new(ErrorMessage::UserNotActivated, 403).error_response()
                }
            } else {
                HttpError::new(ErrorMessage::WrongCredentials, 403).error_response()
            }
        }
        Err(DieselErr::NotFound) => {
            HttpError::new(ErrorMessage::WrongCredentials, 403).error_response()
        }
        Err(e) => {
            error!("{:?}", e);
            HttpError::new(ErrorMessage::ServerError, 500).error_response()
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
        (status = 401, description = "Unauthorized: no valid token provided", body = Response),
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
async fn all_devices(cur_user: AuthenticatedUser) -> impl Responder {
    // cur_user is extracted by RequireAuth middleware
    HttpResponse::Ok().body(format!("{:?}", *cur_user))
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
                HttpError::new(ErrorMessage::PermissionDenied, 403).error_response()
            }
        }
        Err(DieselErr::NotFound) => HttpError::not_found("Device not exists").error_response(),
        Err(e) => {
            error!("{:?}", e);
            HttpError::new(ErrorMessage::ServerError, 500).error_response()
        }
    }
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
