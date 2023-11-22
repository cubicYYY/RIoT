use actix_web::web::Bytes;
use utoipa::ToSchema;

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

#[derive(ToSchema)]
pub struct User {
   id: u32,
   username: String,
   email: String,
   password: String,
   activated: bool,
   privilege: u8,
   since: u64,
}

#[derive(ToSchema)]
pub struct Device {
   id: u32,
   uid: u32,
   name: String,
   desc: String,
   dtype: u32, // TODO: Should we just use a string to describe it?
   since: u64,
   last_update: u64,
}

#[derive(ToSchema)]
pub struct Site {
   id: u32,
   uid: u32,
   name: String,
   desc: String,
   since: u64,
}

#[derive(ToSchema)]
pub struct Record {
   id: u32,
   /// Device id
   did: u32,
   payload: Bytes,
   latitude: Option<f32>,
   longitude: Option<f32>,
   timestamp: u64,
}

#[derive(ToSchema)]
pub struct Owns {
   /// Site id
   sid: u32,
   /// Device id
   did: u32,
}