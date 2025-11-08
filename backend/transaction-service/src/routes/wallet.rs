use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{delete, get, post, put},
};
use uuid::Uuid;

use crate::{
    AppState, database,
    models::{
        response::Error,
        wallet::{CreateWallet, UpdateWallet},
    },
    routes::middlewares::auth_guard,
};

/// Creates a router for the wallet routes
///
/// This function sets up the wallet endpoints and returns a configured Axum router.
///
/// # Arguments
///
/// * `state` - Shared application state containing configuration and database connection
///
/// # Returns
///
/// Returns an Axum router configured with the wallet endpoints with auth middleware.
///
/// # Routes
///
/// - `GET /` - Get all wallets for authenticated user (protected by auth middleware)
/// - `POST /` - Create a new wallet (protected by auth middleware)
/// - `PUT /:id` - Update a wallet by ID (protected by auth middleware)
/// - `DELETE /:id` - Delete a wallet by ID (protected by auth middleware)
pub fn get_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_all_wallets))
        .route("/", post(create_wallet))
        .route("/{id}", put(update_wallet))
        .route("/{id}", delete(delete_wallet))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_guard::auth_guard,
        ))
        .with_state(state)
}

/// Retrieves all wallets for the authenticated user
///
/// # Arguments
///
/// * `user_id` - The UUID of the authenticated user (from auth middleware)
/// * `state` - Shared application state
///
/// # Returns
///
/// * `Ok(Json<Vec<Wallet>>)` - The user's wallets as JSON
/// * `Err(Error)` - Database operation error
async fn get_all_wallets(
    Extension(user_id): Extension<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    tracing::info!("GET /wallet - Fetching all wallets for user {}", user_id);

    let pool = state.get_database_pool();

    let wallets = database::wallet::find_all_by_user(user_id, pool)
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

/// Creates a new wallet for the authenticated user
///
/// # Arguments
///
/// * `user_id` - The UUID of the authenticated user (from auth middleware)
/// * `state` - Shared application state
/// * `create_wallet` - The wallet creation data
///
/// # Returns
///
/// * `Ok(Json<Wallet>)` - The created wallet as JSON
/// * `Err(Error)` - Database operation error
async fn create_wallet(
    Extension(user_id): Extension<Uuid>,
    State(state): State<Arc<AppState>>,
    Json(create_wallet): Json<CreateWallet>,
) -> Result<impl IntoResponse, Error> {
    tracing::info!("POST /wallet - Creating wallet for user {}", user_id);
    tracing::debug!(
        "Create payload: name={}, currency={}",
        create_wallet.name,
        create_wallet.currency
    );

    let pool = state.get_database_pool();

    let wallet = database::wallet::create(user_id, create_wallet, pool)
        .await
        .inspect_err(|_| {
            tracing::error!("Failed to create wallet for user {}", user_id);
        })?;

    tracing::info!(
        "Successfully created wallet {} for user {}",
        wallet.id,
        user_id
    );
    Ok((StatusCode::CREATED, Json(wallet)))
}

/// Updates a wallet
///
/// # Arguments
///
/// * `user_id` - The UUID of the authenticated user (from auth middleware)
/// * `wallet_id` - The UUID of the wallet to update
/// * `state` - Shared application state
/// * `update_wallet` - The wallet update data
///
/// # Returns
///
/// * `Ok(Json<Wallet>)` - The updated wallet as JSON
/// * `Err(Error)` - Database operation error
async fn update_wallet(
    Extension(user_id): Extension<Uuid>,
    Path(wallet_id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
    Json(update_wallet): Json<UpdateWallet>,
) -> Result<impl IntoResponse, Error> {
    tracing::info!(
        "PUT /wallet/{} - Updating wallet for user {}",
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

    let wallet = database::wallet::update(wallet_id, user_id, update_wallet, pool)
        .await
        .inspect_err(|_| {
            tracing::error!("Failed to update wallet {} for user {}", wallet_id, user_id);
        })?;

    tracing::info!(
        "Successfully updated wallet {} for user {}",
        wallet_id,
        user_id
    );
    Ok(Json(wallet))
}

/// Deletes a wallet
///
/// # Arguments
///
/// * `user_id` - The UUID of the authenticated user (from auth middleware)
/// * `wallet_id` - The UUID of the wallet to delete
/// * `state` - Shared application state
///
/// # Returns
///
/// * `Ok(StatusCode::NO_CONTENT)` - If deletion succeeds
/// * `Err(Error)` - Database operation error
async fn delete_wallet(
    Extension(user_id): Extension<Uuid>,
    Path(wallet_id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    tracing::info!(
        "DELETE /wallet/{} - Deleting wallet for user {}",
        wallet_id,
        user_id
    );

    let pool = state.get_database_pool();

    database::wallet::delete(wallet_id, user_id, pool)
        .await
        .inspect_err(|_| {
            tracing::error!("Failed to delete wallet {} for user {}", wallet_id, user_id);
        })?;

    tracing::info!(
        "Successfully deleted wallet {} for user {}",
        wallet_id,
        user_id
    );
    Ok(StatusCode::NO_CONTENT)
}
