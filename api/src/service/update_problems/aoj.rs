use anyhow::Result;
use std::sync::Arc;

use crate::infra::{api::aoj::api_client::AojAPIClient, repository::problem::ProblemRepository};

pub struct UpdateAojUsecase<C, R>
where
    C: AojAPIClient,
    R: ProblemRepository,
{
    api_client: Arc<C>,
    repository: Arc<R>,
}

impl<C, R> UpdateAojUsecase<C, R>
where
    C: AojAPIClient,
    R: ProblemRepository,
{
    pub fn new(api_client: Arc<C>, repository: Arc<R>) -> Self {
        return Self {
            api_client,
            repository,
        };
    }

    pub async fn fetch_and_update(&self) -> Result<()> {
        log::info!("Aizu Online Judge: update problems and contests");

        let (problems, _contests) = self
            .api_client
            .get_aoj_problems_and_contests()
            .await
            .unwrap();
        self.repository.update_problems(&problems).await.unwrap();

        Ok(())
    }
}
