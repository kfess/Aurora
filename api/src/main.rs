use actix_web::{App, HttpResponse, HttpServer};
use api::{
    config::{AUTHORIZED_ROUTES, CONFIG},
    controller::{
        auth::AuthController, contest::ContestController, problem::ProblemController,
        services::config_services, submission::SubmissionController,
    },
    infra::{api::api_client::ApiClient, repository::initialize_pool::initialize_pool},
    service::{
        auth::AuthUsecase, contest::FetchContestUsecase, problem::FetchProblemUsecase,
        submission::FetchSubmissionUsecase,
    },
};
use dotenv::dotenv;
use std::{env, sync::Arc};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// #[tokio::main]
#[actix_web::main]
async fn main() -> Result<()> {
    let pool = initialize_pool(CONFIG.database_url)
        .await
        .expect("Failed to initialize the connection pool");

    let api_client = ApiClient::new();

    let sub_usecase = Arc::new(FetchSubmissionUsecase::new(api_client));
    let sub_controller = Arc::new(SubmissionController::new(sub_usecase.clone()));

    let problem_usecase = Arc::new(FetchProblemUsecase::new(pool.clone()));
    let problem_controller = Arc::new(ProblemController::new(problem_usecase.clone()));

    let contest_usecase = Arc::new(FetchContestUsecase::new(pool.clone()));
    let contest_controller = Arc::new(ContestController::new(contest_usecase.clone()));

    let auth_usecase = Arc::new(AuthUsecase::new(oidc_client, repository));
    let auth_controller = Arc::new(AuthController::new(auth_usecase.clone()));

    HttpServer::new(move || {
        App::new()
            .wrap_fn(|req, srv| {
                if AUTHORIZED_ROUTES.iter().any(|&route| route == req.path()) {
                    let jwt = get_cookie_value(req, &CONFIG.jwt_cookie_key).unwrap();
                    match decode_jwt(CONFIG.jwt_secret, &jwt) {
                        Ok(_) => srv.call(req),
                        Err(_) => HttpResponse::Unauthorized().finish(),
                    }
                }
            })
            .configure(|cfg| {
                config_services(
                    cfg,
                    sub_controller.clone(),
                    problem_controller.clone(),
                    contest_controller.clone(),
                    auth_controller.clone(),
                )
            })
    })
    .bind((CONFIG.host.as_str(), CONFIG.port))?
    .run()
    .await?;

    Ok(())
}
