use std::time::Duration;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tracing::log;

pub async fn connect() -> DatabaseConnection {
	let mut opt = ConnectOptions::new("mysql://root:@localhost:3306/yiyu");
	opt.max_connections(20)
		.min_connections(2)
		.connect_timeout(Duration::from_secs(8))
		.acquire_timeout(Duration::from_secs(8))
		.idle_timeout(Duration::from_secs(8))
		.max_lifetime(Duration::from_secs(8))
		.sqlx_logging(true)
		.sqlx_logging_level(log::LevelFilter::Info)
		.set_schema_search_path("my_schema"); // Setting default PostgreSQL schema

	Database::connect(opt).await.unwrap()
}
