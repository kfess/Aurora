use actix_web::{self, web, HttpResponse};

use crate::utils::jwt;
use crate::{domain::vo::providers::AuthProvider, service::auth::Authenticate};
use std::sync::Arc;

pub struct AuthController<U: Authenticate> {
    usecase: Arc<U>,
}

impl<U: Authenticate> AuthController<U> {
    pub fn new(usecase: Arc<U>) -> Self {
        Self { usecase }
    }

    pub async fn get_authenticate_url(&self, query: web::Query<AuthProviderQuery>) -> HttpResponse {
        match AuthProvider::try_from(&query.provider) {
            Ok(provider) => match self.usecase.get_authenticate_url(&provider).await {
                Ok(url) => HttpResponse::Found()
                    .append_header(("Location", url))
                    .finish(),
                Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
            },
            Err(_) => HttpResponse::BadRequest().body("Invalid provider is specified"),
        }
    }

    pub async fn handle_callback(&self, query: web::Query<AuthCallbackQuery>) -> HttpResponse {
        match AuthProvider::try_from(&query.provider) {
            Ok(provider) => match self.usecase.handle_callback(&provider, &query.code).await {
                Ok(user) => {
                    let jwt = jwt::encode_jwt("", &user.id).ok();
                    // todo: fix jwt secret and jwt key
                    let cookie = actix_web::cookie::Cookie::build("token", jwt.unwrap())
                        .http_only(true)
                        // .secure(true)
                        .path("/")
                        .finish();

                    let response = HttpResponse::Ok().cookie(cookie).json(user);

                    response
                }
                Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
            },
            Err(_) => HttpResponse::BadRequest().body("Invalid provider is specified"),
        }
    }
}

#[derive(serde::Deserialize)]
pub struct AuthProviderQuery {
    provider: String,
}

#[derive(serde::Deserialize)]
pub struct AuthCallbackQuery {
    provider: String,
    code: String,
}
