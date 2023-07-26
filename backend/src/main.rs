use std::io::Result;
use std::sync::Arc;
use actix_cors::Cors;
use actix_multipart::form::MultipartFormConfig;
use actix_multipart::form::tempfile::TempFileConfig;
use actix_web::{HttpServer, App, web::Data, middleware, web};
use env_logger::{init_from_env, Env};
use service::pdf::PdfServiceImpl;
use std::env;
use log::info;

use crate::api::controllers::health_handler::health;
use crate::api::controllers::pdf_handler::{get_all, get_by_id, get_metadata_by_id, search, update, delete, upload};
use crate::repository::pdfs::PdfRepositoryImpl;

pub mod api;
pub mod util;
pub mod repository;
pub mod model;
pub mod service;
pub mod errors;

pub struct AppState {
    service: PdfServiceImpl
}

#[actix_web::main]
async fn main() -> Result<()> {

    dotenv::dotenv().ok();

    init_from_env(Env::new().default_filter_or("info"));

    let database_connection = repository::db::connect()
        .await
        .expect("Error connecting to database");

    let backend_url = env::var("BACKEND_URL").unwrap_or("127.0.0.1".to_string());
    let backend_port = env::var("BACKEND_PORT").unwrap_or("8080".to_string());

    std::fs::create_dir_all("./tmp")?;

    info!("Starting HTTP Server at http://{backend_url}:{backend_port}");
    
    HttpServer::new(move || {
        let cors = Cors::permissive();

        let pdf_repository = PdfRepositoryImpl {
            pool: Arc::new(database_connection.clone())
        };

        let pdf_service = PdfServiceImpl {
            repository: Arc::new(pdf_repository)
        };

        let multipart_config = MultipartFormConfig::default()
        .memory_limit(1073741824 * 5)
        .total_limit(1073741824 * 5);

        App::new()
            .wrap(middleware::Logger::new("%a \"%r\" Status: %s (Req size: %{Content-Length}i) (Time: %T) \"%{Referer}i\""))
            .wrap(cors)
            .app_data(multipart_config)
            .app_data(Data::new(AppState {service: pdf_service.clone()}))
            .app_data(TempFileConfig::default().directory("./tmp"))
            .service(
                web::scope("/health")
                    .route("", web::get().to(health))
            )
            .service(
                web::scope("/pdfs")
                    .route("", web::get().to(get_all))
                    .route("/search", web::get().to(search))
                    .route("/{pdf_id}", web::get().to(get_by_id))
                    .route("/metadata/{pdf_id}", web::get().to(get_metadata_by_id))
                    .route("/{pdf_id}", web::put().to(update))       
                    .route("/{pdf_id}", web::delete().to(delete))
                    .route("/upload", web::post().to(upload))
            )
    })
    .bind(("127.0.0.1", 8081))?
    .workers(3)
    .run()
    .await
}
