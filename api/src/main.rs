use actix_web::{App, HttpServer};
use dotenv::dotenv;

use api::{
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

    let problem_usecase = Arc::new(FetchProblemUsecase::new(pool.clone()));
    let problem_controller = Arc::new(ProblemController::new(problem_usecase.clone()));

    let contest_usecase = Arc::new(FetchContestUsecase::new(pool.clone()));
    let contest_controller = Arc::new(ContestController::new(contest_usecase.clone()));

    let auth_usecase = Arc::new(AuthUsecase::new(oidc_client, repository));
    let auth_controller = Arc::new(AuthController::new(auth_usecase.clone()));

    HttpServer::new(move || {
        App::new()
            .wrap_fn(|req, srv| {
                let authorized_routes = vec!["/api/auth/user"];
                let path = req.path();

                if authorized_routes.iter().any(|route| route == path) {
                    let jwt = get_cookie_value(req, &env::var("JWT_COOKIE_KEY").unwrap()).unwrap();
                    match decode_jwt(env::var("JWT_SECRET").unwrap().as_str(), &jwt) {
                        Ok(_) => srv.call(req),
                        Err(_) => {
                            let res = HttpResponse::Unauthorized().finish();
                            future::ok(res)
                        }
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
    .bind(("127.0.0.1", 8079))?
    .run()
    .await?;

    Ok(())
}
