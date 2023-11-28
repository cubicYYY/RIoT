use actix_web::error::ErrorInternalServerError;
use actix_web::{FromRequest, HttpMessage};
use actix_web::web::Bytes;
use chrono::NaiveDateTime;
use diesel::mysql::Mysql;
use diesel::deserialize::{QueryableByName, Queryable};
use futures_util::future::{Ready, ready};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use crate::schema::*;
use crate::errors::*;
// HTTP Requests

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct LoginForm {
   username: String,
   password: String,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct RegisterForm {
   username: String,
   email: String,
   password: String,
}

// HTTP Responses

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Response {
   pub status: &'static str,
   pub message: String,
}

// Internal Data Structures & SQL Schemas

pub type UserPriv = u8;

#[derive(ToSchema, Queryable, Clone, Debug)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(Mysql))]
pub struct User {
   pub id: u32,
   pub username: String,
   pub email: String,
   pub password: String,
   pub privilege: UserPriv,
   pub api_key: String,
   /// Precision: milliseconds
   pub since: NaiveDateTime,
   pub activated: bool,
}

#[derive(ToSchema, Queryable, Clone, Debug)]
#[diesel(table_name = crate::schema::device)]
#[diesel(check_for_backend(Mysql))]
pub struct Device {
   id: u32,
   uid: u32,
   name: String,
   desc: String,
   dtype: u32, // TODO: Should we just use a string to describe it?
   /// Precision: milliseconds
   since: NaiveDateTime,
   /// Precision: milliseconds
   last_update: NaiveDateTime,
   pub activated: bool,
}

#[derive(ToSchema, Queryable, Clone, Debug)]
#[diesel(table_name = crate::schema::site)]
#[diesel(check_for_backend(Mysql))]
pub struct Site {
   id: u32,
   uid: u32,
   name: String,
   desc: String,
   pub activated: bool,
}

#[derive(ToSchema, Queryable, Clone, Debug)]
#[diesel(table_name = crate::schema::record)]
#[diesel(check_for_backend(Mysql))]
pub struct Record {
   id: u32,
   /// Device id
   did: u32,
   payload: Bytes,
   /// Precision: 32 bits
   latitude: Option<f32>,
   /// Precision: 32 bits
   longitude: Option<f32>,
   /// Precision: milliseconds
   timestamp: NaiveDateTime,
}

#[derive(ToSchema, Queryable, Clone, Debug)]
#[diesel(table_name = crate::schema::owns)]
#[diesel(check_for_backend(Mysql))]
pub struct Owns {
   /// Site id
   sid: u32,
   /// Device id
   did: u32,
}