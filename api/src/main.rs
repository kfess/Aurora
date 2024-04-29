use infrastracture::api::yukicoder::yukicoder_api_client::{
    IYukicoderAPIClient, YukicoderAPIClient,
};

mod infrastracture;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
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

    Ok(())
}
