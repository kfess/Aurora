//! UseCase for Submissions
//!
//! This module handles use cases related to submissions.
//! Its primary functions are to fetch recent submissions and submissions by user.
//!
//! Submission is only available for AtCoder, Codeforces, and AOJ.

use crate::infra::api::{
    aoj::api_client::AojAPIClient, atcoder::api_client::AtcoderAPIClient,
    cf::api_client::CFAPIClient,
};

pub struct FetchSubmissionUsecase<C>
where
    C: AtcoderAPIClient + CFAPIClient + AojAPIClient,
{
    api_client: C,
}

impl<C> FetchSubmissionUsecase<C>
where
    C: AtcoderAPIClient + CFAPIClient + AojAPIClient,
{
    pub fn new(api_client: C) -> Self {
        Self { api_client }
    }

    pub async fn fetch_atcoder_recent_subs(&self) {
        log::info!("AtCoder: fetch recent submissions");

        let submissions = self
            .api_client
            .get_atcoder_recent_submissions()
            .await
            .unwrap();
        for submission in submissions {
            println!("{:?}", submission);
        }
    }

    pub async fn fetch_atcoder_user_subs(&self, user: &str, from_second: Option<u64>) {
        log::info!("AtCoder: fetch user submissions");

        let submissions = self
            .api_client
            .get_atcoder_user_submissions(user, from_second)
            .await
            .unwrap();
        for submission in submissions {
            println!("{:?}", submission);
        }
    }

    pub async fn fetch_cf_recent_subs(&self) {
        log::info!("Codeforces: fetch recent submissions");

        let submissions = self.api_client.get_cf_recent_submissions().await.unwrap();
        for submission in submissions {
            println!("{:?}", submission);
        }
    }

    pub async fn fetch_cf_user_subs(&self, user: &str, from: Option<u32>, count: Option<u32>) {
        log::info!("Codeforces: fetch user submissions");

        let submissions = self
            .api_client
            .get_cf_user_submissions(user, from, count)
            .await
            .unwrap();
        for submission in submissions {
            println!("{:?}", submission);
        }
    }

    pub async fn fetch_aoj_recent_subs(&self) {
        log::info!("AOJ: fetch user submissions");

        let submissions = self.api_client.get_cf_recent_submissions().await.unwrap();
        for submission in submissions {
            println!("{:?}", submission);
        }
    }

    pub async fn fetch_aoj_user_subs(&self, user: &str, page: Option<u32>, size: Option<u32>) {
        log::info!("AOJ: fetch user submissions");

        let submissions = self
            .api_client
            .get_aoj_user_submissions(user, page, size)
            .await
            .unwrap();
        for submission in submissions {
            println!("{:?}", submission);
        }
    }
}
