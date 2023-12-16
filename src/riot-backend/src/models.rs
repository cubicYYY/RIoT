use chrono::NaiveDateTime;

use chrono::naive::serde::ts_milliseconds;
use diesel::deserialize::Queryable;
use diesel::mysql::Mysql;
use diesel::query_builder::AsChangeset;
use diesel::{Identifiable, Insertable, Selectable};

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// HTTP Requests

// HTTP Responses

#[derive(Serialize, Deserialize, ToSchema)]
/// HTTP universal response form.
/// `message` segment can be used to transfer some handy optional data.
pub struct Response {
    pub status: &'static str,
    pub message: String,
}

// Internal Data Structures & SQL Schemas

/// Handy enum to set a proper privilege value, only for convenience, not a strong type constraint.
/// Reserved values for future uses.
#[allow(unused)]
pub enum UserPrivilege {
    Everyone = 0,
    /// Banned or self-destructed account. No op is allowed.
    Suspended = 1,
    /// Full access of self-owned data
    Normal = 4,
    /// Full access of self-owned data + Tag data read permission, no modification is allowed
    ViewerAdmin = 16,
    /// Full access of all data
    Admin = 256,
    /// Full access of the app (data + metadata + admin allocation / revoking)
    SuperAdmin = 1024,
}

#[derive(ToSchema, Serialize, Deserialize, Selectable, Queryable, Identifiable, Clone, Debug)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(Mysql))]
/// RIoT platform user
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
    #[diesel(column_name = password)]
    pub hashed_password: &'a str,
    pub privilege: u32,
    pub api_key: Option<&'a str>,
}

#[derive(Clone, Debug, AsChangeset, Identifiable)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(Mysql))]
pub struct UpdateUser<'a> {
    pub id: u64,
    pub username: Option<&'a str>,
    pub email: Option<&'a str>,
    #[diesel(column_name = password)]
    pub hashed_password: Option<&'a str>,
    pub privilege: Option<u32>,
    pub activated: Option<bool>,
    pub api_key: Option<Option<&'a str>>,
}

#[derive(ToSchema, Serialize, Deserialize, Selectable, Queryable, Identifiable, Clone, Debug)]
#[diesel(table_name = crate::schema::device)]
#[diesel(check_for_backend(Mysql))]
/// IoT device
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
    pub topic: &'a str,
}

#[derive(AsChangeset, Clone, Debug, Identifiable)]
#[diesel(table_name = crate::schema::device)]
#[diesel(check_for_backend(Mysql))]
pub struct UpdateDevice<'a> {
    pub id: u64,
    pub name: Option<&'a str>,
    pub desc: Option<Option<&'a str>>,
    pub dtype: Option<u32>,
    /// Precision: 64 bits
    pub latitude: Option<Option<f64>>,
    /// Precision: 64 bits
    pub longitude: Option<Option<f64>>,
    /// Precision: milliseconds
    pub last_update: Option<&'a NaiveDateTime>,
    pub activated: Option<bool>,
}

#[derive(ToSchema, Serialize, Deserialize, Selectable, Queryable, Insertable, Clone, Debug)]
#[diesel(table_name = crate::schema::tag)]
#[diesel(check_for_backend(Mysql))]
/// Tag for devices
pub struct Tag {
    pub id: u64,
    pub uid: u64,
    pub name: String,
    pub desc: Option<String>,
    pub activated: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Insertable)]
#[diesel(table_name = crate::schema::tag)]
#[diesel(check_for_backend(Mysql))]
pub struct NewTag<'a> {
    pub uid: u64,
    pub name: &'a str,
    pub desc: Option<&'a str>,
    pub activated: bool,
}

#[derive(AsChangeset, Clone, Debug, Identifiable)]
#[diesel(table_name = crate::schema::tag)]
#[diesel(check_for_backend(Mysql))]
pub struct UpdateTag<'a> {
    pub id: u64,
    pub name: Option<&'a str>,
    pub desc: Option<Option<&'a str>>,
    pub activated: Option<bool>,
}

#[derive(
    ToSchema, Serialize, Deserialize, Selectable, Queryable, Insertable, Identifiable, Clone, Debug,
)]
#[diesel(table_name = crate::schema::record)]
#[diesel(check_for_backend(Mysql))]
/// Device data record
pub struct Record {
    id: u64,
    /// Device id
    did: u64,
    payload: Vec<u8>,
    /// Precision: milliseconds
    #[serde(with = "ts_milliseconds")]
    timestamp: NaiveDateTime,
}

#[derive(Clone, Debug, Insertable)]
#[diesel(table_name = crate::schema::record)]
#[diesel(check_for_backend(Mysql))]
pub struct NewRecord<'a> {
    pub did: u64,
    pub payload: &'a [u8],
    /// Precision: milliseconds
    pub timestamp: &'a NaiveDateTime,
}

#[derive(ToSchema, Queryable, Selectable, Insertable, Identifiable, Clone, Debug)]
#[diesel(table_name = crate::schema::owns)]
#[diesel(check_for_backend(Mysql))]
#[diesel(primary_key(tid, did))]
/// Device(did) owns a Tag(tid)
pub struct Owns {
    /// Tag id
    tid: u64,
    /// Device id
    did: u64,
}
