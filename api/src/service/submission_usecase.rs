//! Use Case for Submissions
//!
//! This module handles use cases related to submissions.
//! Its primary functions are to fetch recent submissions and submissions by user.
//!

use crate::infra::api::factory::APIClientFactoryTrait;

pub struct FetchSubmissionUsecase<T: APIClientFactoryTrait> {
    api_client_factory: T,
}

impl<T: APIClientFactoryTrait> FetchSubmissionUsecase<T> {
    pub fn new(api_client_factory: T) -> Self {
        Self { api_client_factory }
    }

    pub async fn fetch_atcoder_recent_subs(&self) {
        log::info!("AtCoder: fetch recent submissions");

        let atcoder_client = self.api_client_factory.get_atcoder_client().await.unwrap();
        let submissions = atcoder_client.get_recent_submissions().await.unwrap();
        for submission in submissions {
            println!("{:?}", submission);
        }
    }

    pub async fn fetch_atcoder_user_subs(&self, user: &str, from_second: Option<u64>) {
        log::info!("AtCoder: fetch user submissions");

        let atcoder_client = self.api_client_factory.get_atcoder_client().await.unwrap();
        let submissions = atcoder_client
            .get_user_submissions(user, from_second)
            .await
            .unwrap();
        for submission in submissions {
            println!("{:?}", submission);
        }
    }
}
