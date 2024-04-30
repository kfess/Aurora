use dotenv::dotenv;
use infrastracture::api::{
    aoj::api_client::{AojAPIClient, IAojAPIClient},
    codeforces::api_client::{CodeforcesAPIClient, ICodeforcesAPICLient},
    yosupo_online_judge::api_client::{IYosupoOnlineJudgeAPIClient, YosupoOnlineJudgeAPIClient},
    yukicoder::api_client::{IYukicoderAPIClient, YukicoderAPIClient},
};

mod domain;
mod infrastracture;
mod utils;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let yukicoder_api_client = YukicoderAPIClient::new();
    match yukicoder_api_client.get_problems().await {
        Ok(problems) => {
            println!("Problem Id: {}", problems[0].title);
        }
        Err(e) => {
            eprintln!("Error fetching problems: {}", e);
        }
    }

    match yukicoder_api_client.get_past_contests().await {
        Ok(contests) => {
            println!(
                "Contest Name: {}, Contest Id {}",
                contests[0].name, contests[0].id
            );
        }
        Err(e) => {
            eprintln!("Error fetching contests: {}", e);
        }
    }

    let aoj_api_client = AojAPIClient::new();
    match aoj_api_client.get_problems().await {
        Ok(problems) => {
            println!("Problem Name: {}", problems[0].name);
        }
        Err(e) => {
            eprintln!("Error fetching problems: {}", e);
        }
    }

    let yosupo_online_judge_api_client = YosupoOnlineJudgeAPIClient::new();
    match yosupo_online_judge_api_client.get_problems().await {
        Ok(problems) => {
            println!("Problem Name: {}", problems[0].name);
        }
        Err(e) => {
            eprintln!("Error fetching problems: {}", e);
        }
    }

    let codeforces_api_client = CodeforcesAPIClient::new();
    match codeforces_api_client.get_problems().await {
        Ok(problems) => {
            println!("Problem Name: {}", problems[0].name);
        }
        Err(e) => {
            eprintln!("Error fetching problems: {}", e);
        }
    }

    match codeforces_api_client.get_contests().await {
        Ok(contests) => {
            println!("Contest Name: {}", contests[0].name);
        }
        Err(e) => {
            eprintln!("Error fetching contests: {}", e);
        }
    }

    Ok(())
}
