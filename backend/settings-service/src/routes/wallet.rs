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
/// - `GET /` - List all wallets for the authenticated user
/// - `GET /:id` - Get a specific wallet by ID
/// - `POST /` - Create a new wallet
/// - `PUT /:id` - Update a wallet
/// - `DELETE /:id` - Delete a wallet
pub fn get_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(list_wallets))
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

/// Lists all wallets for the authenticated user
///
/// # Arguments
///
/// * `user_id` - The UUID of the authenticated user (from middleware)
/// * `state` - Shared application state
///
/// # Returns
///
/// * `Ok(Json<Vec<Wallet>>)` - List of wallets as JSON
/// * `Err(Error)` - Database operation error
///
/// # Example Response
///
/// ```json
/// [
///   {
///     "id": "550e8400-e29b-41d4-a716-446655440000",
///     "user_id": "660e8400-e29b-41d4-a716-446655440000",
///     "name": "Checking Account",
///     "balance": "1500.00",
///     "currency": "USD",
///     "wallet_type": "checking",
///     "created_at": "2024-01-01T00:00:00Z",
///     "updated_at": "2024-01-01T00:00:00Z"
///   }
/// ]
/// ```
async fn list_wallets(
    Extension(user_id): Extension<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    tracing::info!("GET /wallet - Listing wallets for user {}", user_id);

    let pool = state.get_database_pool();
    let wallets = database::wallet::find_by_user_id(user_id, pool)
        .await
        .inspect_err(|_| {
            tracing::error!("Failed to fetch wallets for user {}", user_id);
        })?;

    tracing::info!("Successfully fetched {} wallets for user {}", wallets.len(), user_id);
    Ok(Json(wallets))
}

/// Gets a specific wallet by ID
///
/// # Arguments
///
/// * `user_id` - The UUID of the authenticated user (from middleware)
/// * `wallet_id` - The UUID of the wallet to retrieve
/// * `state` - Shared application state
///
/// # Returns
///
/// * `Ok(Json<Wallet>)` - The wallet as JSON
/// * `Err(Error)` - Database operation error or wallet not found
///
/// # Example Response
///
/// ```json
/// {
///   "id": "550e8400-e29b-41d4-a716-446655440000",
///   "user_id": "660e8400-e29b-41d4-a716-446655440000",
///   "name": "Checking Account",
///   "balance": "1500.00",
///   "currency": "USD",
///   "wallet_type": "checking",
///   "created_at": "2024-01-01T00:00:00Z",
///   "updated_at": "2024-01-01T00:00:00Z"
/// }
/// ```
async fn get_wallet(
    Extension(user_id): Extension<Uuid>,
    State(state): State<Arc<AppState>>,
    Path(wallet_id): Path<Uuid>,
) -> Result<impl IntoResponse, Error> {
    tracing::info!("GET /wallet/{} - Fetching wallet for user {}", wallet_id, user_id);

    let pool = state.get_database_pool();
    let wallet = database::wallet::find_by_id(wallet_id, user_id, pool)
        .await
        .inspect_err(|_| {
            tracing::error!("Failed to fetch wallet {} for user {}", wallet_id, user_id);
        })?;

    tracing::info!("Successfully fetched wallet {} for user {}", wallet_id, user_id);
    Ok(Json(wallet))
}

/// Creates a new wallet
///
/// # Arguments
///
/// * `user_id` - The UUID of the authenticated user (from middleware)
/// * `state` - Shared application state
/// * `create_wallet` - The wallet creation data
///
/// # Returns
///
/// * `Ok(Json<Wallet>)` - The newly created wallet as JSON
/// * `Err(Error)` - Database operation error
///
/// # Example Request
///
/// ```http
/// POST /wallet
/// Content-Type: application/json
///
/// {
///   "name": "Savings Account",
///   "balance": 1000.00,
///   "currency": "USD",
///   "wallet_type": "savings"
/// }
/// ```
///
/// # Example Response
///
/// ```json
/// {
///   "id": "550e8400-e29b-41d4-a716-446655440000",
///   "user_id": "660e8400-e29b-41d4-a716-446655440000",
///   "name": "Savings Account",
///   "balance": "1000.00",
///   "currency": "USD",
///   "wallet_type": "savings",
///   "created_at": "2024-01-01T00:00:00Z",
///   "updated_at": "2024-01-01T00:00:00Z"
/// }
/// ```
async fn create_wallet(
    Extension(user_id): Extension<Uuid>,
    State(state): State<Arc<AppState>>,
    Json(create_wallet): Json<CreateWallet>,
) -> Result<impl IntoResponse, Error> {
    tracing::info!("POST /wallet - Creating wallet for user {}", user_id);
    tracing::debug!(
        "Create payload: name={}, balance={:?}, currency={:?}, type={:?}",
        create_wallet.name,
        create_wallet.balance,
        create_wallet.currency,
        create_wallet.wallet_type
    );

    let pool = state.get_database_pool();
    let wallet = database::wallet::create(user_id, create_wallet, pool)
        .await
        .inspect_err(|_| {
            tracing::error!("Failed to create wallet for user {}", user_id);
        })?;

    tracing::info!("Successfully created wallet {} for user {}", wallet.id, user_id);
    Ok((StatusCode::CREATED, Json(wallet)))
}

/// Updates a wallet
///
/// # Arguments
///
/// * `user_id` - The UUID of the authenticated user (from middleware)
/// * `state` - Shared application state
/// * `wallet_id` - The UUID of the wallet to update
/// * `update_wallet` - The wallet update data
///
/// # Returns
///
/// * `Ok(Json<Wallet>)` - The updated wallet as JSON
/// * `Err(Error)` - Database operation error or wallet not found
///
/// # Example Request
///
/// ```http
/// PUT /wallet/550e8400-e29b-41d4-a716-446655440000
/// Content-Type: application/json
///
/// {
///   "name": "Updated Savings",
///   "balance": 2000.00
/// }
/// ```
///
/// # Example Response
///
/// ```json
/// {
///   "id": "550e8400-e29b-41d4-a716-446655440000",
///   "user_id": "660e8400-e29b-41d4-a716-446655440000",
///   "name": "Updated Savings",
///   "balance": "2000.00",
///   "currency": "USD",
///   "wallet_type": "savings",
///   "created_at": "2024-01-01T00:00:00Z",
///   "updated_at": "2024-01-01T12:00:00Z"
/// }
/// ```
async fn update_wallet(
    Extension(user_id): Extension<Uuid>,
    State(state): State<Arc<AppState>>,
    Path(wallet_id): Path<Uuid>,
    Json(update_wallet): Json<UpdateWallet>,
) -> Result<impl IntoResponse, Error> {
    tracing::info!("PUT /wallet/{} - Updating wallet for user {}", wallet_id, user_id);
    tracing::debug!(
        "Update payload: name={:?}, balance={:?}, currency={:?}, type={:?}",
        update_wallet.name,
        update_wallet.balance,
        update_wallet.currency,
        update_wallet.wallet_type
    );

    let pool = state.get_database_pool();
    let wallet = database::wallet::update(wallet_id, user_id, update_wallet, pool)
        .await
        .inspect_err(|_| {
            tracing::error!("Failed to update wallet {} for user {}", wallet_id, user_id);
        })?;

    tracing::info!("Successfully updated wallet {} for user {}", wallet_id, user_id);
    Ok(Json(wallet))
}

/// Deletes a wallet
///
/// # Arguments
///
/// * `user_id` - The UUID of the authenticated user (from middleware)
/// * `state` - Shared application state
/// * `wallet_id` - The UUID of the wallet to delete
///
/// # Returns
///
/// * `Ok(StatusCode::NO_CONTENT)` - Wallet successfully deleted
/// * `Err(Error)` - Database operation error or wallet not found
///
/// # Example Request
///
/// ```http
/// DELETE /wallet/550e8400-e29b-41d4-a716-446655440000
/// ```
///
/// # Example Response
///
/// ```
/// 204 No Content
/// ```
async fn delete_wallet(
    Extension(user_id): Extension<Uuid>,
    State(state): State<Arc<AppState>>,
    Path(wallet_id): Path<Uuid>,
) -> Result<impl IntoResponse, Error> {
    tracing::info!("DELETE /wallet/{} - Deleting wallet for user {}", wallet_id, user_id);

    let pool = state.get_database_pool();
    database::wallet::delete(wallet_id, user_id, pool)
        .await
        .inspect_err(|_| {
            tracing::error!("Failed to delete wallet {} for user {}", wallet_id, user_id);
        })?;

    tracing::info!("Successfully deleted wallet {} for user {}", wallet_id, user_id);
    Ok(StatusCode::NO_CONTENT)
}
