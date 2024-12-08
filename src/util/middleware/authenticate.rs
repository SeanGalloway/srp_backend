use std::future::{ready, Ready};

use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, Error, HttpMessage};
use actix_web::error::ErrorUnauthorized;
use actix_web::http::header::HeaderName;
use futures_util::future::LocalBoxFuture;
use futures_util::FutureExt;
use crate::core::auth_service;

pub struct Authenticate;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for Authenticate
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthenticateMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticateMiddleware { service }))
    }
}

pub struct AuthenticateMiddleware<S> {
    service: S
}

impl<S> AuthenticateMiddleware<S> {
    fn get_token(req: &ServiceRequest) -> Option<String> {
        println!("{}", req.headers().len());
        match req.headers().get(HeaderName::from_static("authorization"))?.to_str() {
            Ok(token) => {
                if !token.starts_with("Bearer ") {
                    None
                } else {
                    Some(token.replace("Bearer ", ""))
                }
            },
            Err(_) => None
        }
    }
}

impl<S, B> Service<ServiceRequest> for AuthenticateMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        match Self::get_token(&req) {
            Some(token) => {
                match auth_service::validate_token(&token) {
                    Ok(claims) => {
                        req.extensions_mut().insert(claims);
                        let fut = self.service.call(req);
                        Box::pin(async move {
                            let res = fut.await?;
                            Ok(res)
                        })
                    },
                    Err(_) => {
                        Box::pin(async move { Err(ErrorUnauthorized("Unauthorized")) }.boxed_local())
                    }
                }
            },
            None => Box::pin(async move { Err(ErrorUnauthorized("Unauthorized")) }.boxed_local())
        }
    }
}