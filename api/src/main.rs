use dotenv::dotenv;
use infra::api::factory::APIClientFactory;
use service::update_problems::yuki::UpdateYukicoderUsecase;

mod domain;
mod infra;
mod service;
mod utils;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let api_factory = APIClientFactory::new()
        .with_yuki_client()
        .with_atcoder_client()
        .with_cf_client()
        .with_aoj_client()
        .with_yoj_client();
    let usecase = UpdateYukicoderUsecase::new(api_factory);
    usecase.execute(true).await;

    Ok(())
}
