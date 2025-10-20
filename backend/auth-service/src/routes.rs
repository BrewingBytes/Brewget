mod activate;
mod change_password;
mod forgot_password;
mod health;
mod login;
mod login_passkey;
mod logout;
mod middlewares;
mod register;
mod register_passkey;

use std::sync::Arc;

use axum::{
    Router,
    http::{
        HeaderValue, Method,
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    },
};
use diesel_async::{
    AsyncPgConnection,
    pooled_connection::{AsyncDieselConnectionManager, deadpool::Pool},
};
use tower_http::cors::CorsLayer;

use crate::{
    AppState, Config, grpc::email_service::service::email_service_client::EmailServiceClient,
};

pub async fn make_app(config: Config) -> Result<Router, Box<dyn std::error::Error>> {
    let cors = HeaderValue::from_str(&config.cors_url)?;
    let postgres_url = format!(
        "postgres://{}:{}@{}/{}",
        config.pg_username, config.pg_password, config.pg_url, config.pg_database
    );

    let db = AsyncDieselConnectionManager::<AsyncPgConnection>::new(&postgres_url);
    let db = Pool::builder(db)
        .build()
        .expect("Unable to create new db pool");

    // Create all the GRPCs Clients
    let email_service = EmailServiceClient::connect(format!(
        "{}:{}",
        config.email_hostname, config.email_grpc_port
    ))
    .await?;

    let state = Arc::new(AppState::new(config, db, email_service));

    let cors = CorsLayer::new()
        .allow_origin(cors)
        .allow_methods([Method::GET, Method::POST])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let router = Router::new()
        .nest("/health", health::get_router(state.clone()))
        .nest("/register", register::get_router(state.clone()))
        .nest("/register/passkey", register_passkey::get_router(state.clone()))
        .nest("/activate", activate::get_router(state.clone()))
        .nest(
            "/change-password",
            change_password::get_router(state.clone()),
        )
        .nest(
            "/forgot-password",
            forgot_password::get_router(state.clone()),
        )
        .nest("/login", login::get_router(state.clone()))
        .nest("/login/passkey", login_passkey::get_router(state.clone()))
        .nest("/logout", logout::get_router(state.clone()))
        .with_state(state)
        .layer(cors);
    Ok(router)
}
