use std::sync::Arc;

use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};

use crate::{
    AppState, database,
    models::{
        request::login_info::LoginInfo,
        response::{Error, Token},
        token::NewToken,
        token_claim::TokenClaim,
    },
};

/// Creates a router for the login routes
pub fn get_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(login_handler))
        .with_state(state)
}

/// Handles user login requests
///
/// Authenticates users and generates JWT tokens for successful logins
///
/// # Flow
/// 1. Validates username/password combination
/// 2. Generates JWT token with user claims
/// 3. Stores token in database
/// 4. Returns token to client
///
/// # Arguments
/// * `state` - Application state containing config and DB connection
/// * `body` - JSON request body containing login credentials
///
/// # Returns
/// * `Ok(Json<Token>)` - JWT token for authenticated user
/// * `Err(Error)` - Authentication or database errors
///
/// # Example Request
/// ```json
/// {
///     "username": "user@example.com",
///     "password": "password123"
/// }
/// ```
///
/// # Example Response
/// ```json
/// {
///     "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
/// }
/// ```
async fn login_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<LoginInfo>,
) -> Result<impl IntoResponse, Error> {
    tracing::info!("Login attempt for username: {}", body.username);

    // Verify captcha token
    tracing::debug!("Verifying captcha token");
    crate::utils::captcha::verify_turnstile(&body.captcha_token, &state.config.turnstile_secret)
        .await
        .map_err(|_| -> Error {
            tracing::warn!(
                "Captcha verification failed for username: {}",
                body.username
            );
            (StatusCode::BAD_REQUEST, "Captcha verification failed.").into()
        })?;

    // Query database for user with matching username
    let pool = state.get_database_pool();
    tracing::debug!("Querying database for username: {}", body.username);
    let user = database::users::filter_by_username(&body.username, pool).await?;

    // Check if user has activated his account (check before password validation)
    if !user.is_account_verified() {
        tracing::warn!(
            "Unverified account login attempt for username: {}",
            body.username
        );
        return Err((
            StatusCode::UNAUTHORIZED,
            "Email has not been verified, please check your inbox.",
        )
            .into());
    }

    // Validate user exists and password matches
    if !user.is_password_valid(&body.password) {
        tracing::warn!("Invalid password for username: {}", body.username);
        return Err((StatusCode::BAD_REQUEST, "Username or password is invalid.").into());
    }

    // Check if the account is deleted temporarily
    if !user.is_account_active() {
        tracing::warn!(
            "Inactive account login attempt for username: {}",
            body.username
        );
        return Err((
            StatusCode::UNAUTHORIZED,
            "Account has been deleted temporarily",
        )
            .into());
    }

    // Generate token timestamps
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::seconds(state.config.jwt_max_age.into())).timestamp() as usize;

    // Create token claims
    let claims = TokenClaim {
        sub: user.get_uuid().to_string().into(),
        exp,
        iat,
    };

    // Generate JWT token
    tracing::debug!("Generating JWT token for user: {}", user.get_uuid());
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.config.jwt_secret.as_ref()),
    )?;

    // Store token into database
    let new_token = NewToken::new(&user, &token, None, None);
    database::tokens::insert(new_token, pool).await?;

    tracing::info!(
        "Login successful for username: {}, user_id: {}",
        body.username,
        user.get_uuid()
    );
    // Return token to client
    Ok(Json(Token { token }))
}
