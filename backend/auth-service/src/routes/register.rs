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
    tracing::info!(
        "Registration attempt for username: {}, email: {}",
        body.username,
        body.email
    );

    // Verify captcha token
    tracing::debug!("Verifying captcha token for registration");
    crate::utils::captcha::verify_turnstile(&body.captcha_token, &state.config.turnstile_secret)
        .await
        .map_err(|_| -> Error {
            tracing::warn!(
                "Captcha verification failed for registration: {}",
                body.username
            );
            (StatusCode::BAD_REQUEST, "Captcha verification failed.").into()
        })?;

    // Validate username length
    if body.username.len() <= 3 {
        tracing::warn!("Username too short for registration: {}", body.username);
        return Err((
            StatusCode::BAD_REQUEST,
            "Username length cannot be less or equal to 3 characters.",
        )
            .into());
    }

    // Validate password length
    validate_password(&body.password).map_err(|s| -> Error {
        tracing::warn!(
            "Invalid password format for registration: {}",
            body.username
        );
        (StatusCode::BAD_REQUEST, s.as_str()).into()
    })?;

    // Validate email format
    if !email_address::EmailAddress::is_valid(&body.email) {
        tracing::warn!("Invalid email format for registration: {}", body.email);
        return Err((StatusCode::BAD_REQUEST, "Email address is not valid.").into());
    }

    // Check for existing username or email
    let pool = state.get_database_pool();
    tracing::debug!("Checking for existing username or email");
    if database::users::filter_by_username_or_email(&body.username, &body.email, pool)
        .await
        .is_ok()
    {
        tracing::warn!(
            "Username or email already exists: {}, {}",
            body.username,
            body.email
        );
        return Err((
            StatusCode::BAD_REQUEST,
            "Username or email is already used.",
        )
            .into());
    }

    // Create new user record
    tracing::debug!("Creating new user record for: {}", body.username);
    let new_user =
        NewUser::new(&body.username, &body.password, &body.email).map_err(|_| -> Error {
            tracing::error!("Failed to create user record for: {}", body.username);
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

    // Use a transaction to ensure atomicity of user creation, activation link, and password history
    let mut tx = pool.begin().await.map_err(|_| -> Error {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database transaction error.",
        )
            .into()
    })?;

    database::users::insert(new_user, &mut *tx).await?;
    database::activation_links::insert(new_activation_link, &mut *tx).await?;

    // Store initial password in history
    database::password_history::insert(user_uuid, password_hash, &mut *tx).await?;

    // Commit the transaction
    tx.commit().await.map_err(|_| -> Error {
        tracing::error!(
            "Failed to commit registration transaction for: {}",
            body.username
        );
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to commit transaction.",
        )
            .into()
    })?;

    // Send confirmation email
    tracing::debug!("Sending activation email to: {}", body.email);
    let request = ActivateAccountRequest {
        username: body.username.clone(),
        email: body.email.clone(),
        link,
    };
    if let Err(status) = state.send_activate_account(request).await {
        tracing::error!(
            "Failed to send activation email to: {}, error: {}",
            body.email,
            status.message()
        );
        return Err((StatusCode::INTERNAL_SERVER_ERROR, status.message()).into());
    }

    tracing::info!(
        "Registration successful for username: {}, email: {}",
        body.username,
        body.email
    );
    // Return success message
    Ok(Json(Message {
        message: "Account has been created.".into(),
    }))
}
