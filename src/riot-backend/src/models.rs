



use chrono::NaiveDateTime;

use diesel::deserialize::Queryable;
use diesel::mysql::Mysql;
use diesel::{Insertable, Selectable};

use serde::{Serialize, Deserialize};
use utoipa::ToSchema;


// HTTP Requests

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct LoginForm {
   username: String,
   password: String,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct RegisterForm {
   pub username: String,
   pub email: String,
   pub password: String,
}

// HTTP Responses

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Response {
   pub status: &'static str,
   pub message: String,
}

// Internal Data Structures & SQL Schemas

pub enum UserPrivilege {
   Inactivate,
   Viewer,
   Normal,
   Admin,
   SuperAdmin,
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