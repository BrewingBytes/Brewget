mod health;
mod login;
mod logout;
mod middlewares;

use std::sync::Arc;

use axum::{
    Router,
    http::{
        HeaderValue, Method,
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    },
    middleware,
    routing::{get, post},
};
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;

use crate::{
    AppState, Config,
    routes::{
        health::health_checker_handler, login::login_handler, logout::logout_handler,
        middlewares::auth_guard::auth_guard,
    },
};

pub async fn make_app() -> Result<Router, Box<dyn std::error::Error>> {
    let config = Config::init();

    let cors = HeaderValue::from_str(&config.cors_url)?;
    let postgres_url = format!(
        "postgres://{}:{}@{}/{}",
        config.pg_username, config.pg_password, config.pg_url, config.pg_database
    );

    println!("{postgres_url}");

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&postgres_url)
        .await?;
    let state = Arc::new(AppState { config, db: db });

    let cors = CorsLayer::new()
        .allow_origin(cors)
        .allow_methods([Method::GET, Method::POST])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let router = Router::new()
        .route("/", get(health_checker_handler))
        // .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .route(
            "/logout",
            get(logout_handler)
                .route_layer(middleware::from_fn_with_state(state.clone(), auth_guard)),
        )
        .with_state(state)
        .layer(cors);
    Ok(router)
}
