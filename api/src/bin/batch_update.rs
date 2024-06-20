//! Batch Process to Retrieve and Update Database
//! with All Problems/Contest Information from Various Programming Contest Sites

use anyhow::Result;
use dotenv::dotenv;
use env_logger;
use std::env;
use std::sync::Arc;

use api::domain::vo::platform;
use api::infra::api::api_client;
use api::infra::repository::initialize_pool::initialize_pool;
use api::service::update;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    env_logger::init();
    log::info!("Batch process started.");

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set.");
    let pool = Arc::new(
        initialize_pool(db_url)
            .await
            .expect("Failed to initialize the connection pool"),
    );
    let api_client = Arc::new(api_client::ApiClient::new());

    let platforms = vec![
        platform::Platform::Atcoder,
        platform::Platform::Codeforces,
        platform::Platform::Aoj,
        platform::Platform::YOJ,
        // platform::Platform::Yukicoder, // Takes a long time to fetch all problems.
    ];

    for p in platforms {
        log::info!("Start fetching problems from {:?}.", p);
        let usecase = update::UpdateUsecase::new(api_client.clone(), pool.clone());
        usecase.fetch_and_update(&p).await?;
        log::info!("Finished fetching problems from {:?}.", p);
    }

    log::info!("Batch process finished.");

    Ok(())
}
