use dotenv::dotenv;
use infra::api::factory::APIClientFactory;
use service::{
    submission_usecase::FetchSubmissionUsecase,
    update_problems::{atcoder::FetchAtcoderUsecase, yuki::UpdateYukicoderUsecase},
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
    let usecase = FetchSubmissionUsecase::new(api_factory);
    usecase.fetch_atcoder_recent_subs().await;

    Ok(())
}
