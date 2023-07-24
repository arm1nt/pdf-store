use std::io::Result;
use actix_cors::Cors;
use actix_web::{HttpServer, App, web::Data, middleware};
use env_logger::{init_from_env, Env};
use std::env;
use log::info;
use sqlx::{Pool, Postgres};

pub mod api;
pub mod util;
pub mod repository;

pub struct AppState {
    db: Pool<Postgres>
}

#[actix_web::main]
async fn main() -> Result<()> {

    dotenv::dotenv().ok();

    init_from_env(Env::new().default_filter_or("info"));

    let database_connection = repository::db::connect()
        .await
        .expect("Error connecting to database");

    
    let backend_url = env::var("BACKEND_URL").unwrap_or("http://localhost".to_string());
    let backend_port = env::var("BACKEND_PORT").unwrap_or("8080".to_string());

    info!("Starting HTTP Server at {backend_url}:{backend_port}");
    
    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(Data::new(AppState {db: database_connection.clone()}))
            .service(api::health::health)
    })
    .bind(("127.0.0.1", 8080))?
    .workers(3)
    .run()
    .await
}
