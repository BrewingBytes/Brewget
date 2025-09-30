use crate::{config::Config, routes::make_app};

mod app_state;
mod config;
mod models;
mod routes;
mod schema;

pub use app_state::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let config = Config::init();
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.settings_http_port))
        .await
        .expect("Could not bind TcpListener.");

    let app = make_app(config).await.expect("Could not create app.");

    println!("🚀 Server started successfully");
    axum::serve(listener, app)
        .await
        .expect("Could not serve axum server.");

    Ok(())
}
