use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::{env, time::Duration};
use tracing::log;

pub async fn connect() -> DatabaseConnection {
    let app_debug = env::var("APP_DEBUG").expect("APP_DEBUG is not set in .env file");
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    let mut opt = ConnectOptions::new(db_url);
    opt.max_connections(20)
        .min_connections(2)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(app_debug == "true")
        .sqlx_logging_level(log::LevelFilter::Info); // Setting default PostgreSQL schema

    Database::connect(opt).await.unwrap()
}
