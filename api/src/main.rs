use actix_web::{App, HttpServer};
use dotenv::dotenv;

use api::{
    controller::{
        problem::ProblemController, services::config_services, submission::SubmissionController,
    },
    infra::{api::api_client::ApiClient, repository::initialize_pool::initialize_pool},
    service::{problem::FetchProblemUsecase, submission::FetchSubmissionUsecase},
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

    let sub_usecase = Arc::new(FetchSubmissionUsecase::new(api_client));
    let sub_controller = Arc::new(SubmissionController::new(sub_usecase.clone()));

    let problem_usecase = Arc::new(FetchProblemUsecase::new(pool));
    let problem_controller = Arc::new(ProblemController::new(problem_usecase.clone()));

    HttpServer::new(move || {
        App::new().configure(|cfg| {
            config_services(cfg, sub_controller.clone(), problem_controller.clone())
        })
    })
    .bind(("127.0.0.1", 8079))?
    .run()
    .await?;

    Ok(())
}
