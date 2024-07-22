use std::sync::Arc;

use actix_web::{web, HttpResponse};
use serde::Deserialize;

use crate::domain::vo::platform::Platform;
use crate::infra::repository::problem::Condition;
use crate::service::problem::FetchProblem;

#[derive(Deserialize)]
struct QueryParams {
    algo_id: Option<String>,
    technical_tag_id: Option<String>,
    page: Option<String>,
    page_size: Option<String>,
    from_difficulty: Option<String>,
    to_difficulty: Option<String>,
}

pub struct ProblemController<U: FetchProblem> {
    usecase: Arc<U>,
}

impl<U: FetchProblem> ProblemController<U> {
    pub fn new(usecase: Arc<U>) -> Self {
        Self { usecase }
    }

    pub async fn problems(
        &self,
        path: web::Path<String>,
        query: web::Query<serde_json::Value>,
    ) -> HttpResponse {
        if let Ok(params) = serde_json::from_value::<QueryParams>(query.into_inner()) {
            let condition = Condition {
                platform: Some(path.as_str()),
                algo_id: params.algo_id.as_deref(),
                technical_tag_id: params.technical_tag_id.as_deref(),
                page: params.page.as_deref().and_then(|s| s.parse().ok()),
                page_size: params.page_size.as_deref().and_then(|s| s.parse().ok()),
                from_difficulty: params
                    .from_difficulty
                    .as_deref()
                    .and_then(|s| s.parse().ok()),
                to_difficulty: params.to_difficulty.as_deref().and_then(|s| s.parse().ok()),
            };

            match self.usecase.fetch_problems(&condition).await {
                Ok(problems) => HttpResponse::Ok().json(problems),
                Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
            }
        } else {
            HttpResponse::BadRequest().body("Invalid query")
        }
    }
}
