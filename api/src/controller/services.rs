use super::{problem::ProblemController, submission::SubmissionController};
use crate::service::{problem::FetchProblem, submission::FetchSubmission};
use actix_web::web;
use std::sync::Arc;

pub fn config_services(
    cfg: &mut web::ServiceConfig,
    submission_controller: Arc<SubmissionController<impl FetchSubmission + 'static>>,
    problem_controller: Arc<ProblemController<impl FetchProblem + 'static>>,
) {
    cfg.service(
        web::scope("/api")
            .service(
                web::resource("/submissions/recent/{platform}").route(web::get().to({
                    let controller = Arc::clone(&submission_controller);
                    move |platform| {
                        let controller = Arc::clone(&controller);
                        async move { controller.recent_submissions(platform).await }
                    }
                })),
            )
            .service(
                web::resource("/submissions/{platform}/{user_id}").route(web::get().to({
                    let controller = Arc::clone(&submission_controller);
                    move |path, query| {
                        let controller = Arc::clone(&controller);
                        async move { controller.user_submissions(path, query).await }
                    }
                })),
            )
            .service(web::resource("/problems").route(web::get().to({
                let controller = Arc::clone(&problem_controller);
                move || {
                    let controller = Arc::clone(&controller);
                    async move { controller.problems().await }
                }
            }))),
    );
}
