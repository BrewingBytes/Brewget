use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use diesel::{
    BoolExpressionMethods, ExpressionMethods, SelectableHelper,
    query_dsl::methods::{FilterDsl, LimitDsl, SelectDsl},
};
use diesel_async::RunQueryDsl;

use crate::{
    AppState,
    models::{
        request::register_info::RegisterInfo,
        response::{error::Error, message::Message},
        user::{NewUser, User},
    },
    schema::users::dsl::*,
};

pub async fn register_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<RegisterInfo>,
) -> Result<impl IntoResponse, Error> {
    if body.username.len() <= 3 {
        return Err((
            StatusCode::BAD_REQUEST,
            "Username length cannot be less or equal to 3 characters.",
        )
            .into());
    }

    if body.password.len() <= 7 {
        return Err((
            StatusCode::BAD_REQUEST,
            "Password length cannot be less or equal to 7 characters.",
        )
            .into());
    }

    if !email_address::EmailAddress::is_valid(&body.email) {
        return Err((StatusCode::BAD_REQUEST, "Email address is not valid.").into());
    }

    let user_res: Vec<User> = users
        .filter(
            username
                .eq(body.username.clone())
                .or(email.eq(body.email.clone())),
        )
        .limit(1)
        .select(User::as_select())
        .load(&mut state.db.get().await?)
        .await?;

    if user_res.len() == 1 {
        return Err((
            StatusCode::BAD_REQUEST,
            "Username or email is already used.",
        )
            .into());
    }

    diesel::insert_into(users)
        .values(NewUser::new(
            body.username,
            body.password,
            body.email,
            &state.config.salt_str,
        ))
        .execute(&mut state.db.get().await?)
        .await?;

    Ok(Json(Message {
        message: "Account has been created.".into(),
    }))
}
