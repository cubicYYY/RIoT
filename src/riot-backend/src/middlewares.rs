use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::error::{ErrorForbidden, ErrorInternalServerError, ErrorUnauthorized};
use actix_web::guard::Options;
use actix_web::http::header;
use actix_web::{http, web, FromRequest, HttpMessage, ResponseError};
use futures_util::future::{ready, LocalBoxFuture, Ready};
use futures_util::FutureExt;
use std::rc::Rc;
use std::task::{Context, Poll};

use crate::errors::{ErrorMessage, ErrorResponse, HttpError};
use crate::jwt_utils::parse_token;
use crate::models::{User, UserPriv};
use crate::AppState;

pub struct AuthenticatedUser(User);

impl FromRequest for AuthenticatedUser {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let value = req.extensions().get::<User>().cloned();
        let result = match value {
            Some(user) => Ok(AuthenticatedUser(user)),
            None => Err(ErrorInternalServerError(HttpError::server_error(
                "Authentication Error",
            ))),
        };
        ready(result)
    }
}

impl std::ops::Deref for AuthenticatedUser {
    type Target = User;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct RequireAuth {
    pub priv_needed: Rc<UserPriv>,
}

impl RequireAuth {
    pub fn with_priv_level(priv_level: UserPriv) -> Self {
        RequireAuth {
            priv_needed: Rc::new(priv_level),
        }
    }
}

impl<S> Transform<S, ServiceRequest> for RequireAuth
where
    S: Service<
            ServiceRequest,
            Response = ServiceResponse<actix_web::body::BoxBody>,
            Error = actix_web::Error,
        > + 'static,
{
    type Response = ServiceResponse<actix_web::body::BoxBody>;
    type Error = actix_web::Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware {
            service: Rc::new(service),
            allowed_roles: self.priv_needed.clone(),
        }))
    }
}

pub struct AuthMiddleware<S> {
    service: Rc<S>,
    allowed_roles: Rc<UserPriv>,
}

impl<S> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<
            ServiceRequest,
            Response = ServiceResponse<actix_web::body::BoxBody>,
            Error = actix_web::Error,
        > + 'static,
{
    type Response = ServiceResponse<actix_web::body::BoxBody>;
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, actix_web::Error>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let token = req
            .cookie("token")
            .map(|c| c.value().to_string())
            .or_else(|| {
                req.headers()
                    .get(http::header::AUTHORIZATION)
                    .map(|h| h.to_str().unwrap())
                    .and_then(|token| token.strip_prefix("Bearer ").map(String::from))
            });

        // No token in Cookie or Header
        if token.is_none() {
            let json_error = ErrorResponse {
                status: "fail".to_string(),
                message: ErrorMessage::TokenNotProvided.to_string(),
            };
            return Box::pin(ready(Err(ErrorUnauthorized(json_error))));
        }

        let app_state = req.app_data::<web::Data<AppState>>().unwrap();
        let user_id = match parse_token(&token.unwrap(), app_state.env.jwt_secret.as_bytes()) {
            Ok(id) => id,
            Err(e) => {
                let json_error = ErrorResponse {
                    status: "fail".to_string(),
                    message: e.message,
                };
                return Box::pin(ready(Err(ErrorUnauthorized(json_error))))
            }
        };

        // Now the user identity is verified, start authentication checking
        let cloned_app_state = app_state.clone();
        let allowed_roles = self.allowed_roles.clone();
        let srv = Rc::clone(&self.service);

        async move {
            let user_id = uuid::Uuid::parse_str(user_id.as_str()).expect("UUID parsing failed");
            // TODO: Check the database
            // let result = cloned_app_state
            //     .db_client
            //     .get_user(Some(user_id.clone()), None, None)
            //     .await
            //     .map_err(|e| ErrorInternalServerError(HttpError::server_error(e.to_string())))?;
            let result = Option::Some(User {
                id: 1,
                username: "1".into(),
                email: "1".into(),
                password: "1".into(),
                activated: false,
                privilege: 0,
                since: 0,
            });

            let user = result.ok_or(ErrorUnauthorized(ErrorResponse {
                status: "fail".to_string(),
                message: ErrorMessage::UserNoLongerExist.to_string(),
            }))?;

            // Check if user's role matches the required role
            if &user.privilege >= &allowed_roles {
                req.extensions_mut().insert::<User>(user);
                let res = srv.call(req).await?;
                Ok(res)
            } else {
                let json_error = ErrorResponse {
                    status: "fail".to_string(),
                    message: ErrorMessage::PermissionDenied.to_string(),
                };
                Err(ErrorForbidden(json_error))
            }
        }
        .boxed_local()
    }
}
