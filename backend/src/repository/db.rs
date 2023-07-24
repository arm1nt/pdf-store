use sqlx::{Pool, Postgres, Error, postgres::PgPoolOptions};
use log::info;
use std::env;



pub async fn connect() -> Result<Pool<Postgres>, Error> {
    info!("Creating database connection pool");

    let database_url = env::var("DATABASE_URL").expect("Environment variable DATABASE_URL must be set");
    
    let pool = PgPoolOptions::new()
    .max_connections(10)
    .connect(&database_url)
    .await;

    return pool;
}