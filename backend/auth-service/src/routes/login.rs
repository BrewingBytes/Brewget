use std::sync::Arc;

use axum::{Json, Router, extract::State, http::{StatusCode, header::ACCEPT_LANGUAGE}, response::IntoResponse, routing::post};
use axum::http::HeaderMap;
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use shared_types::i18n;

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
    headers: HeaderMap,
    Json(body): Json<LoginInfo>,
) -> Result<impl IntoResponse, Error> {
    // Extract language from Accept-Language header
    let lang = i18n::extract_language(
        headers
            .get(ACCEPT_LANGUAGE)
            .and_then(|v| v.to_str().ok()),
    );

    // Query database for user with matching username
    let pool = state.get_database_pool();
    let user = database::users::filter_by_username(&body.username, pool).await?;

    // Validate user exists and password matches
    if !user.is_password_valid(&body.password) {
        let msg = i18n::translate("auth.username_or_password_invalid", &lang);
        return Err((StatusCode::BAD_REQUEST, msg.as_str()).into());
    }

    // Check if user has activated his account
    if !user.is_account_verified() {
        let msg = i18n::translate("auth.email_not_verified", &lang);
        return Err((StatusCode::UNAUTHORIZED, msg.as_str()).into());
    }

    // Check if the account is deleted temporarily
    if !user.is_account_active() {
        let msg = i18n::translate("auth.account_deleted", &lang);
        return Err((StatusCode::UNAUTHORIZED, msg.as_str()).into());
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
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.config.jwt_secret.as_ref()),
    )?;

    // Store token into database
    let new_token = NewToken::new(&user, &token, None, None);
    database::tokens::insert(new_token, pool).await?;

    // Return token to client
    Ok(Json(Token { token }))
}
