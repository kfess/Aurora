use crate::infra::{
    api::yuki::api_client::YukicoderAPIClient,
    repository::{contest::ContestRepository, problem::ProblemRepository},
};
use anyhow::{Context, Result};
use std::sync::Arc;

pub struct UpdateYukicoderUsecase<C, R>
where
    C: YukicoderAPIClient,
    R: ProblemRepository + ContestRepository,
{
    api_client: Arc<C>,
    repository: Arc<R>,
}

impl<C, R> UpdateYukicoderUsecase<C, R>
where
    C: YukicoderAPIClient,
    R: ProblemRepository + ContestRepository,
{
    pub fn new(api_client: Arc<C>, repository: Arc<R>) -> Self {
        Self {
            api_client,
            repository,
        }
    }

    pub async fn fetch_and_update(&self, is_recent: bool) -> Result<()> {
        let (problems, contests) = self
            .api_client
            .get_yuki_problems_and_contests()
            .await
            .unwrap();

        for contest in contests.iter() {
            println!("{:?}", contest);
        }

        self.repository
            .update_problems(&problems)
            .await
            .with_context(|| format!("Failed to update Yukicoder problems"))
            .unwrap();

        Ok(())
    }
}
