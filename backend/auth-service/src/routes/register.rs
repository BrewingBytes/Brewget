use std::sync::Arc;

use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};

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
    Json(body): Json<RegisterInfo>,
) -> Result<impl IntoResponse, Error> {
    // Verify captcha token
    crate::utils::captcha::verify_turnstile(&body.captcha_token, &state.config.turnstile_secret)
        .await
        .map_err(|_| -> Error {
            (StatusCode::BAD_REQUEST, "Captcha verification failed.").into()
        })?;

    // Validate username length
    if body.username.len() <= 3 {
        return Err((
            StatusCode::BAD_REQUEST,
            "Username length cannot be less or equal to 3 characters.",
        )
            .into());
    }

    // Validate password length
    validate_password(&body.password)
        .map_err(|s| -> Error { (StatusCode::BAD_REQUEST, s.as_str()).into() })?;

    // Validate email format
    if !email_address::EmailAddress::is_valid(&body.email) {
        return Err((StatusCode::BAD_REQUEST, "Email address is not valid.").into());
    }

    // Check for existing username or email
    let pool = state.get_database_pool();
    if database::users::filter_by_username_or_email(&body.username, &body.email, pool)
        .await
        .is_ok()
    {
        return Err((
            StatusCode::BAD_REQUEST,
            "Username or email is already used.",
        )
            .into());
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

    // Store the password hash for the initial password history entry
    let user_uuid = new_user.get_uuid();
    let password_hash = new_user.get_password_hash();

    // Create new activation link
    let new_activation_link = NewActivationLink::new(user_uuid);
    let link = new_activation_link.get_link(&state.config);

    database::users::insert(new_user, pool).await?;
    database::activation_links::insert(new_activation_link, pool).await?;

    // Store initial password in history
    database::password_history::insert(user_uuid, password_hash, pool).await?;

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
    Ok(Json(Message {
        message: "Account has been created.".into(),
    }))
}
