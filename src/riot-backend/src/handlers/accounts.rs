use std::ops::Deref;

use actix_web::{get, post, put, web, HttpResponse, Responder, ResponseError};
use diesel::result::{DatabaseErrorKind, Error as DieselErr};
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;
use validator::{Validate, ValidationError};

use crate::{
    errors::{ErrorMessage, HttpError},
    middlewares::{AuthenticatedUser, RequireAuth},
    models::{NewUser, Response, UpdateUser, UserPrivilege},
    utils::password::{get_pwd_hash, verify},
    AppState,
};

#[derive(Deserialize, IntoParams)]
/// Param in path, user account(email/username) to verify
struct VerifyEmail {
    account: String,
}

#[derive(Deserialize, IntoParams)]
/// Param in path, one-time verification code
struct OneTimeCode {
    code: String,
}

fn validate_username(password: &str) -> Result<(), ValidationError> {
    let is_valid_username = password.chars().all(|c| c.is_alphanumeric());

    if !is_valid_username {
        let mut err = ValidationError::new("Invalid password");
        err.message =
            Option::Some("Username can only contain number or letters. e.g.: AaBb01".into());
        return Err(err);
    }

    Ok(())
}

fn validate_pwd(password: &str) -> Result<(), ValidationError> {
    let is_valid_pwd = {
        let has_uppercase = password.chars().any(char::is_uppercase);
        let has_lowercase = password.chars().any(char::is_lowercase);
        let has_digit = password.chars().any(char::is_numeric);
        let has_special_char = password.chars().any(|c| !c.is_alphanumeric());
        (has_uppercase || has_lowercase) && has_digit && has_special_char
    };

    if !is_valid_pwd {
        let mut err = ValidationError::new("Invalid password");
        err.message = Option::Some(
            "Your password is too weak: should contain both number, letter and symbols.".into(),
        );
        return Err(err);
    }

    Ok(())
}

#[derive(Validate, Serialize, Deserialize, ToSchema, Clone, Debug)]
/// Web json form to register a new user
pub struct RegisterForm {
    #[validate(
        length(min = 6, max = 64, message = "Username must be 6-64 characters"),
        custom = "validate_username"
    )]
    pub username: Option<String>,
    #[validate(email)]
    pub email: String,
    #[validate(
        length(min = 8, max = 64, message = "Password must be 8-64 characters"),
        custom = "validate_pwd"
    )]
    pub password: String,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
/// Web json form to login
pub struct LoginForm {
    #[serde(alias = "username")]
    #[serde(alias = "email")]
    /// aliases: `username` `email`
    pub account: String,
    pub password: String,
}

#[derive(Validate, Serialize, Deserialize, ToSchema, Clone, Debug)]
/// Web json form to update a user
pub struct UpdateUserForm {
    #[validate(
        length(min = 6, max = 64, message = "Username must be 6-64 characters"),
        custom = "validate_username"
    )]
    pub username: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    #[validate(
        length(min = 8, max = 64, message = "Password must be 8-64 characters"),
        custom = "validate_pwd"
    )]
    pub password: Option<String>,
}

// account reg/login
#[utoipa::path(
        post,
        context_path = "/api",
        path = "/accounts/register",
        tag = "Account",
        request_body(
            content = RegisterForm,
            description = "Register form", 
            example = json!({"username": "yyysama", "email": "egPerson@example.com","password": "pass.!w0rd"})
        ),
        responses(
            (status = 200, description = "Success", body = Response),
            (status = 400, description = "Bad input", body = Response),
            (status = 409, description = "Duplicate user / conflicted identity / user exists", body = Response),
            (status = 500, description = "Internal error, contact webtag admin", body = Response)
        ),
        params(),
        security(
            ("jwt_header" = []),
            ("jwt_cookie" = [])
        )
    )]
#[post("/accounts/register")]
/// Register a new user
/// WARNING: The password is transfered as a plain text, HTTPS is a must here
pub(crate) async fn user_register(
    form: web::Json<RegisterForm>,
    app: web::Data<AppState>,
) -> impl Responder {
    if let Err(e) = form.deref().validate() {
        info!("Illegal input detected: {:?}", e);
        return HttpError::new(e.to_string(), 400).error_response();
    }

    let RegisterForm {
        username,
        email,
        password,
    } = form.into_inner();
    let api_key = Uuid::new_v4().to_string();
    let user = NewUser {
        username: &username.unwrap_or_else(|| email.clone()), // Better performance when using lazy calc!
        email: &email,
        hashed_password: &get_pwd_hash(app.env.riot.password_salt.as_bytes(), password.as_bytes()),
        privilege: UserPrivilege::Normal as u32,
        api_key: Some(&api_key),
    };

    match app.db.register_user(&user).await {
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
        tag = "Account",
        request_body(
            content = LoginForm,
            description = "Login form", 
            example = json!({"username": "yyysama", "password": "pass.!w0rd"})
        ),
        responses(
            (status = 200, description = "Success and return user token in message, set the token Cookie", body = Response),
            (status = 403, description = "Failed: wrong credentials or suspended/non-valid account ", body = Response),
            (status = 500, description = "Internal error, contact webtag admin", body = Response)
        ),
        params(),
        security(
            ("jwt_header" = []),
            ("jwt_cookie" = [])
        )
    )]
#[post("/accounts/login")]
/// Login with a password
/// To login with an email, use `send_verification` endpoint instead
pub(crate) async fn user_login(
    form: web::Json<LoginForm>,
    app: web::Data<AppState>,
) -> impl Responder {
    let LoginForm { account, password } = form.into_inner();

    // ! NOTE: We MUST perform this hash comparison using a special function provided in the library, otherwise
    // ! it can be vulnerable to time-based attacks.

    match app.db.get_user_by_username_or_email(&account).await {
        Ok(user) => {
            debug!(
                "{:?}, provided={:?}--{:?}",
                user,
                account.as_bytes(),
                password.as_bytes()
            );
            if verify(&user.password, password.as_bytes()) {
                let jwt_cookie = app.get_jwt_cookie(user.id);
                if user.activated {
                    HttpResponse::Ok()
                        .cookie(jwt_cookie.clone())
                        .json(Response {
                            status: "ok",
                            message: jwt_cookie.value().to_string(),
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
        path = "/accounts/user_info",
        tag = "Account",
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
#[get("/accounts/user_info", wrap = "RequireAuth::no_auth()")]
/// Current user info
pub(crate) async fn user_info(cur_user: Option<AuthenticatedUser>) -> impl Responder {
    match cur_user {
        Some(user) => {
            let mut user = (*user).clone();
            user.password = "".into();
            user.api_key = None;
            HttpResponse::Ok().json(user)
        }
        None => HttpResponse::Accepted().json(Response {
            status: "ok",
            message: "You are not logged-in.".into(),
        }),
    }
}

#[utoipa::path(
    put,
    context_path = "/api",
    path = "/accounts/user_info",
    tag = "Account",
    request_body(
        content = UpdateUserForm,
        description = "Update user", 
        example = json!(
            {
                "username": "new_name",
                "email": "new_email@example.com",
                "password": "raw!pass.word!",
            })
    ),
    responses(
        (status = 200, description = "Ok", body = Response),
        (status = 304, description = "No change to be done", body = Response),
        (status = 401, description = "Not logged in", body = Response),
        (status = 500, description = "Server internal error", body = Response),
    ),
    security(
        ("jwt_header" = []),
        ("jwt_cookie" = [])
    )
)]
#[put(
    "/accounts/user_info",
    wrap = "RequireAuth::with_priv_level(UserPrivilege::Normal as u32)"
)]
/// Update user personal info(email/username/password)
pub(crate) async fn upd_user_info(
    app: web::Data<AppState>,
    cur_user: AuthenticatedUser,
    form: web::Json<UpdateUserForm>,
) -> impl Responder {
    if let Err(e) = form.deref().validate() {
        info!("Illegal input detected: {:?}", e);
        return HttpError::new(e.to_string(), 400).error_response();
    }

    let UpdateUserForm {
        username,
        email,
        password,
    } = form.into_inner();
    match app
        .db
        .update_user(&UpdateUser {
            id: cur_user.id,
            username: username.as_deref(),
            email: email.as_deref(),
            hashed_password: password
                .map(|pwd| get_pwd_hash(app.env.riot.password_salt.as_bytes(), pwd.as_bytes()))
                .as_deref(),
            privilege: None,
            activated: None,
            api_key: None,
        })
        .await
    {
        Ok(_) => HttpResponse::Ok().json(Response {
            status: "ok",
            message: "".into(),
        }),
        Err(diesel::result::Error::QueryBuilderError(_)) => {
            HttpError::not_modified(ErrorMessage::NoChange).error_response()
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
    path = "/accounts/send_verification",
    tag = "Account",
    params(VerifyEmail),
    responses(
        (status = 200, description = "Ok", body = Response),
        (status = 304, description = "No email provided", body = Response),
        (status = 429, description = "Only 1 request is allowed in 60s", body = Response),
        (status = 500, description = "Server internal error", body = Response),
    )
)]
#[get("/accounts/send_verification")]
/// Send the verification to the user (specified by email/username) for activation / logging
/// ONLY actually send if the user exists.
pub(crate) async fn send_verification_email(
    app: web::Data<AppState>,
    query: web::Query<VerifyEmail>,
) -> impl Responder {
    let account = &query.account;
    // Check access frequency
    if app.rate_limit.get(account).await.is_some() {
        return HttpError::too_many_requests(ErrorMessage::TooFast).error_response();
    } else {
        app.rate_limit.insert(account.into(), ()).await;
    }
    match app.db.get_user_by_username_or_email(account).await {
        Ok(user) => {
            let code = Uuid::new_v4();
            app.one_time_code.insert(code.to_string(), user.id).await;
            let verify_link =
                app.env.riot.host.to_string() + &format!("/api/accounts/verify?code={code}");
            debug!("OTC link = {verify_link}");
            if let Err(e) = app.send_verify_mail(&user.email, &verify_link).await {
                error!("{}", e);
            }
            HttpResponse::Ok().json(Response {
                status: "ok",
                message: "If the user exists, the verification email has been sent.".into(),
            })
        }
        Err(e) => {
            error!("{:?}", e);
            // !Do not leak the info that the user not exists
            HttpResponse::Ok().json(Response {
                status: "ok",
                message: "If the user exists, the verification email has been sent.".into(),
            })
        }
    }
}

#[utoipa::path(
    get,
    context_path = "/api",
    path = "/accounts/verify",
    tag = "Account",
    params(OneTimeCode),
    responses(
        (status = 200, description = "Ok, the user is now activated and logged-in", body = Response),
        (status = 403, description = "Verification failed", body = Response),
        (status = 304, description = "No email provided", body = Response),
        (status = 500, description = "Server internal error", body = Response),
    )
)]
#[get("/accounts/verify")]
/// Verify the email address of the user, or login by the email
pub(crate) async fn verify_login_by_email(
    app: web::Data<AppState>,
    query: web::Query<OneTimeCode>,
) -> impl Responder {
    let code = &query.code;
    dbg!(&app.one_time_code);
    if let Some(uid) = &app.one_time_code.remove(code).await {
        // Activate the user
        app.db
            .update_user(&UpdateUser {
                id: *uid,
                username: None,
                email: None,
                hashed_password: None,
                privilege: None,
                activated: Some(true), // activate!
                api_key: None,
            })
            .await
            .expect("User Activation Failed!");
        let jwt_cookie = app.get_jwt_cookie(*uid);
        HttpResponse::Ok()
            .cookie(jwt_cookie.clone())
            .json(Response {
                status: "ok",
                message: jwt_cookie.value().to_string(),
            })
    } else {
        HttpError::permission_denied(ErrorMessage::InvalidToken).error_response()
    }
}
