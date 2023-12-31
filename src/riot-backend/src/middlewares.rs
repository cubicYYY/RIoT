use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::error::{ErrorForbidden, ErrorUnauthorized};

use actix_web::{http, web, FromRequest, HttpMessage};

use futures_util::future::{ready, LocalBoxFuture, Ready};
use futures_util::FutureExt;
use log::error;
use serde::Deserialize;
use std::rc::Rc;
use std::task::{Context, Poll};

use crate::errors::{ErrorMessage, ErrorResponse, HttpError};
use crate::models::{User, UserPrivilege};
use crate::utils::jwt::parse_token;
use crate::AppState;

pub struct AuthenticatedUser(User);

impl FromRequest for AuthenticatedUser {
    type Error = HttpError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let value = req.extensions().get::<User>().cloned();
        let result = match value {
            Some(user) => Ok(AuthenticatedUser(user)),
            None => Err(HttpError::permission_denied(
                "Authentication Error: You are not logged-in an account with enough privilege to perform this.",
            )),
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
    pub priv_needed: Rc<u32>,
}

impl RequireAuth {
    pub fn with_priv_level(priv_level: u32) -> Self {
        RequireAuth {
            priv_needed: Rc::new(priv_level),
        }
    }
    pub fn no_auth() -> Self {
        RequireAuth {
            priv_needed: Rc::new(UserPrivilege::Everyone as u32),
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
            least_priv: self.priv_needed.clone(),
        }))
    }
}

pub struct AuthMiddleware<S> {
    service: Rc<S>,
    least_priv: Rc<u32>,
}
#[derive(Debug, Deserialize)]
pub struct ApiKey {
    pub api_key: String,
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
        // Check api key first
        let app_state = req.app_data::<web::Data<AppState>>().unwrap();
        let apikey = req
            .app_data::<web::Query<ApiKey>>()
            .map(|key| key.api_key.clone());
        let cloned_app_state = app_state.clone();
        if let Some(key) = apikey {
            let srv = Rc::clone(&self.service);
            let least_priv = self.least_priv.clone();
            return async move {
                let result = cloned_app_state.db.get_user_by_api_key(key.as_str()).await;
                let user = result.map_err(|_e| {
                    ErrorUnauthorized(ErrorResponse {
                        status: "fail".to_string(),
                        message: ErrorMessage::UserNotActivated.to_string(),
                    })
                })?;
                if &user.privilege >= &least_priv {
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
            .boxed_local();
        }

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
            // Just return it so that Option<AuthenticatedUser> will work
            let srv = Rc::clone(&self.service);
            return async move { srv.call(req).await }.boxed_local();
        }

        let user_id = match parse_token(token.unwrap(), app_state.env.jwt.secret.as_bytes()) {
            Ok(id) => id,
            Err(e) => {
                error!("{}", e);
                return Box::pin(ready(Err(ErrorUnauthorized(ErrorResponse {
                    status: "fail".to_string(),
                    message: ErrorMessage::InvalidToken.to_string(),
                }))));
            }
        };

        // Now the user identity is verified, start authentication checking
        let cloned_app_state = app_state.clone();
        let least_priv = self.least_priv.clone();
        let srv = Rc::clone(&self.service);

        async move {
            let result = cloned_app_state
                .db
                .get_user_by_id(user_id.parse::<u64>().unwrap())
                .await;

            let user = result.map_err(|_e| {
                ErrorUnauthorized(ErrorResponse {
                    status: "fail".to_string(),
                    message: ErrorMessage::UserNotActivated.to_string(),
                })
            })?;

            if &user.privilege >= &least_priv {
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
