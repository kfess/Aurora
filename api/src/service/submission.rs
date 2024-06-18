//! UseCase for Submissions
//!
//! This module handles use cases related to submissions.
//! Its primary functions are to fetch recent submissions and submissions by user.
//!
//! Submission is only available for AtCoder, Codeforces, and AOJ.

use anyhow::Result;

use crate::{
    domain::{submission::Submission, vo::platform::Platform},
    infra::api::{
        aoj::api_client::AojAPIClient, atcoder::api_client::AtcoderAPIClient,
        cf::api_client::CFAPIClient,
    },
};

pub enum PageCondition<'a> {
    Atcoder {
        user: &'a str,
        from_second: Option<u64>,
    },
    Other {
        user: &'a str,
        page: Option<u32>,
        size: Option<u32>,
    },
}

pub struct FetchSubmissionUsecase<C>
where
    C: AtcoderAPIClient + CFAPIClient + AojAPIClient,
{
    api_client: C,
}

#[trait_variant::make]
pub trait FetchSubmission {
    async fn fetch_recent_submissions(&self, platform: &Platform) -> Result<Vec<Submission>>;
    async fn fetch_user_submissions(
        &self,
        platform: &Platform,
        condition: &PageCondition,
    ) -> Result<Vec<Submission>>;
}

impl<C> FetchSubmissionUsecase<C>
where
    C: AtcoderAPIClient + CFAPIClient + AojAPIClient,
{
    pub fn new(api_client: C) -> Self {
        Self { api_client }
    }
}

impl<C> FetchSubmission for FetchSubmissionUsecase<C>
where
    C: AtcoderAPIClient + CFAPIClient + AojAPIClient,
{
    async fn fetch_recent_submissions(&self, platform: &Platform) -> Result<Vec<Submission>> {
        let submissions = match platform {
            Platform::Atcoder => self
                .api_client
                .get_atcoder_recent_submissions()
                .await
                .unwrap(),
            Platform::Codeforces => self.api_client.get_cf_recent_submissions().await.unwrap(),
            Platform::Aoj => self.api_client.get_aoj_recent_submissions().await.unwrap(),
            _ => {
                unimplemented!("This platform is not currently supported.");
            }
        };

        Ok(submissions)
    }

    async fn fetch_user_submissions(
        &self,
        platform: &Platform,
        condition: &PageCondition<'_>,
    ) -> Result<Vec<Submission>> {
        let submissions = match platform {
            Platform::Atcoder => {
                let (user, from_second) = match condition {
                    PageCondition::Atcoder { user, from_second } => (user, from_second),
                    _ => unreachable!(),
                };
                self.api_client
                    .get_atcoder_user_submissions(user, *from_second)
                    .await
                    .unwrap()
            }

            Platform::Codeforces => {
                let (user, page, size) = match condition {
                    PageCondition::Other { user, page, size } => (user, page, size),
                    _ => unreachable!(),
                };

                self.api_client
                    .get_cf_user_submissions(user, *page, *size)
                    .await
                    .unwrap()
            }

            Platform::Aoj => {
                let (user, page, size) = match condition {
                    PageCondition::Other { user, page, size } => (user, page, size),
                    _ => unreachable!(),
                };
                self.api_client
                    .get_aoj_user_submissions(user, *page, *size)
                    .await
                    .unwrap()
            }
            _ => {
                unimplemented!("This platform is not currently supported.");
            }
        };

        Ok(submissions)
    }
}
