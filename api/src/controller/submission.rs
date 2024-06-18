use std::sync::Arc;

use actix_web::{web, HttpResponse};
use serde::Deserialize;

use crate::{
    domain::vo::platform::Platform,
    service::submission::{FetchSubmission, PageCondition},
};

#[derive(Deserialize)]
struct AtcoderQueryParams {
    from_second: Option<String>,
}

#[derive(Deserialize)]
struct OtherQueryParams {
    page: Option<String>,
    size: Option<String>,
}

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
        query: web::Query<serde_json::Value>,
    ) -> HttpResponse {
        let platform = Platform::from(path.0.as_str());
        let user_id = path.1.as_str();

        let submissions = match platform {
            Platform::Atcoder => {
                if let Ok(params) = serde_json::from_value::<AtcoderQueryParams>(query.into_inner())
                {
                    self.usecase
                        .fetch_user_submissions(
                            &platform,
                            &PageCondition::Atcoder {
                                user: user_id,
                                from_second: params.from_second.map(|s| s.parse().unwrap()),
                            },
                        )
                        .await
                } else {
                    return HttpResponse::BadRequest().finish();
                }
            }
            Platform::Codeforces | Platform::Aoj => {
                if let Ok(params) = serde_json::from_value::<OtherQueryParams>(query.into_inner()) {
                    self.usecase
                        .fetch_user_submissions(
                            &platform,
                            &PageCondition::Other {
                                user: user_id,
                                page: params.page.map(|s| s.parse().unwrap()),
                                size: params.size.map(|s| s.parse().unwrap()),
                            },
                        )
                        .await
                } else {
                    return HttpResponse::BadRequest().finish();
                }
            }
            _ => {
                unimplemented!("Unsupported platform");
            }
        };

        match submissions {
            Ok(submissions) => HttpResponse::Ok().json(submissions),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    }
}
