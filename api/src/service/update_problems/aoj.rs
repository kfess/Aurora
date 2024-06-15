use anyhow::{Context, Result};
use std::sync::Arc;

use crate::infra::{
    api::aoj::api_client::AojAPIClient,
    repository::{contest::ContestRepository, problem::ProblemRepository},
};

pub struct UpdateAojUsecase<C, R>
where
    C: AojAPIClient,
    R: ProblemRepository + ContestRepository,
{
    api_client: Arc<C>,
    repository: Arc<R>,
}

impl<C, R> UpdateAojUsecase<C, R>
where
    C: AojAPIClient,
    R: ProblemRepository + ContestRepository,
{
    pub fn new(api_client: Arc<C>, repository: Arc<R>) -> Self {
        return Self {
            api_client,
            repository,
        };
    }

    pub async fn fetch_and_update(&self) -> Result<()> {
        let (problems, contests) = self
            .api_client
            .get_aoj_problems_and_contests()
            .await
            .unwrap();

        self.repository
            .update_problems(&problems)
            .await
            .with_context(|| "Failed to update Aizu Online Judge problems")
            .unwrap();

        self.repository
            .update_contests(&contests)
            .await
            .with_context(|| "Failed to update Aizu Online Judge contests")
            .unwrap();

        Ok(())
    }
}
