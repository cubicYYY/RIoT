use std::ops::Deref;

use crate::{
    models::{NewDevice, NewRecord, UpdateDevice},
    UserPrivilege,
};
use actix_web::{
    delete, get, post, put,
    web::{self},
    HttpResponse, Responder, ResponseError,
};
use chrono::Utc;
use diesel::result::Error as DieselErr;
use log::{error, info};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::{
    errors::{ErrorMessage, HttpError},
    middlewares::{AuthenticatedUser, RequireAuth},
    models::Response,
    AppState,
};

#[derive(Validate, Serialize, Deserialize, ToSchema, Clone, Debug)]
/// Web json form to add a new device
pub struct NewDeviceForm { //TODO: validate
    pub name: String,
    pub desc: Option<String>,
    pub dtype: u32,
    /// Precision: 64 bits
    pub latitude: Option<f64>,
    /// Precision: 64 bits
    pub longitude: Option<f64>,
    pub topic: String,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
/// Web json form (wrapper) to upload data
pub struct RecordForm {
    pub payload: Vec<u8>,
}

#[derive(Validate, Serialize, Deserialize, ToSchema, Clone, Debug)]
/// Web json form to update a device
pub struct UpdateDeviceForm {
    pub name: Option<String>,
    pub desc: Option<Option<String>>,
    pub dtype: Option<u32>,
    /// Precision: 64 bits
    pub latitude: Option<Option<f64>>,
    /// Precision: 64 bits
    pub longitude: Option<Option<f64>>,
}

#[utoipa::path(
        get,
        context_path = "/api",
        path = "/devices",
        tag = "Device",
        responses(
            (status = 200, description = "Devices", body = Vec<Device>),
            (status = 403, description = "Permission denied", body = Response),
            (status = 500, description = "Internal error, contact webtag admin", body = Response)
        ),
        params(),
        security(
            ("jwt_header" = []),
            ("jwt_cookie" = [])
        )
    )]
#[get(
    "/devices",
    wrap = "RequireAuth::with_priv_level(UserPrivilege::Normal as u32)"
)]
/// List all devices owned by the user ("deleted" devices included)
pub(crate) async fn owned_devices(
    cur_user: AuthenticatedUser,
    app: web::Data<AppState>,
) -> impl Responder {
    let devices = app.get_owned_devices(cur_user.id).await;
    match devices {
        Ok(devices) => HttpResponse::Ok().json(devices),
        Err(e) => {
            error!("{:?}", e);
            HttpError::server_error(ErrorMessage::ServerError).error_response()
        }
    }
}

#[utoipa::path(
        post,
        context_path = "/api",
        path = "/devices",
        tag = "Device",
        request_body(
            content = NewDeviceForm,
            description = "Form for a new device", 
            example = json!(
                {
                    "name":"test_device",
                    "desc":"(optional)McDonald",
                    "dtype":1,
                    "latitude":114.514,
                    "longitude":19.19810,
                    "topic":"test-key/home/light"
                })
        ),
        responses(
            (status = 200, description = "Added a new device, message=device id", body = Response),
            (status = 401, description = "Unauthorized", body = Response),
            (status = 500, description = "Internal error, contact webtag admin", body = Response)
        ),
        security(
            ("jwt_header" = []),
            ("jwt_cookie" = [])
        )
    )]
#[post(
    "/devices",
    wrap = "RequireAuth::with_priv_level(UserPrivilege::Normal as u32)"
)]
/// Add a new device
pub(crate) async fn add_device(
    cur_user: AuthenticatedUser,
    app: web::Data<AppState>,
    form: web::Json<NewDeviceForm>,
) -> impl Responder {
    if let Err(e) = form.0.validate() {
        info!("Illegal input detected: {:?}", e);
        return HttpError::new(e.to_string(), 400).error_response();
    }

    let NewDeviceForm {
        name,
        desc,
        dtype,
        latitude,
        longitude,
        topic,
    } = form.into_inner();

    let device = NewDevice {
        uid: cur_user.id,
        name: &name,
        desc: desc.as_deref(),
        dtype,
        latitude,
        longitude,
        topic: &topic,
    };

    match app.add_device(&device).await {
        Ok(id) => HttpResponse::Ok().json(Response {
            status: "ok",
            message: id.to_string(),
        }),
        Err(e) => {
            error!("{:?}", e);
            HttpError::server_error(ErrorMessage::ServerError).error_response()
        }
    }
}

#[utoipa::path(
            get,
            context_path = "/api",
            path = "/devices/{did}",
            tag = "Device",
            responses(
                (status = 200, description = "Device info", body = Device),
                (status = NOT_FOUND, description = "Device was not found")
            ),
            security(
                ("jwt_header" = []),
                ("jwt_cookie" = [])
            )
        )]
#[get(
    "/devices/{did}",
    wrap = "RequireAuth::with_priv_level(UserPrivilege::Normal as u32)"
)]
/// Device info
pub(crate) async fn device_info(
    path: web::Path<u64>,
    app: web::Data<AppState>,
    cur_user: AuthenticatedUser,
) -> impl Responder {
    let did = path.into_inner();
    match app.get_device_by_id(did).await {
        Ok(device) => {
            if device.uid == cur_user.id {
                HttpResponse::Ok().json(device)
            } else {
                HttpError::not_found(ErrorMessage::ObjectNotFound).error_response()
            }
        }
        Err(DieselErr::NotFound) => {
            HttpError::not_found(ErrorMessage::ObjectNotFound).error_response()
        }
        Err(e) => {
            error!("{:?}", e);
            HttpError::server_error(ErrorMessage::ServerError).error_response()
        }
    }
}

#[utoipa::path(
        delete,
        context_path = "/api",
        path = "/devices/{did}",
        tag = "Device",
        responses(
            (status = 200, description = "Delete success", body = Response),
            (status = 401, description = "Unauthorized", body = Response),
            (status = 404, description = "Device was not found or the device is not yours \
        and you do not have enough privilege to delete it", body = Response),
            (status = 500, description = "Internal error, contact webtag admin", body = Response)
        ),
        security(
            ("jwt_header" = []),
            ("jwt_cookie" = [])
        )
    )]
#[delete(
    "/devices/{did}",
    wrap = "RequireAuth::with_priv_level(UserPrivilege::Normal as u32)"
)]
/// Delete a device (soft delete, i.e. deactivate it)
pub(crate) async fn del_device(
    path: web::Path<u64>,
    app: web::Data<AppState>,
    cur_user: AuthenticatedUser,
) -> impl Responder {
    let did = path.into_inner();
    match app
        .update_device(
            &UpdateDevice {
                id: did,
                name: None,
                desc: None,
                dtype: None,
                latitude: None,
                longitude: None,
                last_update: None,
                activated: Some(false),
            },
            Some(cur_user.id),
        )
        .await
    {
        Ok(1) => HttpResponse::Ok().json(Response {
            status: "ok",
            message: "".into(),
        }),
        Ok(_) => HttpError::not_found(ErrorMessage::ObjectNotFound).error_response(),
        Err(e) => {
            error!("{:?}", e);
            HttpError::server_error(ErrorMessage::ServerError).error_response()
        }
    }
}

#[utoipa::path(
        put,
        context_path = "/api",
        path = "/devices/{did}",
        tag = "Device",
        request_body(
            content = UpdateDeviceForm,
            description = "Form for a new device", 
            example = json!(
                {
                    "name":"new_name",
                    "desc":"Balala",
                    "dtype":"1",
                    "latitude":"114.514",
                    "longitude":"19.19810"
                })
        ),
        responses(
            (status = 200, description = "Update successed", body = Response),
            (status = 401, description = "Unauthorized", body = Response),
            (status = 404, description = "Device was not found or the device is not yours \
        and you do not have enough privilege to delete it", body = Response),
            (status = 500, description = "Internal error, contact webtag admin", body = Response)
        ),
        security(
            ("jwt_header" = []),
            ("jwt_cookie" = [])
        )
    )]
#[put(
    "/devices/{did}",
    wrap = "RequireAuth::with_priv_level(UserPrivilege::Normal as u32)"
)]
/// Update a device's information
pub(crate) async fn upd_device_info(
    path: web::Path<u64>,
    app: web::Data<AppState>,
    cur_user: AuthenticatedUser,
    form: web::Json<UpdateDeviceForm>,
) -> impl Responder {
    let did = path.into_inner();

    if let Err(e) = form.deref().validate() {
        info!("Illegal input detected: {:?}", e);
        return HttpError::new(e.to_string(), 400).error_response();
    }

    let UpdateDeviceForm {
        name,
        desc,
        dtype,
        latitude,
        longitude,
    } = form.into_inner();

    match app
        .update_device(
            &UpdateDevice {
                id: did,
                name: name.as_deref(),
                desc: desc.as_ref().map(|inner| inner.as_deref()),
                dtype,
                latitude,
                longitude,
                last_update: None,
                activated: None,
            },
            Some(cur_user.id),
        )
        .await
    {
        Ok(1) => HttpResponse::Ok().json(Response {
            status: "ok",
            message: "".into(),
        }),
        Ok(_) => HttpError::not_found(ErrorMessage::ObjectNotFound).error_response(),
        Err(e) => {
            error!("{:?}", e);
            HttpError::server_error(ErrorMessage::ServerError).error_response()
        }
    }
}

#[utoipa::path(
        get,
        context_path = "/api",
        path = "/devices/{did}/records",
        tag = "Record",
        responses(
            (status = 200, description = "Records of the device", body = Vec<Record>),
            (status = 401, description = "Unauthorized", body = Response),
            (status = 404, description = "Device was not found or the device is not yours \
        and you do not have enough privilege to delete it", body = Response),
            (status = 500, description = "Internal error, contact webtag admin", body = Response)
        ),
        security(
            ("jwt_header" = []),
            ("jwt_cookie" = [])
        )
    )]
#[get(
    "/devices/{did}/records",
    wrap = "RequireAuth::with_priv_level(UserPrivilege::Normal as u32)"
)]
/// Device records
pub(crate) async fn device_records(
    path: web::Path<u64>,
    app: web::Data<AppState>,
    cur_user: AuthenticatedUser,
) -> impl Responder {
    let did = path.into_inner();
    if Ok(true) == app.device_belongs_to(did, cur_user.id).await {
    } else {
        return HttpError::not_found(ErrorMessage::ObjectNotFound).error_response();
    }
    let records = app.get_device_records(did).await;
    match records {
        Ok(records) => HttpResponse::Ok().json(records),
        Err(e) => {
            error!("{:?}", e);
            HttpError::server_error(ErrorMessage::ServerError).error_response()
        }
    }
}

#[utoipa::path(
        post,
        context_path = "/api",
        path = "/devices/{did}/records",
        tag = "Record",
        request_body(
            content=RecordForm,
            description="Record submit. Mainly for tests. Timestamp is of millisecond precision",
            example = json!({"payload":[1,1,4,5,1,4], "timestamp":1145141919})
        ),
        responses(
            (status = 200, description = "Insert record success", body = Response),
            (status = 401, description = "Unauthorized", body = Response),
            (status = 404, description = "Device was not found or the device is not yours \
        and you do not have enough privilege to delete it", body = Response),
            (status = 500, description = "Internal error, contact webtag admin", body = Response)
        ),
        security(
            ("jwt_header" = []),
            ("jwt_cookie" = [])
        )
    )]
#[post(
    "/devices/{did}/records",
    wrap = "RequireAuth::with_priv_level(UserPrivilege::Normal as u32)"
)]
/// Insert new device data record
pub(crate) async fn insert_device_records(
    path: web::Path<u64>,
    app: web::Data<AppState>,
    form: web::Json<RecordForm>,
    cur_user: AuthenticatedUser,
) -> impl Responder {
    let did = path.into_inner();
    if Ok(true) == app.device_belongs_to(did, cur_user.id).await {
    } else {
        return HttpError::not_found(ErrorMessage::ObjectNotFound).error_response();
    }
    let RecordForm { payload } = form.into_inner();

    match app
        .add_device_records(&NewRecord {
            did,
            payload: &payload,
            timestamp: &Utc::now().naive_utc(),
        })
        .await
    {
        Ok(_) => HttpResponse::Ok().json(Response {
            status: "ok",
            message: "".into(),
        }),
        Err(e) => {
            error!("{:?}", e);
            HttpError::server_error(ErrorMessage::ServerError).error_response()
        }
    }
}
