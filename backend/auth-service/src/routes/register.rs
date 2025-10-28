use std::sync::Arc;

use axum::{Json, Router, extract::State, http::{StatusCode, header::ACCEPT_LANGUAGE}, response::IntoResponse, routing::post};
use axum::http::HeaderMap;
use shared_types::i18n;

use crate::{
    AppState, database,
    grpc::email_service::service::ActivateAccountRequest,
    models::{
        activation_link::NewActivationLink,
        request::register_info::RegisterInfo,
        response::{Error, Message},
        user::NewUser,
    },
    utils::password::validate_password,
};

/// Creates a router for the register routes
pub fn get_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(register_handler))
        .with_state(state)
}

/// Handles new user registration requests
///
/// Creates new user accounts after validating registration information
///
/// # Flow
/// 1. Validates username length (> 3 chars)
/// 2. Validates password strength (> 7 chars)
/// 3. Validates email format
/// 4. Checks for existing username/email
/// 5. Creates new user record
/// 6. Returns success message
///
/// # Arguments
/// * `state` - Application state containing config and DB connection
/// * `body` - JSON request body containing registration information
///
/// # Returns
/// * `Ok(Json<Message>)` - Success message on account creation
/// * `Err(Error)` - Validation or database errors
///
/// # Example Request
/// ```json
/// {
///     "username": "newuser",
///     "password": "password123",
///     "email": "user@example.com"
/// }
/// ```
///
/// # Example Response
/// ```json
/// {
///     "message": "Account has been created."
/// }
/// ```
async fn register_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(body): Json<RegisterInfo>,
) -> Result<impl IntoResponse, Error> {
    // Extract language from Accept-Language header
    let lang = i18n::extract_language(
        headers
            .get(ACCEPT_LANGUAGE)
            .and_then(|v| v.to_str().ok()),
    );

    // Validate username length
    if body.username.len() <= 3 {
        let msg = i18n::translate("auth.username_length", &lang);
        return Err((StatusCode::BAD_REQUEST, msg.as_str()).into());
    }

    // Validate password length
    validate_password(&body.password)
        .map_err(|s| -> Error { (StatusCode::BAD_REQUEST, s.as_str()).into() })?;

    // Validate email format
    if !email_address::EmailAddress::is_valid(&body.email) {
        let msg = i18n::translate("auth.email_invalid", &lang);
        return Err((StatusCode::BAD_REQUEST, msg.as_str()).into());
    }

    // Check for existing username or email
    let pool = state.get_database_pool();
    if database::users::filter_by_username_or_email(&body.username, &body.email, pool)
        .await
        .is_ok()
    {
        let msg = i18n::translate("auth.username_email_used", &lang);
        return Err((StatusCode::BAD_REQUEST, msg.as_str()).into());
    }

    // Create new user record
    let new_user =
        NewUser::new(&body.username, &body.password, &body.email).map_err(|_| -> Error {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Could not create account.",
            )
                .into()
        })?;

    // Create new activation link
    let new_activation_link = NewActivationLink::new(new_user.get_uuid());
    let link = new_activation_link.get_link(&state.config);

    database::users::insert(new_user, pool).await?;
    database::activation_links::insert(new_activation_link, pool).await?;

    // Send confirmation email
    let request = ActivateAccountRequest {
        username: body.username,
        email: body.email,
        link,
    };
    if let Err(status) = state.send_activate_account(request).await {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, status.message()).into());
    }

    // Return success message
    let msg = i18n::translate("auth.account_created", &lang);
    Ok(Json(Message {
        message: msg,
    }))
}
