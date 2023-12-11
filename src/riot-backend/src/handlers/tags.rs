use crate::{
    app_context::AppState,
    errors::{ErrorMessage, HttpError},
    middlewares::AuthenticatedUser,
    models::{NewTag, Response, UpdateTag},
    UserPrivilege,
};
use actix_web::{
    delete, get, post, put,
    web::{self},
    HttpResponse, Responder, ResponseError,
};
use diesel::result::Error as DieselErr;
use log::{error, info};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::middlewares::RequireAuth;

#[derive(Validate, Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct UpdateTagForm {
    pub name: Option<String>,
    pub desc: Option<Option<String>>,
}

#[derive(Validate, Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct TagDeviceForm {
    pub did: u64,
}

#[derive(Validate, Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct NewTagForm {
    pub name: String,
    pub desc: Option<String>,
}
// tags
#[utoipa::path(
        get,
        context_path = "/api",
        path = "/tags",
        tag = "Tag",
        responses(
            (status = 200, description = "Tags owned by the user", body = Vec<Tag>),
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
    "/tags",
    wrap = "RequireAuth::with_priv_level(UserPrivilege::Normal as u32)"
)]
pub(crate) async fn owned_tags(
    cur_user: AuthenticatedUser,
    app: web::Data<AppState>,
) -> impl Responder {
    let tags = app.get_owned_tags(cur_user.id).await;
    match tags {
        Ok(tags) => HttpResponse::Ok().json(tags),
        Err(e) => {
            error!("{:?}", e);
            HttpError::server_error(ErrorMessage::ServerError).error_response()
        }
    }
}

#[utoipa::path(
    post,
    context_path = "/api",
    path = "/tags",
    tag = "Tag",
    request_body(content=NewTagForm),
        responses(
        (status = 200, description = "Added a new tag", body = Response),
        (status = 401, description = "Unauthorized", body = Response),
        (status = 500, description = "Internal error, contact web admin", body = Response)
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
pub(crate) async fn add_tag(
    cur_user: AuthenticatedUser,
    app: web::Data<AppState>,
    form: web::Json<NewTagForm>,
) -> impl Responder {
    if let Err(e) = form.0.validate() {
        info!("Illegal input detected: {:?}", e);
        return HttpError::new(e.to_string(), 400).error_response();
    }

    let NewTagForm { name, desc } = form.into_inner();

    let tag = NewTag {
        uid: cur_user.id,
        name: &name,
        desc: desc.as_deref(),
        activated: true,
    };

    match app.add_tag(&tag).await {
        Ok(_) => HttpResponse::Ok().json(Response {
            status: "ok",
            message: "".into(),
        }),
        Err(e) => {
            error!("{:?}", e);
            HttpError::new(ErrorMessage::ServerError, 500).error_response()
        }
    }
}

#[utoipa::path(
        get,
        context_path = "/api",
        path = "/tags/{tid}",
        tag = "Tag",
        responses(
            (status = 200, description = "Tag info", body = Tag),
            (status = NOT_FOUND, description = "Tag was not found")
        ),
        security(
            ("jwt_header" = []),
            ("jwt_cookie" = [])
        )
    )]
#[get(
    "/tags/{tid}",
    wrap = "RequireAuth::with_priv_level(UserPrivilege::Normal as u32)"
)]
pub(crate) async fn tag_info(
    path: web::Path<u64>,
    app: web::Data<AppState>,
    cur_user: AuthenticatedUser,
) -> impl Responder {
    let tid = path.into_inner();
    match app.get_tag_by_id(tid).await {
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
        path = "/tags/{tid}",
        tag = "Tag",
        responses(
            (status = 200, description = "Delete success", body = Response),
            (status = 401, description = "Unauthorized", body = Response),
            (status = 404, description = "Tag was not found or the tag is not yours \
        and you do not have enough privilege to delete it", body = Response),
            (status = 500, description = "Internal error, contact webtag admin", body = Response)
        ),
        security(
            ("jwt_header" = []),
            ("jwt_cookie" = [])
        )
    )]
#[delete(
    "/tags/{tid}",
    wrap = "RequireAuth::with_priv_level(UserPrivilege::Normal as u32)"
)]
pub(crate) async fn del_tag(
    path: web::Path<u64>,
    app: web::Data<AppState>,
    cur_user: AuthenticatedUser,
) -> impl Responder {
    let tid = path.into_inner();
    match app
        .update_tag(
            &UpdateTag {
                id: tid,
                name: None,
                desc: None,
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
        path = "/tags/{tid}",
        tag = "Tag",
        request_body(content=UpdateTagForm),
        responses(
            (status = 200, description = "Update successed", body = Response),
            (status = 401, description = "Unauthorized", body = Response),
            (status = 404, description = "Tag was not found or the tag is not yours \
        and you do not have enough privilege to delete it", body = Response),
            (status = 500, description = "Internal error, contact webtag admin", body = Response)
        ),
        security(
            ("jwt_header" = []),
            ("jwt_cookie" = [])
        )
    )]
#[put(
    "/tags/{tid}",
    wrap = "RequireAuth::with_priv_level(UserPrivilege::Normal as u32)"
)]
pub(crate) async fn upd_tag_info(
    path: web::Path<u64>,
    app: web::Data<AppState>,
    cur_user: AuthenticatedUser,
    form: web::Json<UpdateTagForm>,
) -> impl Responder {
    let tid = path.into_inner();
    let UpdateTagForm { name, desc } = form.into_inner();

    match app
        .update_tag(
            &UpdateTag {
                id: tid,
                name: name.as_deref(),
                desc: desc.as_ref().map(|inner| inner.as_deref()),
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
        path = "/tags/{tid}/devices",
        tag = "Tag",
        responses(
            (status = 200, description = "Devices tagged by this tag", body = Vec<Device>),
            (status = 401, description = "Unauthorized", body = Response),
            (status = 404, description = "Tag was not found or the tag is not yours \
        and you do not have enough privilege to delete it", body = Response),
            (status = 500, description = "Internal error, contact webtag admin", body = Response)
        ),
        security(
            ("jwt_header" = []),
            ("jwt_cookie" = [])
        )
    )]
#[get(
    "/tags/{tid}/devices",
    wrap = "RequireAuth::with_priv_level(UserPrivilege::Normal as u32)"
)]
// Devices tagged with this tag
pub(crate) async fn tagged_devices(
    path: web::Path<u64>,
    app: web::Data<AppState>,
    cur_user: AuthenticatedUser,
) -> impl Responder {
    let tid = path.into_inner();
    if Ok(true) == app.tag_belongs_to(tid, cur_user.id).await {
    } else {
        return HttpError::not_found(ErrorMessage::ObjectNotFound).error_response();
    }
    let dids = app.get_dids_under_tag(tid).await;
    let dids = match dids {
        Ok(dids) => dids,
        Err(e) => {
            error!("{:?}", e);
            return HttpError::server_error(ErrorMessage::ServerError).error_response();
        }
    };
    match app.get_device_by_ids(dids.as_ref()).await {
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
    path = "/tags",
    tag = "Tag",
    request_body(content=TagDeviceForm),
    responses(
        (status = 200, description = "Tag a device", body = Response),
        (status = 401, description = "Unauthorized", body = Response),
        (status = 404, description = "Device/tag was not found or the device/tag is not yours \
        and you do not have enough privilege to delete it", body = Response),
        (status = 500, description = "Internal error, contact webtag admin", body = Response)
    ),
    security(
        ("jwt_header" = []),
        ("jwt_cookie" = [])
    )
)]
#[post(
    "/tags/{tid}/devices",
    wrap = "RequireAuth::with_priv_level(UserPrivilege::Normal as u32)"
)]
// Tag a device
pub(crate) async fn tag_device(
    path: web::Path<u64>,
    app: web::Data<AppState>,
    cur_user: AuthenticatedUser,
    form: web::Json<TagDeviceForm>,
) -> impl Responder {
    let tid = path.into_inner();
    if Ok(true) == app.tag_belongs_to(tid, cur_user.id).await {
    } else {
        return HttpError::not_found(ErrorMessage::ObjectNotFound).error_response();
    }
    let TagDeviceForm { did } = form.into_inner();
    if Ok(true) == app.device_belongs_to(did, cur_user.id).await {
    } else {
        return HttpError::not_found(ErrorMessage::ObjectNotFound).error_response();
    }
    match app.tag_device(tid, did).await {
        Ok(_) => HttpResponse::Ok().json(Response {
            status: "ok",
            message: "".into(),
        }),
        Err(e) => {
            error!("{:?}", e);
            HttpError::new(ErrorMessage::ServerError, 500).error_response()
        }
    }
}

#[utoipa::path(
    delete,
    context_path = "/api",
    path = "/tags/{tid}/devices",
    tag = "Tag",
    request_body(content=TagDeviceForm),
    responses(
        (status = 200, description = "Delete success", body = Response),
        (status = 401, description = "Unauthorized", body = Response),
        (status = 404, description = "Device/tag was not found or the device/tag is not yours \
    and you do not have enough privilege to delete it", body = Response),
        (status = 500, description = "Internal error, contact webtag admin", body = Response)
    ),
    security(
        ("jwt_header" = []),
        ("jwt_cookie" = [])
    )
)]
#[delete(
    "/tags/{tid}/devices",
    wrap = "RequireAuth::with_priv_level(UserPrivilege::Normal as u32)"
)]
// Remove a device tag
pub(crate) async fn untag_device(
    path: web::Path<u64>,
    app: web::Data<AppState>,
    cur_user: AuthenticatedUser,
    form: web::Json<TagDeviceForm>,
) -> impl Responder {
    let tid = path.into_inner();
    if Ok(true) == app.tag_belongs_to(tid, cur_user.id).await {
    } else {
        return HttpError::not_found(ErrorMessage::ObjectNotFound).error_response();
    }
    let TagDeviceForm { did } = form.into_inner();
    if Ok(true) == app.device_belongs_to(did, cur_user.id).await {
    } else {
        return HttpError::not_found(ErrorMessage::ObjectNotFound).error_response();
    }
    match app.untag_device(tid, did).await {
        Ok(_) => HttpResponse::Ok().json(Response {
            status: "ok",
            message: "".into(),
        }),
        Err(e) => {
            error!("{:?}", e);
            HttpError::new(ErrorMessage::ServerError, 500).error_response()
        }
    }
}