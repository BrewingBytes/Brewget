use std::sync::Arc;

use axum::{Extension, Json, extract::State, response::IntoResponse};
use diesel_async::RunQueryDsl;

use crate::{
    AppState,
    models::response::{error::Error, message::Message},
    schema::tokens::{self},
};

pub async fn logout_handler(
    State(state): State<Arc<AppState>>,
    Extension(user_uuid): Extension<String>,
) -> Result<impl IntoResponse, Error> {
    println!("User {} has been logged out.", user_uuid);

    diesel::delete(tokens::table)
        .execute(&mut state.db.get().await?)
        .await?;

    Ok(Json(Message {
        message: "Ok".into(),
    }))
}
