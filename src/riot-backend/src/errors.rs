use std::{fmt, error::Error};

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

#[derive(Debug, PartialEq)]
pub enum ErrorMessage {
    EmptyPassword,
    ExceededMaxPasswordLength(usize),
    InsufficientPasswordLength(usize),
    HashingError,
    InvalidHashFormat,
    InvalidToken,
    ServerError,
    WrongCredentials,
    EmailExist,
    UserNoLongerExist,
    TokenNotProvided,
    PermissionDenied,
}

impl ToString for ErrorMessage {
    fn to_string(&self) -> String {
        self.to_str().to_owned()
    }
}

impl Into<String> for ErrorMessage {
    fn into(self) -> String {
        self.to_string()
    }
}

impl ErrorMessage {
    fn to_str(&self) -> String {
        match self {
            ErrorMessage::ServerError => "Server Error. Please try again later".into(),
            ErrorMessage::WrongCredentials => "Email or password is wrong".into(),
            ErrorMessage::EmailExist => "An User with this email already exists".into(),
            ErrorMessage::UserNoLongerExist => {
                "User belonging to this token no longer exists".into()
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
            ErrorMessage::InvalidToken => "Authentication token is invalid or expired".into(),
            ErrorMessage::TokenNotProvided => {
                "You are not logged in, please provide token".into()
            }
            ErrorMessage::PermissionDenied => {
                "You are not allowed to perform this action (higher privilege required)".into()
            }
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

    pub fn bad_request(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: 400,
        }
    }

    pub fn unique_constraint_voilation(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: 409,
        }
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: 401,
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
            400 => HttpResponse::BadRequest().json(Response {
                status: "fail",
                message: cloned.message.into(),
            }),
            401 => HttpResponse::Unauthorized().json(Response {
                status: "fail",
                message: cloned.message.into(),
            }),
            409 => HttpResponse::Conflict().json(Response {
                status: "fail",
                message: cloned.message.into(),
            }),
            500 => HttpResponse::InternalServerError().json(Response {
                status: "error",
                message: cloned.message.into(),
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
