use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};

use crate::{
    AppState,
    models::{
        dto::{login_info::LoginInfo, token_response::TokenResponse},
        error::Error,
        token_claim::TokenClaim,
    },
};

pub async fn login_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<LoginInfo>,
) -> Result<impl IntoResponse, Error> {
    // Search for user
    if body.email != "test@test.com" || body.password != "password" {
        return Err((StatusCode::UNAUTHORIZED, "Email or password is incorrect.").into());
    }

    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::seconds(state.config.jwt_max_age.into())).timestamp() as usize;

    let claims = TokenClaim {
        sub: body.email,
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.config.jwt_secret.as_ref()),
    )?;

    // TODO: Save token in the DB

    Ok(Json(TokenResponse::new(&token)))
}
