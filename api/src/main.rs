use axum::{
    routing::get,
    http::StatusCode,
    Json, Router,
};
use serde_json::Value;
use tracing_appender::{non_blocking, rolling};
use tracing_subscriber::{
    filter::EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt, Registry,
};

#[tokio::main]
async fn main() {
    // 初始化 tracing

    // 环境筛选
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));

    // 输出到控制台中
    let formatting_layer = fmt::layer().pretty().with_writer(std::io::stderr);
    // 输出到文件中
    let file_appender = rolling::never("logs", "app.log");
    let (non_blocking_appender, _guard) = non_blocking(file_appender);
    let file_layer = fmt::layer()
        .with_ansi(false)
        .with_writer(non_blocking_appender);

    // 注册
    Registry::default()
        .with(env_filter)
        .with(formatting_layer)
        .with(file_layer)
        .init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> (StatusCode, Json<Value>) {
	// Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "version": "0.1.0",
            "message": "一隅之地，偏安于此。"
        }"#;

	(StatusCode::OK, Json(serde_json::from_str(data).unwrap()))
}
