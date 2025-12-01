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
        transaction::{CreateTransaction, UpdateTransaction},
    },
    routes::middlewares::auth_guard,
};

/// Creates a router for the transaction routes
///
/// This function sets up the transaction endpoints and returns a configured Axum router.
///
/// # Arguments
///
/// * `state` - Shared application state containing configuration and database connection
///
/// # Returns
///
/// Returns an Axum router configured with the transaction endpoints with auth middleware.
///
/// # Routes
///
/// - `GET /` - Get all transactions for authenticated user (protected by auth middleware)
/// - `GET /wallet/:wallet_id` - Get all transactions for a specific wallet (protected by auth middleware)
/// - `GET /:id` - Get a specific transaction by ID (protected by auth middleware)
/// - `POST /` - Create a new transaction (protected by auth middleware)
/// - `PUT /:id` - Update a transaction by ID (protected by auth middleware)
/// - `DELETE /:id` - Delete a transaction by ID (protected by auth middleware)
pub fn get_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_all_transactions))
        .route("/", post(create_transaction))
        .route("/wallet/{wallet_id}", get(get_wallet_transactions))
        .route("/{id}", get(get_transaction))
        .route("/{id}", put(update_transaction))
        .route("/{id}", delete(delete_transaction))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_guard::auth_guard,
        ))
        .with_state(state)
}

/// Retrieves all transactions for the authenticated user
///
/// # Arguments
///
/// * `user_id` - The UUID of the authenticated user (from auth middleware)
/// * `state` - Shared application state
///
/// # Returns
///
/// * `Ok(Json<Vec<Transaction>>)` - The user's transactions as JSON
/// * `Err(Error)` - Database operation error
async fn get_all_transactions(
    Extension(user_id): Extension<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    tracing::info!(
        "GET /transaction - Fetching all transactions for user {}",
        user_id
    );

    let pool = state.get_database_pool();

    let transactions = database::transaction::find_all_by_user(user_id, pool)
        .await
        .inspect_err(|_| {
            tracing::error!("Failed to fetch transactions for user {}", user_id);
        })?;

    tracing::info!(
        "Successfully fetched {} transactions for user {}",
        transactions.len(),
        user_id
    );
    Ok(Json(transactions))
}

/// Retrieves all transactions for a specific wallet
///
/// # Arguments
///
/// * `user_id` - The UUID of the authenticated user (from auth middleware)
/// * `wallet_id` - The UUID of the wallet
/// * `state` - Shared application state
///
/// # Returns
///
/// * `Ok(Json<Vec<Transaction>>)` - The wallet's transactions as JSON
/// * `Err(Error)` - Database operation error
async fn get_wallet_transactions(
    Extension(user_id): Extension<Uuid>,
    Path(wallet_id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    tracing::info!(
        "GET /transaction/wallet/{} - Fetching transactions for user {}",
        wallet_id,
        user_id
    );

    let pool = state.get_database_pool();

    let transactions = database::transaction::find_all_by_wallet(wallet_id, user_id, pool)
        .await
        .inspect_err(|_| {
            tracing::error!(
                "Failed to fetch transactions for wallet {} and user {}",
                wallet_id,
                user_id
            );
        })?;

    tracing::info!(
        "Successfully fetched {} transactions for wallet {} and user {}",
        transactions.len(),
        wallet_id,
        user_id
    );
    Ok(Json(transactions))
}

/// Retrieves a specific transaction by ID
///
/// # Arguments
///
/// * `user_id` - The UUID of the authenticated user (from auth middleware)
/// * `transaction_id` - The UUID of the transaction
/// * `state` - Shared application state
///
/// # Returns
///
/// * `Ok(Json<Transaction>)` - The transaction as JSON
/// * `Err(Error)` - Database operation error or transaction not found
async fn get_transaction(
    Extension(user_id): Extension<Uuid>,
    Path(transaction_id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    tracing::info!(
        "GET /transaction/{} - Fetching transaction for user {}",
        transaction_id,
        user_id
    );

    let pool = state.get_database_pool();

    let transaction = database::transaction::find_by_id(transaction_id, user_id, pool)
        .await
        .inspect_err(|_| {
            tracing::error!(
                "Failed to fetch transaction {} for user {}",
                transaction_id,
                user_id
            );
        })?;

    tracing::info!(
        "Successfully fetched transaction {} for user {}",
        transaction_id,
        user_id
    );
    Ok(Json(transaction))
}

/// Creates a new transaction for the authenticated user
///
/// # Arguments
///
/// * `user_id` - The UUID of the authenticated user (from auth middleware)
/// * `state` - Shared application state
/// * `create_transaction` - The transaction creation data
///
/// # Returns
///
/// * `Ok(Json<Transaction>)` - The created transaction as JSON
/// * `Err(Error)` - Database operation error
async fn create_transaction(
    Extension(user_id): Extension<Uuid>,
    State(state): State<Arc<AppState>>,
    Json(create_transaction): Json<CreateTransaction>,
) -> Result<impl IntoResponse, Error> {
    tracing::info!(
        "POST /transaction - Creating transaction for user {}",
        user_id
    );
    tracing::debug!(
        "Create payload: wallet_id={}, amount={}, type={}",
        create_transaction.wallet_id,
        create_transaction.amount,
        create_transaction.transaction_type
    );

    let pool = state.get_database_pool();

    let transaction = database::transaction::create(user_id, create_transaction, pool)
        .await
        .inspect_err(|_| {
            tracing::error!("Failed to create transaction for user {}", user_id);
        })?;

    tracing::info!(
        "Successfully created transaction {} for user {}",
        transaction.id,
        user_id
    );
    Ok((StatusCode::CREATED, Json(transaction)))
}

/// Updates a transaction
///
/// # Arguments
///
/// * `user_id` - The UUID of the authenticated user (from auth middleware)
/// * `transaction_id` - The UUID of the transaction to update
/// * `state` - Shared application state
/// * `update_transaction` - The transaction update data
///
/// # Returns
///
/// * `Ok(Json<Transaction>)` - The updated transaction as JSON
/// * `Err(Error)` - Database operation error
async fn update_transaction(
    Extension(user_id): Extension<Uuid>,
    Path(transaction_id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
    Json(update_transaction): Json<UpdateTransaction>,
) -> Result<impl IntoResponse, Error> {
    tracing::info!(
        "PUT /transaction/{} - Updating transaction for user {}",
        transaction_id,
        user_id
    );
    tracing::debug!(
        "Update payload: amount={:?}, category={:?}",
        update_transaction.amount,
        update_transaction.category
    );

    let pool = state.get_database_pool();

    let transaction =
        database::transaction::update(transaction_id, user_id, update_transaction, pool)
            .await
            .inspect_err(|_| {
                tracing::error!(
                    "Failed to update transaction {} for user {}",
                    transaction_id,
                    user_id
                );
            })?;

    tracing::info!(
        "Successfully updated transaction {} for user {}",
        transaction_id,
        user_id
    );
    Ok(Json(transaction))
}

/// Deletes a transaction
///
/// # Arguments
///
/// * `user_id` - The UUID of the authenticated user (from auth middleware)
/// * `transaction_id` - The UUID of the transaction to delete
/// * `state` - Shared application state
///
/// # Returns
///
/// * `Ok(StatusCode::NO_CONTENT)` - If deletion succeeds
/// * `Err(Error)` - Database operation error
async fn delete_transaction(
    Extension(user_id): Extension<Uuid>,
    Path(transaction_id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    tracing::info!(
        "DELETE /transaction/{} - Deleting transaction for user {}",
        transaction_id,
        user_id
    );

    let pool = state.get_database_pool();

    database::transaction::delete(transaction_id, user_id, pool)
        .await
        .inspect_err(|_| {
            tracing::error!(
                "Failed to delete transaction {} for user {}",
                transaction_id,
                user_id
            );
        })?;

    tracing::info!(
        "Successfully deleted transaction {} for user {}",
        transaction_id,
        user_id
    );
    Ok(StatusCode::NO_CONTENT)
}
