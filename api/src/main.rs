use dotenv::dotenv;
use infra::api::{yoj::api_client::YOJAPIClient, yuki::api_client::YukicoderAPIClient};

mod domain;
mod infra;
mod service;
mod utils;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let api_client = YukicoderAPIClient::new();
    let usecase = service::update_problems::yuki::UpdateYukicoderUsecase::new(api_client);
    usecase.update_recent().await;

    // let api_client = YOJAPIClient::new();
    // let usecase: service::update_problems::yoj::UpdateYOJUsecase<_> =
    //     service::update_problems::yoj::UpdateYOJUsecase::new(api_client);
    // usecase.execute().await;

    Ok(())
}
