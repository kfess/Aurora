use std::{
    future::{ready, Future, Ready},
    pin::Pin,
};

use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::Error;

use crate::config::CONFIG;
use crate::utils::cookie;
use crate::utils::jwt;

const AUTHORIZED_ROUTES: [&str; 1] = ["/api/auth/user"];

type LocalBoxFuture<T> = Pin<Box<dyn Future<Output = T> + 'static>>;

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService { service }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        if AUTHORIZED_ROUTES.iter().any(|&route| route == req.path()) {
            match cookie::get_cookie_value(&req.request(), &CONFIG.jwt_cookie_key) {
                Some(jwt) => {
                    if jwt::decode_jwt(&CONFIG.jwt_secret, &jwt).is_ok() {
                        return Box::pin(self.service.call(req));
                    } else {
                        return Box::pin(async {
                            Err(actix_web::error::ErrorUnauthorized("Unauthorized Request"))
                        });
                    }
                }
                None => {
                    return Box::pin(async {
                        Err(actix_web::error::ErrorUnauthorized("Unauthorized Request"))
                    });
                }
            }
        } else {
            Box::pin(self.service.call(req))
        }
    }
}
