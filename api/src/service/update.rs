use anyhow::{Context, Result};
use std::sync::Arc;

use crate::{
    domain::vo::platform::Platform,
    infra::{
        api::{
            aoj::api_client::AojAPIClient, atcoder::api_client::AtcoderAPIClient,
            cf::api_client::CFAPIClient, yoj::api_client::YOJAPIClient,
            yuki::api_client::YukicoderAPIClient,
        },
        repository::{contest::ContestRepository, problem::ProblemRepository},
    },
};

pub struct UpdateUsecase<C, R>
where
    C: AtcoderAPIClient + CFAPIClient + YukicoderAPIClient + AojAPIClient + YOJAPIClient,
    R: ProblemRepository + ContestRepository,
{
    api_client: Arc<C>,
    repository: Arc<R>,
}

impl<C, R> UpdateUsecase<C, R>
where
    C: AtcoderAPIClient + CFAPIClient + YukicoderAPIClient + AojAPIClient + YOJAPIClient,
    R: ProblemRepository + ContestRepository,
{
    pub fn new(api_client: Arc<C>, repository: Arc<R>) -> Self {
        return Self {
            api_client,
            repository,
        };
    }

    pub async fn fetch_and_update(&self, platform: &Platform) -> Result<()> {
        let (problems, contests) = match platform {
            Platform::Atcoder => self
                .api_client
                .get_atcoder_problems_and_contests()
                .await
                .unwrap(),
            Platform::Codeforces => self
                .api_client
                .get_cf_problems_and_contests()
                .await
                .unwrap(),
            Platform::Yukicoder => self
                .api_client
                .get_yuki_problems_and_contests()
                .await
                .unwrap(),
            Platform::Aoj => self
                .api_client
                .get_aoj_problems_and_contests()
                .await
                .unwrap(),
            Platform::YOJ => self
                .api_client
                .get_yoj_problems_and_contests()
                .await
                .unwrap(),
        };

        self.repository
            .update_problems(&problems)
            .await
            .with_context(|| "Failed to update problems")
            .unwrap();

        self.repository
            .update_contests(&contests)
            .await
            .with_context(|| "Failed to update contests")
            .unwrap();

        Ok(())
    }
}
