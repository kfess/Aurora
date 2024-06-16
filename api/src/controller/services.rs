use actix_web::web;

use crate::controller::health;
use crate::controller::submission;

pub fn config_services(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            // .service(web::scope("/submission").service(submission::))
            .service(health::get_healthcheck),
    );
}
