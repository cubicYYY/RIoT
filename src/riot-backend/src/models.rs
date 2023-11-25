use actix_web::web::Bytes;
use chrono::{DateTime, NaiveDate};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

// HTTP Requests

#[derive(ToSchema)]
pub struct LoginForm {
   username: String,
   password: String,
}

#[derive(ToSchema)]
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

// Internal Data Structures

pub type UserPriv = u8;

#[derive(ToSchema, Clone)]
pub struct User {
   pub id: u32,
   pub username: String,
   pub email: String,
   pub password: String,
   pub activated: bool,
   pub privilege: UserPriv,

   /// Precision: milliseconds
   pub since: u64,
}

// SQL Schemas

#[derive(ToSchema)]
pub struct Device {
   id: u32,
   uid: u32,
   name: String,
   desc: String,
   dtype: u32, // TODO: Should we just use a string to describe it?
   /// Precision: milliseconds
   since: NaiveDate,
   /// Precision: milliseconds
   last_update: NaiveDate,
}

#[derive(ToSchema)]
pub struct Site {
   id: u32,
   uid: u32,
   name: String,
   desc: String,
}

#[derive(ToSchema)]
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
   timestamp: NaiveDate,
}

#[derive(ToSchema)]
pub struct Owns {
   /// Site id
   sid: u32,
   /// Device id
   did: u32,
}