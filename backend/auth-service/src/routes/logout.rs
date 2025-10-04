use std::{str::FromStr, sync::Arc};

use crate::{
    AppState, database,
    models::response::{Error, Message},
    routes::middlewares::auth_guard::auth_guard,
};
use axum::{
    Extension, Json, Router, extract::State, middleware, response::IntoResponse, routing::get,
};

use uuid::Uuid;

/// Creates a router for the logout routes
pub fn get_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/",
            get(logout_handler)
                .route_layer(middleware::from_fn_with_state(state.clone(), auth_guard)),
        )
        .with_state(state)
}

/// Handles user logout requests
///
/// Invalidates user's JWT tokens by removing them from the database
///
/// # Flow
/// 1. Extracts user ID from request extensions (set by auth middleware)
/// 2. Deletes all tokens associated with the user
/// 3. Returns success message
///
/// # Arguments
/// * `state` - Application state containing DB connection
/// * `user_uuid` - User ID from auth middleware
///
/// # Returns
/// * `Ok(Json<Message>)` - Success message on logout
/// * `Err(Error)` - Database errors
///
/// # Example Response
/// ```json
/// {
///     "message": "Ok"
/// }
/// ```
async fn logout_handler(
    State(state): State<Arc<AppState>>,
    Extension(user_uuid): Extension<String>,
) -> Result<impl IntoResponse, Error> {
    // Delete all tokens for the user
    let conn = &mut state.get_database_connection().await?;
    database::tokens::delete_by_uuid(Uuid::from_str(&user_uuid)?, conn).await?;

    // Return success message
    Ok(Json(Message {
        message: "Ok".into(),
    }))
}
