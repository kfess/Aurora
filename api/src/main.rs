use actix_web::{web, App, HttpServer};
use dotenv::dotenv;

use api::{
    controller::{self, services},
    infra::{api::api_client::ApiClient, repository::initialize_pool::initialize_pool},
    service::submission::{FetchSubmission, FetchSubmissionUsecase},
};

use std::{env, sync::Arc};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// #[tokio::main]
#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set.");
    let pool = initialize_pool(db_url)
        .await
        .expect("Failed to initialize the connection pool");

    let api_client = ApiClient::new();
    let submission_usecase = Arc::new(FetchSubmissionUsecase::new(api_client));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(submission_usecase.clone()))
            .configure(services::config_services)
    })
    .bind(("127.0.0.1", 8079))?
    .run()
    .await?;

    Ok(())
}
