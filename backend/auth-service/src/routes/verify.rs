use std::sync::Arc;

use crate::{
    AppState,
    models::response::{Error, TranslationKey, TranslationKeyMessage},
    routes::middlewares::auth_guard::auth_guard,
};
use axum::{Extension, Json, Router, middleware, response::IntoResponse, routing::get};

/// Creates a router for the verify token routes
pub fn get_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/",
            get(verify_handler)
                .route_layer(middleware::from_fn_with_state(state.clone(), auth_guard)),
        )
        .with_state(state)
}

/// Handles token verification requests
///
/// Verifies that the JWT token is valid and not expired
///
/// # Flow
/// 1. Token is validated by auth_guard middleware
/// 2. If middleware passes, token is valid
/// 3. Returns OK message
///
/// # Arguments
/// * `user_uuid` - User ID from auth middleware (proves token is valid)
///
/// # Returns
/// * `Ok(Json<TranslationKeyMessage>)` - Success message if token is valid
/// * `Err(Error)` - Token is invalid or expired (handled by middleware)
///
/// # Example Response
/// ```json
/// {
///     "translation_key": "OK"
/// }
/// ```
async fn verify_handler(
    Extension(_user_uuid): Extension<String>,
) -> Result<impl IntoResponse, Error> {
    tracing::debug!("Token verification successful");

    // If we reached here, the auth_guard middleware has already validated the token
    Ok(Json(TranslationKeyMessage {
        translation_key: TranslationKey::Ok,
    }))
}
