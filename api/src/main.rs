use dotenv::dotenv;
use infra::api::{yoj::api_client::YOJAPIClient, yukicoder::api_client::YukicoderAPIClient};

mod domain;
mod infra;
mod service;
mod utils;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let api_client = YukicoderAPIClient::new();
    let usecase = service::update_problems::yukicoder::UpdateYukicoderUsecase::new(api_client);
    usecase.execute().await;

    let api_client = YOJAPIClient::new();
    let usecase: service::update_problems::yoj::UpdateYOJUsecase<_> =
        service::update_problems::yoj::UpdateYOJUsecase::new(api_client);
    usecase.execute().await;

    Ok(())
}
