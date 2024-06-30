use axum::{http::StatusCode, routing::get, Json, Router};
use serde_json::Value;
use tracing_appender::{non_blocking, rolling};
use tracing_subscriber::{
    filter::EnvFilter, layer::SubscriberExt, util::SubscriberInitExt, Registry,
};
use yiyu_core::{db, logger};

#[tokio::main]
async fn main() {
	// Load environment variables from .env file.
	// Fails if .env file not found, not readable or invalid.
	dotenvy::dotenv().unwrap();

    // 初始化 tracing

    // 环境筛选
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));
    // 自定义的时间格式器
    let custom_timer = logger::timer("[year]-[month]-[day] [hour]:[minute]:[second]");

    // 输出文本到控制台中
    let formatting_layer = logger::text_layer().with_writer(std::io::stderr);
    // 输出JSON到文件中
    let file_appender = rolling::never("logs", "app.log");
    let (non_blocking_appender, _guard) = non_blocking(file_appender);
    let file_layer = logger::json_layer()
        .with_timer(custom_timer)
        .with_writer(non_blocking_appender);

    // 注册
    Registry::default()
        .with(env_filter)
        .with(formatting_layer)
        .with(file_layer)
        .init();

    // 连接数据库
    let _db = db::connect().await;

    // 构建应用路由
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root));

    // 使用 hyper 运行应用, 监听全局端口 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// 根路由
pub async fn root() -> (StatusCode, Json<Value>) {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "version": "0.1.0",
            "message": "一隅之地，偏安于此。"
        }"#;

    (StatusCode::OK, Json(serde_json::from_str(data).unwrap()))
}
