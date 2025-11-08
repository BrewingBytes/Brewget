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
        custom_category::{CreateCustomCategory, UpdateCustomCategory},
        response::Error,
    },
    routes::middlewares::auth_guard,
};

/// Creates a router for the custom category routes
///
/// This function sets up the custom category endpoints and returns a configured Axum router.
///
/// # Arguments
///
/// * `state` - Shared application state containing configuration and database connection
///
/// # Returns
///
/// Returns an Axum router configured with the custom category endpoints with auth middleware.
///
/// # Routes
///
/// - `GET /` - Get all custom categories for authenticated user (protected by auth middleware)
/// - `POST /` - Create a new custom category (protected by auth middleware)
/// - `PUT /:id` - Update a custom category by ID (protected by auth middleware)
/// - `DELETE /:id` - Delete a custom category by ID (protected by auth middleware)
pub fn get_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_all_categories))
        .route("/", post(create_category))
        .route("/{id}", put(update_category))
        .route("/{id}", delete(delete_category))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_guard::auth_guard,
        ))
        .with_state(state)
}

/// Retrieves all custom categories for the authenticated user
///
/// # Arguments
///
/// * `user_id` - The UUID of the authenticated user (from auth middleware)
/// * `state` - Shared application state
///
/// # Returns
///
/// * `Ok(Json<Vec<CustomCategory>>)` - The user's custom categories as JSON
/// * `Err(Error)` - Database operation error
async fn get_all_categories(
    Extension(user_id): Extension<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    tracing::info!("GET /category - Fetching all custom categories for user {}", user_id);

    let pool = state.get_database_pool();

    let categories = database::custom_category::find_all_by_user(user_id, pool)
        .await
        .inspect_err(|_| {
            tracing::error!("Failed to fetch custom categories for user {}", user_id);
        })?;

    tracing::info!(
        "Successfully fetched {} custom categories for user {}",
        categories.len(),
        user_id
    );
    Ok(Json(categories))
}

/// Creates a new custom category for the authenticated user
///
/// # Arguments
///
/// * `user_id` - The UUID of the authenticated user (from auth middleware)
/// * `state` - Shared application state
/// * `create_category` - The custom category creation data
///
/// # Returns
///
/// * `Ok(Json<CustomCategory>)` - The created custom category as JSON
/// * `Err(Error)` - Database operation error
async fn create_category(
    Extension(user_id): Extension<Uuid>,
    State(state): State<Arc<AppState>>,
    Json(create_category): Json<CreateCustomCategory>,
) -> Result<impl IntoResponse, Error> {
    tracing::info!("POST /category - Creating custom category for user {}", user_id);
    tracing::debug!("Create payload: name={}", create_category.name);

    let pool = state.get_database_pool();

    let category = database::custom_category::create(user_id, create_category, pool)
        .await
        .inspect_err(|_| {
            tracing::error!("Failed to create custom category for user {}", user_id);
        })?;

    tracing::info!(
        "Successfully created custom category {} for user {}",
        category.id,
        user_id
    );
    Ok((StatusCode::CREATED, Json(category)))
}

/// Updates a custom category
///
/// # Arguments
///
/// * `user_id` - The UUID of the authenticated user (from auth middleware)
/// * `category_id` - The UUID of the custom category to update
/// * `state` - Shared application state
/// * `update_category` - The custom category update data
///
/// # Returns
///
/// * `Ok(Json<CustomCategory>)` - The updated custom category as JSON
/// * `Err(Error)` - Database operation error
async fn update_category(
    Extension(user_id): Extension<Uuid>,
    Path(category_id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
    Json(update_category): Json<UpdateCustomCategory>,
) -> Result<impl IntoResponse, Error> {
    tracing::info!(
        "PUT /category/{} - Updating custom category for user {}",
        category_id,
        user_id
    );
    tracing::debug!("Update payload: name={}", update_category.name);

    let pool = state.get_database_pool();

    let category = database::custom_category::update(category_id, user_id, update_category, pool)
        .await
        .inspect_err(|_| {
            tracing::error!(
                "Failed to update custom category {} for user {}",
                category_id,
                user_id
            );
        })?;

    tracing::info!(
        "Successfully updated custom category {} for user {}",
        category_id,
        user_id
    );
    Ok(Json(category))
}

/// Deletes a custom category
///
/// # Arguments
///
/// * `user_id` - The UUID of the authenticated user (from auth middleware)
/// * `category_id` - The UUID of the custom category to delete
/// * `state` - Shared application state
///
/// # Returns
///
/// * `Ok(StatusCode::NO_CONTENT)` - If deletion succeeds
/// * `Err(Error)` - Database operation error
async fn delete_category(
    Extension(user_id): Extension<Uuid>,
    Path(category_id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    tracing::info!(
        "DELETE /category/{} - Deleting custom category for user {}",
        category_id,
        user_id
    );

    let pool = state.get_database_pool();

    database::custom_category::delete(category_id, user_id, pool)
        .await
        .inspect_err(|_| {
            tracing::error!(
                "Failed to delete custom category {} for user {}",
                category_id,
                user_id
            );
        })?;

    tracing::info!(
        "Successfully deleted custom category {} for user {}",
        category_id,
        user_id
    );
    Ok(StatusCode::NO_CONTENT)
}
