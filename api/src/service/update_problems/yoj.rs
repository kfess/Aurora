use anyhow::{Context, Result};
use std::sync::Arc;

use crate::infra::{
    api::yoj::api_client::YOJAPIClient,
    repository::{contest::ContestRepository, problem::ProblemRepository},
};

pub struct UpdateYOJUsecase<C, R>
where
    C: YOJAPIClient,
    R: ProblemRepository + ContestRepository,
{
    api_client: Arc<C>,
    repository: Arc<R>,
}

impl<C, R> UpdateYOJUsecase<C, R>
where
    C: YOJAPIClient,
    R: ProblemRepository + ContestRepository,
{
    pub fn new(api_client: Arc<C>, repository: Arc<R>) -> Self {
        Self {
            api_client,
            repository,
        }
    }

    pub async fn fetch_and_update(&self) -> Result<()> {
        let (problems, contests) = self
            .api_client
            .get_yoj_problems_and_contests()
            .await
            .unwrap();

        self.repository
            .update_problems(&problems)
            .await
            .with_context(|| format!("Failed to update yosupo online judge problems"))
            .unwrap();

        self.repository
            .update_contests(&contests)
            .await
            .with_context(|| format!("Failed to update yosupo online judge contests"))
            .unwrap();

        Ok(())
    }
}
