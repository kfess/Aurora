use std::sync::Arc;

use actix_web::HttpResponse;

use crate::infra::repository::problem::Condition;
use crate::service::problem::FetchProblem;

pub struct ProblemController<U: FetchProblem> {
    usecase: Arc<U>,
}

impl<U: FetchProblem> ProblemController<U> {
    pub fn new(usecase: Arc<U>) -> Self {
        Self { usecase }
    }

    pub async fn problems(&self) -> HttpResponse {
        let condition = Condition::default();
        match self.usecase.fetch_problems(&condition).await {
            Ok(problems) => HttpResponse::Ok().json(problems),
            Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
        }
    }
}
