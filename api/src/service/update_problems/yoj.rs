use anyhow::Result;
use std::sync::Arc;

use crate::infra::{api::yoj::api_client::YOJAPIClient, repository::problem::ProblemRepository};

pub struct UpdateYOJUsecase<C, R>
where
    C: YOJAPIClient,
    R: ProblemRepository,
{
    api_client: Arc<C>,
    repository: Arc<R>,
}

impl<C, R> UpdateYOJUsecase<C, R>
where
    C: YOJAPIClient,
    R: ProblemRepository,
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

        for contest in contests.iter() {
            println!("{:?}", contest);
        }

        self.repository.update_problems(&problems).await.unwrap();

        Ok(())
    }
}
