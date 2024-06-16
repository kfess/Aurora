use actix_web::{get, HttpResponse, Responder};

#[get("/health")]
pub async fn get_healthcheck() -> impl Responder {
    HttpResponse::Ok().body("API is healthy!")
}
