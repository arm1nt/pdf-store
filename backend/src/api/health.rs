use actix_web::{get, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}


#[get("/health")]
async fn health() -> impl Responder {
    let health_response = Response {
        message: "Everything is working fine".to_string()
    };
    HttpResponse::Ok().json(health_response)
}