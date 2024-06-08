use anyhow::Context;
use anyhow::Ok;
use anyhow::Result;
use std::sync::Arc;

use crate::infra::{
    api::atcoder::api_client::AtcoderAPIClient, repository::problem::ProblemRepository,
};

pub struct UpdateAtcoderUsecase<C: AtcoderAPIClient, R: ProblemRepository> {
    api_client: Arc<C>,
    repository: Arc<R>,
}

impl<C: AtcoderAPIClient, R: ProblemRepository> UpdateAtcoderUsecase<C, R> {
    pub fn new(api_client: Arc<C>, repository: Arc<R>) -> Self {
        Self {
            api_client,
            repository,
        }
    }

    /// Fetch Atocoder problems and contests and update the database.
    /// This method is called periodically by the scheduler.
    pub async fn fetch_and_update(&self) -> Result<()> {
        let (problems, _contests) = self
            .api_client
            .get_atcoder_problems_and_contests()
            .await
            .unwrap();

        self.repository
            .update_problems(&problems)
            .await
            .with_context(|| "Failed to update AtCoder problems")
            .unwrap();

        Ok(())
    }
}
