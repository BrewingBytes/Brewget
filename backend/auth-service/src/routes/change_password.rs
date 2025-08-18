use std::sync::Arc;

use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};

use crate::{
    AppState, database,
    models::{
        request::reset_password_info::ResetPasswordInfo,
        response::{error::Error, message::Message},
    },
    utils::password::{hash_password, validate_password},
};

/// Creates a router for the change password routes
pub fn get_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(change_password_handler))
        .with_state(state)
}

/// Change password endpoint handler
///
/// # Returns
/// JSON response with "Password sucessfully changed." if the password was changed.
///
/// # Example Response
/// ```json
/// {
///     "message": "Password sucessfully changed."
/// }
/// ```
async fn change_password_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<ResetPasswordInfo>,
) -> Result<impl IntoResponse, Error> {
    // Get the forgot password link from the db
    let mut conn = state.get_database_connection().await?;
    let link = database::forgot_password_links::filter_by_id(body.id, &mut conn).await?;

    // If the link is expired, remove it from the database and send a BAD_REQUEST
    if link.is_expired() {
        database::forgot_password_links::delete(body.id, &mut conn).await?;
        return Err((StatusCode::BAD_REQUEST, "Link is expired.").into());
    }

    // Check if the password is ok and hash it
    validate_password(&body.password)?;
    let new_hashed_password = hash_password(&body.password).map_err(|_| -> Error {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong, please try again!",
        )
            .into()
    })?;

    // Change the password of the user
    database::users::change_password(link.get_uuid(), new_hashed_password, &mut conn).await?;

    // Delete the forgot password link from the db
    if database::forgot_password_links::delete(body.id, &mut conn).await? != 1 {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Could not delete from the database.",
        )
            .into());
    }

    Ok(Json(Message {
        message: "Password sucessfully changed.".into(),
    }))
}
