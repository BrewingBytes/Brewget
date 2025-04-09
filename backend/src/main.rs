use axum::{
    routing::get,
    Router,
    response::Json,
};
use serde_json::json;
use tower_http::cors::{CorsLayer, Any};

#[tokio::main]
async fn main() {
    // Initialize CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build our application with a route
    let app = Router::new()
        .route("/health", get(health_check))
        .layer(cors);

    // Run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

// Health check handler
async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "message": "Server is running"
    }))
}