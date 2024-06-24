use axum::{
    routing::get,
    http::StatusCode,
    Json, Router,
};
use serde_json::Value;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
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
