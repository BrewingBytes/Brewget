use std::sync::Arc;

use axum::{
    extract::{Request, State},
    http::{StatusCode, header},
    middleware::Next,
    response::IntoResponse,
};
use diesel::{
    ExpressionMethods, SelectableHelper,
    query_dsl::methods::{FilterDsl, LimitDsl, SelectDsl},
};
use diesel_async::RunQueryDsl;
use jsonwebtoken::{DecodingKey, Validation, decode};

use crate::{
    AppState,
    models::{response::error::Error, token::Token, token_claim::TokenClaim},
    schema::tokens::dsl::*,
};

#[allow(clippy::get_first)]
pub async fn auth_guard(
    State(state): State<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, Error> {
    let received_token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "))
        .ok_or((
            StatusCode::UNAUTHORIZED,
            "You are not logged in, please provide token",
        ))?;

    let decoded_token = decode::<TokenClaim>(
        received_token,
        &DecodingKey::from_secret(state.config.jwt_secret.as_ref()),
        &Validation::default(),
    )?;

    let token_res = tokens
        .filter(token.eq(received_token))
        .limit(1)
        .select(Token::as_select())
        .load(&mut state.db.get().await?)
        .await?;

    if token_res.len() != 1 || token_res.get(0).unwrap().is_expired() {
        return Err((StatusCode::UNAUTHORIZED, "Token has expired").into());
    }

    if token_res.get(0).unwrap().get_uuid().to_string() != *decoded_token.claims.sub {
        return Err((StatusCode::UNAUTHORIZED, "Token is invalid").into());
    }

    req.extensions_mut()
        .insert(decoded_token.claims.sub.to_string());
    Ok(next.run(req).await)
}
