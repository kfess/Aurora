use dotenv::dotenv;
use infrastracture::api::{
    yosupo_online_judge::api_client::{IYosupoOnlineJudgeAPIClient, YosupoOnlineJudgeAPIClient},
    yukicoder::api_client::{IYukicoderAPIClient, YukicoderAPIClient},
};

mod domain;
mod infrastracture;
mod service;
mod utils;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    // let api_client = YukicoderAPIClient::new();
    // let usecase =
    //     service::update_problems::update_yukicoder_problems::UpdateYukicoderProblemUsecase::new(
    //         api_client,
    //     );
    // usecase.execute().await;

    let api_client = YosupoOnlineJudgeAPIClient::new();
    let usecase =
            service::update_problems::update_yosupo_online_judge_problems::UpdateYosupoOnlineJudgeProblemUsecase::new(
                api_client,
            );
    usecase.execute().await;

    Ok(())
}
