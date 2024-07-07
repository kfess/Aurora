use actix_web::{self, web, HttpRequest, HttpResponse};

use crate::utils::jwt;
use crate::{domain::vo::providers::AuthProvider, service::auth::Authenticate};
use std::env;
use std::sync::Arc;

pub struct AuthController<U: Authenticate> {
    usecase: Arc<U>,
}

impl<U: Authenticate> AuthController<U> {
    pub fn new(usecase: Arc<U>) -> Self {
        Self { usecase }
    }

    pub async fn get_authenticate_url(&self, path: web::Path<String>) -> HttpResponse {
        match AuthProvider::try_from(path.as_str()) {
            Ok(provider) => match self.usecase.get_authenticate_url(&provider).await {
                Ok(url) => HttpResponse::Found()
                    .append_header(("Location", url))
                    .finish(),
                Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
            },
            Err(_) => HttpResponse::BadRequest().body("Invalid provider is specified"),
        }
    }

    pub async fn handle_callback(
        &self,
        path: web::Path<String>,
        query: web::Query<AuthCallbackQuery>,
    ) -> HttpResponse {
        match AuthProvider::try_from(path.as_str()) {
            Ok(provider) => match self.usecase.handle_callback(&provider, &query.code).await {
                Ok(user) => {
                    let secret = env::var("JWT_SECRET").expect("JWT is not set.");
                    let cookie_key =
                        env::var("JWT_COOKIE_KEY").expect("JWT Cookie Key is not set.");
                    let jwt = jwt::encode_jwt(&secret, &user.id).ok();
                    let cookie = actix_web::cookie::Cookie::build(cookie_key, jwt.unwrap())
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

    pub async fn user_info(&self, req: HttpRequest, path: web::Path<String>) -> HttpResponse {
        let user_id = path.into_inner();
        let user_info = self.usecase.get_user_info(&user_id).await.unwrap();
        HttpResponse::Ok().json(user_info)
    }
}

#[derive(serde::Deserialize)]
pub struct AuthCallbackQuery {
    code: String,
}
