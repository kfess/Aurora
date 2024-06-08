use dotenv::dotenv;
use infra::repository::{
    initialize_pool::initialize_pool,
    problem::{Condition, ProblemRepository},
};

use std::env;

mod domain;
mod infra;
mod service;
mod utils;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set.");
    let pool = initialize_pool(db_url)
        .await
        .expect("Failed to initialize the connection pool");

    pool.get_problems_by_condition(&Condition {
        platform: Some("codeforces"),
        algo_id: None,
        technical_tag_id: None,
        page: Some(2),
        page_size: Some(20),
        from_difficulty: None,
        to_difficulty: Some(2200),
    })
    .await?;

    Ok(())
}
