//! Batch Process to Retrieve and Update Database
//! with All Problems/Contest Information from Various Programming Contest Sites

use anyhow::Result;
use dotenv::dotenv;
use std::env;
use std::sync::Arc;

use api::infra::api::api_client;
use api::infra::repository::initialize_pool::initialize_pool;
use api::service::update_problems;

#[tokio::main]
async fn main() -> Result<()> {
    log::info!("Batch process started.");

    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set.");
    let pool = Arc::new(
        initialize_pool(db_url)
            .await
            .expect("Failed to initialize the connection pool"),
    );
    let api_client = Arc::new(api_client::ApiClient::new());

    // Atcoder
    {
        log::info!("Start fetching AtCoder problems.");
        let usecase =
            update_problems::atcoder::UpdateAtcoderUsecase::new(api_client.clone(), pool.clone());
        usecase.fetch_and_update().await?;

        log::info!("Finished fetching AtCoder problems.");
    }

    // Codeforces
    {
        log::info!("Start fetching Codeforces problems.");
        let usecase =
            update_problems::cf::UpdateCodeforcesUsecase::new(api_client.clone(), pool.clone());
        usecase.fetch_and_update().await?;

        log::info!("Finished fetching Codeforces problems.")
    }

    // Yukicoder
    {
        log::info!("Start fetching Yukicoder problems.");
        let usecase =
            update_problems::yuki::UpdateYukicoderUsecase::new(api_client.clone(), pool.clone());
        // usecase.fetch_and_update(false).await?;

        log::info!("Finished fetching Yukicoder problems.");
    }

    // yosupo online judge
    {
        log::info!("Start fetching yosupo online judge problems.");
        let usecase = update_problems::yoj::UpdateYOJUsecase::new(api_client.clone(), pool.clone());
        // usecase.fetch_and_update().await?;

        log::info!("Finished fetching yosupo online judge problems.");
    }

    // Aizu Online Judge
    {
        log::info!("Start fetching Aizu Online Judge problems.");
        let usecase = update_problems::aoj::UpdateAojUsecase::new(api_client.clone(), pool.clone());
        // usecase.fetch_and_update().await?;

        log::info!("Finished fetching Aizu Online Judge problems.");
    }

    log::info!("Batch process finished.");
    Ok(())
}
