use std::sync::Arc;

use actix_web::{web, HttpResponse};
use serde::Deserialize;

use crate::{domain::vo::platform::Platform, service::submission::FetchSubmission};

pub struct SubmissionController<U: FetchSubmission> {
    usecase: Arc<U>,
}

impl<U: FetchSubmission> SubmissionController<U> {
    pub fn new(usecase: Arc<U>) -> Self {
        Self { usecase }
    }

    pub async fn recent_submissions(&self, platform: web::Path<String>) -> HttpResponse {
        let platform = Platform::from(platform.as_str());

        match self.usecase.fetch_recent_submissions(&platform).await {
            Ok(submissions) => HttpResponse::Ok().json(submissions),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    }

    pub async fn user_submissions(
        &self,
        path: web::Path<(String, String)>,
        query: web::Query<String>, // query: web::Query<serde_json::Value>,
    ) -> HttpResponse {
        HttpResponse::Ok().finish()
    }
}
