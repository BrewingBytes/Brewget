use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    middleware,
    response::IntoResponse,
    routing::{delete, get, post, put},
};
use uuid::Uuid;

use crate::{
    AppState, wallet_db,
    wallet_model::{CreateWallet, UpdateWallet},
    auth_guard,
};
use shared_types::Error;

pub fn get_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_wallets))
        .route("/", post(create_wallet))
        .route("/:id", get(get_wallet))
        .route("/:id", put(update_wallet))
        .route("/:id", delete(delete_wallet))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_guard::auth_guard,
        ))
        .with_state(state)
}

async fn get_wallets(
    Extension(user_id): Extension<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    tracing::info!("GET / - Fetching all wallets for user {}", user_id);

    let pool = state.get_database_pool();
    let wallets = wallet_db::find_all_by_user(user_id, pool)
        .await
        .inspect_err(|_| {
            tracing::error!("Failed to fetch wallets for user {}", user_id);
        })?;

    tracing::info!(
        "Successfully fetched {} wallets for user {}",
        wallets.len(),
        user_id
    );
    Ok(Json(wallets))
}

async fn create_wallet(
    Extension(user_id): Extension<Uuid>,
    State(state): State<Arc<AppState>>,
    Json(create_wallet): Json<CreateWallet>,
) -> Result<impl IntoResponse, Error> {
    tracing::info!("POST / - Creating wallet for user {}", user_id);
    tracing::debug!(
        "Wallet data: name={}, currency={:?}, balance={:?}",
        create_wallet.name,
        create_wallet.currency,
        create_wallet.balance
    );

    let pool = state.get_database_pool();
    let wallet = wallet_db::create(user_id, create_wallet, pool)
        .await
        .inspect_err(|_| {
            tracing::error!("Failed to create wallet for user {}", user_id);
        })?;

    tracing::info!(
        "Successfully created wallet {} for user {}",
        wallet.id,
        user_id
    );
    Ok(Json(wallet))
}

async fn get_wallet(
    Extension(user_id): Extension<Uuid>,
    State(state): State<Arc<AppState>>,
    Path(wallet_id): Path<Uuid>,
) -> Result<impl IntoResponse, Error> {
    tracing::info!(
        "GET /{} - Fetching wallet for user {}",
        wallet_id,
        user_id
    );

    let pool = state.get_database_pool();
    let wallet = wallet_db::find_by_id(wallet_id, user_id, pool)
        .await
        .inspect_err(|_| {
            tracing::error!(
                "Failed to fetch wallet {} for user {}",
                wallet_id,
                user_id
            );
        })?;

    tracing::info!(
        "Successfully fetched wallet {} for user {}",
        wallet_id,
        user_id
    );
    Ok(Json(wallet))
}

async fn update_wallet(
    Extension(user_id): Extension<Uuid>,
    State(state): State<Arc<AppState>>,
    Path(wallet_id): Path<Uuid>,
    Json(update_wallet): Json<UpdateWallet>,
) -> Result<impl IntoResponse, Error> {
    tracing::info!(
        "PUT /{} - Updating wallet for user {}",
        wallet_id,
        user_id
    );
    tracing::debug!(
        "Update payload: name={:?}, balance={:?}, currency={:?}",
        update_wallet.name,
        update_wallet.balance,
        update_wallet.currency
    );

    let pool = state.get_database_pool();
    let wallet = wallet_db::update(wallet_id, user_id, update_wallet, pool)
        .await
        .inspect_err(|_| {
            tracing::error!(
                "Failed to update wallet {} for user {}",
                wallet_id,
                user_id
            );
        })?;

    tracing::info!(
        "Successfully updated wallet {} for user {}",
        wallet_id,
        user_id
    );
    Ok(Json(wallet))
}

async fn delete_wallet(
    Extension(user_id): Extension<Uuid>,
    State(state): State<Arc<AppState>>,
    Path(wallet_id): Path<Uuid>,
) -> Result<impl IntoResponse, Error> {
    tracing::info!(
        "DELETE /{} - Deleting wallet for user {}",
        wallet_id,
        user_id
    );

    let pool = state.get_database_pool();
    let deleted = wallet_db::delete(wallet_id, user_id, pool)
        .await
        .inspect_err(|_| {
            tracing::error!(
                "Failed to delete wallet {} for user {}",
                wallet_id,
                user_id
            );
        })?;

    tracing::info!(
        "Successfully deleted wallet {} for user {}",
        wallet_id,
        user_id
    );
    Ok(Json(deleted))
}
