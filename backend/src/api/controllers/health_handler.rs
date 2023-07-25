use log::trace;
use actix_web::{Responder, HttpResponse};

pub async fn health() -> impl Responder {
    trace!("health()");

    HttpResponse::Ok().json("Everything is working fine")
}
