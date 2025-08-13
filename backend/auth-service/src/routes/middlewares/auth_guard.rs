use std::sync::Arc;

use axum::{
    extract::{Request, State},
    http::{StatusCode, header},
    middleware::Next,
    response::IntoResponse,
};
use jsonwebtoken::{DecodingKey, Validation, decode};

use crate::{
    AppState,
    models::{error::Error, token_claim::TokenClaim, user::User},
};

pub async fn auth_guard(
    State(state): State<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, Error> {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "))
        .ok_or((
            StatusCode::UNAUTHORIZED,
            "You are not logged in, please provide token",
        ))?;

    let token = decode::<TokenClaim>(
        token,
        &DecodingKey::from_secret(state.config.jwt_secret.as_ref()),
        &Validation::default(),
    )?;

    // TODO: Check JWT is in DB
    // let user = state.db.select(("user", token.claims.sub)).await?;
    let user = Some(User {
        email: token.claims.sub,
    });

    let user = user.ok_or((StatusCode::UNAUTHORIZED, "No user matches this token"))?;
    req.extensions_mut().insert(user);

    Ok(next.run(req).await)
}
