use chrono::NaiveDateTime;

use chrono::naive::serde::{ts_microseconds_option, ts_milliseconds};
use diesel::deserialize::Queryable;
use diesel::mysql::Mysql;
use diesel::query_builder::AsChangeset;
use diesel::{Identifiable, Insertable, Selectable};
use validator::{Validate, ValidationError};

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// HTTP Requests

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct LoginForm {
    #[serde(alias = "username")]
    #[serde(alias = "email")]
    pub account: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct NewDeviceForm {
    pub name: String,
    pub desc: Option<String>,
    pub dtype: u32,
    /// Precision: 64 bits
    pub latitude: Option<f64>,
    /// Precision: 64 bits
    pub longitude: Option<f64>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct RecordForm {
    pub payload: Vec<u8>,
    /// Precision: milliseconds
    #[serde(with = "ts_microseconds_option")]
    pub timestamp: Option<NaiveDateTime>,
}

#[derive(Validate, Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct RegisterForm {
    #[validate(
        length(min = 4, max = 16, message = "Username must be 4-64 characters"),
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
pub struct DeviceForm {
    pub name: String,
    pub desc: Option<String>,
    pub dtype: u32,
    /// Precision: 64 bits
    pub latitude: Option<f64>,
    /// Precision: 64 bits
    pub longitude: Option<f64>,
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
    Everyone = 0,
    /// Banned or self-destructed account. No op is allowed.
    Suspended = 1,
    /// Full access of self-owned data
    Normal = 4,
    /// Full access of self-owned data + Site data read permission, no modification is allowed
    ViewerAdmin = 16,
    /// Full access of all data
    Admin = 256,
    /// Full access of the app (data + metadata + admin allocation / revoking)
    SuperAdmin = 1024,
}

#[derive(ToSchema, Serialize, Deserialize, Selectable, Queryable, Identifiable, Clone, Debug)]
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
    #[serde(with = "ts_milliseconds")]
    pub since: NaiveDateTime,
    pub activated: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Insertable)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(Mysql))]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    #[column_name = "password"]
    pub hashed_password: &'a str,
    pub privilege: u32,
}

#[derive(Clone, Debug, AsChangeset, Identifiable)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(Mysql))]
pub struct UpdateUser<'a> {
    pub id: u64,
    pub username: Option<&'a str>,
    pub email: Option<&'a str>,
    #[column_name = "password"]
    pub hashed_password: Option<&'a str>,
    pub privilege: Option<u32>,
    pub activated: Option<bool>,
    pub api_key: Option<Option<&'a str>>,
}

#[derive(ToSchema, Serialize, Deserialize, Selectable, Queryable, Identifiable, Clone, Debug)]
#[diesel(table_name = crate::schema::device)]
#[diesel(check_for_backend(Mysql))]
pub struct Device {
    pub id: u64,
    pub uid: u64,
    pub name: String,
    pub desc: Option<String>,
    pub dtype: u32, // TODO: Should we just use a string to describe it?
    /// Precision: 64 bits
    pub latitude: Option<f64>,
    /// Precision: 64 bits
    pub longitude: Option<f64>,
    /// Precision: milliseconds
    #[serde(with = "ts_milliseconds")]
    pub since: NaiveDateTime,
    /// Precision: milliseconds
    #[serde(with = "ts_milliseconds")]
    pub last_update: NaiveDateTime,
    pub activated: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Insertable)]
#[diesel(table_name = crate::schema::device)]
#[diesel(check_for_backend(Mysql))]
pub struct NewDevice<'a> {
    pub uid: u64,
    pub name: &'a str,
    pub desc: Option<&'a str>,
    pub dtype: u32,
    /// Precision: 64 bits
    pub latitude: Option<f64>,
    /// Precision: 64 bits
    pub longitude: Option<f64>,
}

#[derive(ToSchema, AsChangeset, Clone, Debug, Identifiable)]
#[diesel(table_name = crate::schema::device)]
#[diesel(check_for_backend(Mysql))]
pub struct UpdateDevice<'a> {
    pub id: u64,
    pub name: Option<&'a str>,
    pub desc: Option<Option<&'a str>>,
    /// Precision: 64 bits
    pub latitude: Option<Option<f64>>,
    /// Precision: 64 bits
    pub longitude: Option<Option<f64>>,
    /// Precision: milliseconds
    pub last_update: Option<&'a NaiveDateTime>,
    pub activated: Option<bool>,
}

#[derive(ToSchema, Selectable, Queryable, Insertable, Clone, Debug)]
#[diesel(table_name = crate::schema::site)]
#[diesel(check_for_backend(Mysql))]
pub struct Site {
    pub id: u64,
    pub uid: u64,
    pub name: String,
    pub desc: Option<String>,
    pub activated: bool,
}

#[derive(
    ToSchema, Serialize, Deserialize, Selectable, Queryable, Insertable, Identifiable, Clone, Debug,
)]
#[diesel(table_name = crate::schema::record)]
#[diesel(check_for_backend(Mysql))]
pub struct Record {
    id: u64,
    /// Device id
    did: u64,
    payload: Vec<u8>,
    /// Precision: milliseconds
    #[serde(with = "ts_milliseconds")]
    timestamp: NaiveDateTime,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug, Insertable)]
#[diesel(table_name = crate::schema::record)]
#[diesel(check_for_backend(Mysql))]
pub struct NewRecord {
    pub did: u64,
    pub payload: Vec<u8>,
    /// Precision: milliseconds
    #[serde(with = "ts_microseconds_option")]
    pub timestamp: Option<NaiveDateTime>,
}

#[derive(ToSchema, Queryable, Selectable, Insertable, Identifiable, Clone, Debug)]
#[diesel(table_name = crate::schema::owns)]
#[diesel(check_for_backend(Mysql))]
#[diesel(primary_key(sid, did))]
pub struct Owns {
    /// Site id
    sid: u64,
    /// Device id
    did: u64,
}
