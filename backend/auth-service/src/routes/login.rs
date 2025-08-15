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

#[allow(clippy::get_first)]
pub async fn login_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<LoginInfo>,
) -> Result<impl IntoResponse, Error> {
    let user_res: Vec<User> = users
        .filter(username.eq(body.username))
        .limit(1)
        .select(User::as_select())
        .load(&mut state.db.get().await?)
        .await?;

    if user_res.len() != 1 || user_res.get(0).unwrap().is_password_valid(&body.password) {
        return Err((StatusCode::BAD_REQUEST, "Username or password is invalid.").into());
    }

    let user = user_res.get(0).unwrap();

    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::seconds(state.config.jwt_max_age.into())).timestamp() as usize;

    let claims = TokenClaim {
        sub: user.get_uuid().to_string().into(),
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.config.jwt_secret.as_ref()),
    )?;

    diesel::insert_into(tokens::table)
        .values(NewToken::new(user, &token, None, None))
        .execute(&mut state.db.get().await?)
        .await?;

    Ok(Json(Token {
        token: token.into(),
    }))
}
