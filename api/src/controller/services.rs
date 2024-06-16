use super::submission::SubmissionController;
use crate::service::submission::FetchSubmission;
use actix_web::web;
use std::sync::Arc;

pub fn config_submission_service(
    cfg: &mut web::ServiceConfig,
    submission_controller: Arc<SubmissionController<impl FetchSubmission + 'static>>,
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
                web::resource("/submissions/{platform}/{user}").route(web::get().to({
                    let controller = Arc::clone(&submission_controller);
                    move |path, query| {
                        let controller = Arc::clone(&controller);
                        async move { controller.user_submissions(path, query).await }
                    }
                })),
            ),
    );
}
