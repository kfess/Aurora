use std::sync::Arc;

use crate::infra::{
    api::cf::api_client::CFAPIClient, repository::contest::ContestRepository,
    repository::problem::ProblemRepository,
};
use anyhow::{Context, Result};

pub struct UpdateCodeforcesUsecase<C, R>
where
    C: CFAPIClient,
    R: ProblemRepository + ContestRepository,
{
    api_client: Arc<C>,
    repository: Arc<R>,
}

impl<C, R> UpdateCodeforcesUsecase<C, R>
where
    C: CFAPIClient,
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
            .get_cf_problems_and_contests()
            .await
            .with_context(|| "Failed to fetch Codeforces problems")
            .unwrap();

        self.repository
            .update_problems(&problems)
            .await
            .with_context(|| "Failed to update Codeforces problems")
            .unwrap();

        self.repository
            .update_contests(&contests)
            .await
            .with_context(|| "Failed to update Codeforces contests")
            .unwrap();

        Ok(())
    }
}
