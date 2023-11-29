use chrono::NaiveDateTime;

use diesel::deserialize::Queryable;
use diesel::mysql::Mysql;
use diesel::{Insertable, Selectable};
use validator::{Validate, ValidationError};

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// HTTP Requests

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct LoginForm {
    pub account: String,
    pub password: String,
}

#[derive(Validate, Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct RegisterForm {
    #[validate(
        length(min = 4, max = 16, message = "Username must be 4-64 characters"),
        custom = "validate_username"
    )]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(
        length(min = 8, max = 64, message = "Password must be 8-64 characters"),
        custom = "validate_pwd"
    )]
    pub password: String,
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

// HTTP Responses

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Response {
    pub status: &'static str,
    pub message: String,
}

// Internal Data Structures & SQL Schemas

/// Handy enum to set a proper privilege value, only for convenience, not a strong type constraint.
/// Reserved values for future uses.
pub enum UserPrivilege {
    /// Banned or self-destructed account. No op is allowed.
    Suspended = 0,
    /// Full access of self-owned data
    Normal = 4,
    /// Full access of self-owned data + Site data read permission, no modification is allowed
    ViewerAdmin = 16,
    /// Full access of all data
    Admin = 256,
    /// Full access of the app (data + metadata + admin allocation / revoking)
    SuperAdmin = 1024,
}

#[derive(ToSchema, Queryable, Selectable, Insertable, Clone, Debug)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(Mysql))]
pub struct User {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub password: String,
    pub privilege: u32,
    pub api_key: Option<String>,
    /// Precision: milliseconds
    pub since: NaiveDateTime,
    pub activated: bool,
}

#[derive(ToSchema, Queryable, Selectable, Insertable, Clone, Debug)]
#[diesel(table_name = crate::schema::device)]
#[diesel(check_for_backend(Mysql))]
pub struct Device {
    id: u64,
    uid: u64,
    name: String,
    desc: Option<String>,
    dtype: u32, // TODO: Should we just use a string to describe it?
    /// Precision: milliseconds
    since: NaiveDateTime,
    /// Precision: milliseconds
    last_update: NaiveDateTime,
    pub activated: bool,
}

#[derive(ToSchema, Queryable, Selectable, Insertable, Clone, Debug)]
#[diesel(table_name = crate::schema::site)]
#[diesel(check_for_backend(Mysql))]
pub struct Site {
    id: u64,
    uid: u64,
    name: String,
    desc: Option<String>,
    pub activated: bool,
}

#[derive(ToSchema, Queryable, Selectable, Insertable, Clone, Debug)]
#[diesel(table_name = crate::schema::record)]
#[diesel(check_for_backend(Mysql))]
pub struct Record {
    id: u64,
    /// Device id
    did: u64,
    payload: Vec<u8>,
    /// Precision: 64 bits
    latitude: Option<f64>,
    /// Precision: 64 bits
    longitude: Option<f64>,
    /// Precision: milliseconds
    timestamp: NaiveDateTime,
}

#[derive(ToSchema, Queryable, Selectable, Insertable, Clone, Debug)]
#[diesel(table_name = crate::schema::owns)]
#[diesel(check_for_backend(Mysql))]
pub struct Owns {
    /// Site id
    sid: u64,
    /// Device id
    did: u64,
}
