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
    AppState, database,
    models::{
        response::Error,
        wallet::{CreateWallet, UpdateWallet},
    },
    routes::middlewares::auth_guard,
};

/// Creates a router for wallet routes
///
/// # Arguments
///
/// * `state` - Shared application state containing configuration and database connection
///
/// # Returns
///
/// Returns an Axum router configured with wallet endpoints with auth middleware.
///
/// # Routes
///
/// - `GET /` - Get all wallets for the authenticated user
/// - `POST /` - Create a new wallet
/// - `GET /:id` - Get a specific wallet by ID
/// - `PUT /:id` - Update a wallet by ID
/// - `DELETE /:id` - Delete a wallet by ID
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

/// Retrieves all wallets for a user
///
/// # Example Response
///
/// ```json
/// [
///   {
///     "id": "550e8400-e29b-41d4-a716-446655440000",
///     "user_id": "550e8400-e29b-41d4-a716-446655440001",
///     "name": "My Savings",
///     "balance": 1500.50,
///     "currency": "USD",
///     "wallet_type": "savings",
///     "created_at": "2024-01-01T00:00:00Z",
///     "updated_at": "2024-01-01T00:00:00Z"
///   }
/// ]
/// ```
async fn get_wallets(
    Extension(user_id): Extension<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    tracing::info!("GET /wallets - Fetching all wallets for user {}", user_id);

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

/// Creates a new wallet
///
/// # Example Request
///
/// ```json
/// {
///   "name": "My Wallet",
///   "balance": 100.0,
///   "currency": "USD",
///   "wallet_type": "general"
/// }
/// ```
async fn create_wallet(
    Extension(user_id): Extension<Uuid>,
    State(state): State<Arc<AppState>>,
    Json(create_wallet): Json<CreateWallet>,
) -> Result<impl IntoResponse, Error> {
    tracing::info!("POST /wallets - Creating wallet for user {}", user_id);
    tracing::debug!(
        "Wallet data: name={}, currency={}, balance={:?}",
        create_wallet.name,
        create_wallet.currency,
        create_wallet.balance
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
    Ok(Json(wallet))
}

/// Retrieves a specific wallet by ID
///
/// # Example Response
///
/// ```json
/// {
///   "id": "550e8400-e29b-41d4-a716-446655440000",
///   "user_id": "550e8400-e29b-41d4-a716-446655440001",
///   "name": "My Savings",
///   "balance": 1500.50,
///   "currency": "USD",
///   "wallet_type": "savings",
///   "created_at": "2024-01-01T00:00:00Z",
///   "updated_at": "2024-01-01T00:00:00Z"
/// }
/// ```
async fn get_wallet(
    Extension(user_id): Extension<Uuid>,
    State(state): State<Arc<AppState>>,
    Path(wallet_id): Path<Uuid>,
) -> Result<impl IntoResponse, Error> {
    tracing::info!(
        "GET /wallets/{} - Fetching wallet for user {}",
        wallet_id,
        user_id
    );

    let pool = state.get_database_pool();
    let wallet = database::wallet::find_by_id(wallet_id, user_id, pool)
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

/// Updates a wallet
///
/// # Example Request
///
/// ```json
/// {
///   "name": "Updated Name",
///   "balance": 2000.0
/// }
/// ```
async fn update_wallet(
    Extension(user_id): Extension<Uuid>,
    State(state): State<Arc<AppState>>,
    Path(wallet_id): Path<Uuid>,
    Json(update_wallet): Json<UpdateWallet>,
) -> Result<impl IntoResponse, Error> {
    tracing::info!(
        "PUT /wallets/{} - Updating wallet for user {}",
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

/// Deletes a wallet
///
/// # Example Response
///
/// Returns 200 OK with number of deleted rows
async fn delete_wallet(
    Extension(user_id): Extension<Uuid>,
    State(state): State<Arc<AppState>>,
    Path(wallet_id): Path<Uuid>,
) -> Result<impl IntoResponse, Error> {
    tracing::info!(
        "DELETE /wallets/{} - Deleting wallet for user {}",
        wallet_id,
        user_id
    );

    let pool = state.get_database_pool();
    let deleted = database::wallet::delete(wallet_id, user_id, pool)
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
