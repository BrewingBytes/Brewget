use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use chrono::{Duration, Utc};
use diesel::{
    ExpressionMethods, SelectableHelper,
    query_dsl::methods::{FilterDsl, LimitDsl, SelectDsl},
};
use diesel_async::RunQueryDsl;
use jsonwebtoken::{EncodingKey, Header, encode};

use crate::{
    AppState,
    models::{
        request::login_info::LoginInfo,
        response::{error::Error, token::Token},
        token::NewToken,
        token_claim::TokenClaim,
        user::User,
    },
    schema::{tokens, users::dsl::*},
};

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
#[allow(clippy::get_first)]
pub async fn login_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<LoginInfo>,
) -> Result<impl IntoResponse, Error> {
    // Query database for user with matching username
    let user_res: Vec<User> = users
        .filter(username.eq(body.username))
        .limit(1)
        .select(User::as_select())
        .load(&mut state.db.get().await?)
        .await?;

    // Validate user exists and password matches
    if user_res.len() != 1 || user_res.get(0).unwrap().is_password_valid(&body.password) {
        return Err((StatusCode::BAD_REQUEST, "Username or password is invalid.").into());
    }

    let user = user_res.get(0).unwrap();

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
    diesel::insert_into(tokens::table)
        .values(NewToken::new(user, &token, None, None))
        .execute(&mut state.db.get().await?)
        .await?;

    // Return token to client
    Ok(Json(Token {
        token: token.into(),
    }))
}
