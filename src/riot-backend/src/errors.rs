use std::{error::Error, fmt};

use actix_web::{HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};

use crate::models::Response;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[derive(PartialEq)]
#[allow(unused)]
pub enum ErrorMessage {
    EmptyPassword,
    ExceededMaxPasswordLength(usize),
    InsufficientPasswordLength(usize),
    HashingError,
    InvalidHashFormat,
    InvalidUserInput,
    InvalidToken,
    InvalidUsername,
    InvalidPassword,
    InvalidEmail,
    NoChange,
    ServerError,
    WrongCredentials,
    EmailExist,
    UsernameExist,
    UserExist,
    TagExist,
    UserNotActivated,
    UpdateFailed,
    TokenNotProvided,
    PermissionDenied,
    TooFast,
}

impl ToString for ErrorMessage {
    fn to_string(&self) -> String {
        self.to_str().to_owned()
    }
}

impl From<ErrorMessage> for String {
    fn from(val: ErrorMessage) -> Self {
        val.to_string()
    }
}

impl ErrorMessage {
    fn to_str(&self) -> String {
        match self {
            ErrorMessage::ServerError => "Server Error. Please try again later".into(),
            ErrorMessage::WrongCredentials => "Email or password is wrong".into(),
            ErrorMessage::EmailExist => "An User with this email already exists".into(),
            ErrorMessage::UsernameExist => "An User with this username already exists".into(),
            ErrorMessage::UserExist => "User with this email(or username) already exists".into(),
            ErrorMessage::TagExist => "This device has been tagged by this tag".into(),
            ErrorMessage::NoChange => "No change to be done".into(),
            ErrorMessage::UserNotActivated => {
                "User is not activated (after registration) or is banned".into()
            }
            ErrorMessage::EmptyPassword => "Password cannot be empty".into(),
            ErrorMessage::HashingError => "Error while hashing password".into(),
            ErrorMessage::InvalidHashFormat => "Invalid password hash format".into(),
            ErrorMessage::ExceededMaxPasswordLength(max_length) => {
                format!("Password must not be more than {} characters", max_length)
            }
            ErrorMessage::InsufficientPasswordLength(min_length) => {
                format!("Password must not be less than {} characters", min_length)
            }
            ErrorMessage::InvalidUserInput => "The user input provided is illegal".into(),
            ErrorMessage::InvalidToken => "Authentication token is invalid or expired".into(),
            ErrorMessage::InvalidEmail => "Email format is invalid".into(),
            ErrorMessage::InvalidPassword => "Password format or length is invalid".into(),
            ErrorMessage::InvalidUsername => "Username format or length is invalid".into(),
            ErrorMessage::UpdateFailed => "Object not exists / not owned by you, or the update makes no change".into(),
            ErrorMessage::TokenNotProvided => "You are not logged in, please provide token".into(),
            ErrorMessage::PermissionDenied => {
                "You are not allowed to perform this action (resources not owned, or higher privilege required)".into()
            },
            ErrorMessage::TooFast => {"Your access is too frequent.".into()}
        }
    }
}

#[derive(Debug, Clone)]
pub struct HttpError {
    pub message: String,
    pub status: u16,
}

impl HttpError {
    pub fn new(message: impl Into<String>, status: u16) -> Self {
        HttpError {
            message: message.into(),
            status,
        }
    }

    pub fn server_error(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: 500,
        }
    }

    pub fn permission_denied(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: 403,
        }
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: 400,
        }
    }

    pub fn too_many_requests(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: 429,
        }
    }

    pub fn not_modified(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: 304,
        }
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: 404,
        }
    }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "HttpError: message: {}, status: {}",
            self.message, self.status
        )
    }
}

impl Error for HttpError {}

/// As an actix-web response error
impl ResponseError for HttpError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let cloned = self.clone();
        match cloned.status {
            400 => HttpResponse::BadRequest()
                .cookie(
                    actix_web::cookie::Cookie::build("token", "")
                        .path("/")
                        .max_age(actix_web::cookie::time::Duration::new(0, 0))
                        .http_only(true)
                        .finish(),
                )
                .json(Response {
                    status: "fail",
                    message: cloned.message,
                }),
            401 => HttpResponse::Unauthorized().json(Response {
                status: "fail",
                message: cloned.message,
            }),
            403 => HttpResponse::Forbidden().json(Response {
                status: "fail",
                message: cloned.message,
            }),
            404 => HttpResponse::NotFound().json(Response {
                status: "fail",
                message: cloned.message,
            }),
            429 => HttpResponse::TooManyRequests().json(Response {
                status: "fail",
                message: cloned.message,
            }),
            409 => HttpResponse::Conflict().json(Response {
                status: "fail",
                message: cloned.message,
            }),
            500 => HttpResponse::InternalServerError().json(Response {
                status: "error",
                message: cloned.message,
            }),
            _ => {
                eprintln!(
                    "Warning: Unknown response error type. Converted status code {} to 500.",
                    cloned.status
                );

                HttpResponse::InternalServerError().json(Response {
                    status: "error",
                    message: ErrorMessage::ServerError.into(),
                })
            }
        }
    }
}
