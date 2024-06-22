use std::sync::Arc;

use actix_web::HttpResponse;

use crate::infra::repository::contest::Condition;
use crate::service::contest::FetchContest;

pub struct ContestController<U: FetchContest> {
    usecase: Arc<U>,
}

impl<U: FetchContest> ContestController<U> {
    pub fn new(usecase: Arc<U>) -> Self {
        Self { usecase }
    }

    pub async fn contests(&self) -> HttpResponse {
        let condition = Condition::default();
        match self.usecase.fetch_contests(&condition).await {
            Ok(contests) => HttpResponse::Ok().json(contests),
            Err(e) => {
                println!("{:?}", e);
                return HttpResponse::InternalServerError().body("Internal Server Error");
            }
        }
    }
}
