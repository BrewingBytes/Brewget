mod app_state;
mod config;
mod models;
mod routes;
mod schema;
mod utils;

pub use app_state::AppState;
pub use config::Config;

use crate::routes::make_app;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let app = make_app().await.expect("Could not create app.");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8081")
        .await
        .expect("Could not bind TcpListener.");

    println!("🚀 Server started successfully");

    axum::serve(listener, app)
        .await
        .expect("Could not serve axum server.");
}
