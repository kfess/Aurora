use std::env;

use dotenv::dotenv;
use infra::{
    api::factory::APIClientFactory,
    repository::{initialize_pool::initialize_pool, technical_tag::TechnicalTagRepositoryTrait},
};
use service::{
    submission::FetchSubmissionUsecase,
    update_problems::{atcoder::UpdateAtcoderUsecase, yuki::UpdateYukicoderUsecase},
};

mod domain;
mod infra;
mod service;
mod utils;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let api_factory = APIClientFactory::new()
        .with_atcoder()
        .with_cf()
        .with_yuki()
        .with_aoj()
        .with_yoj();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set.");
    let pool = initialize_pool(db_url)
        .await
        .expect("Failed to initialize the connection pool");

    let usecase = UpdateAtcoderUsecase::new(api_factory, pool);
    usecase.fetch_and_update().await;

    // let usecase = FetchSubmissionUsecase::new(api_factory);

    // AtCoder
    // usecase.fetch_atcoder_recent_subs().await;
    // usecase.fetch_atcoder_user_subs("kenkoooo", Some(0)).await;

    // // Codeforces
    // usecase.fetch_cf_recent_subs().await;
    // usecase
    //     .fetch_cf_user_subs("tourist", Some(1), Some(10))
    //     .await;

    // // AOJ
    // usecase.fetch_aoj_recent_subs().await;
    // usecase.fetch_aoj_user_subs("eidensuke", None, None).await;

    // pool.get_tags(None).await?;

    Ok(())
}
