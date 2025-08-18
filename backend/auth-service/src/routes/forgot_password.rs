use std::sync::Arc;

use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};

use crate::{
    AppState, database,
    grpc::email_service::service::ForgotPasswordRequest,
    models::{
        forgot_password_link::NewForgotPasswordLink,
        request::forgot_password_info::ForgotPasswordInfo,
        response::{error::Error, message::Message},
    },
};

/// Creates a router for the forgot password routes
pub fn get_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(forgot_password_handler))
        .with_state(state)
}

/// Forgot password endpoint handler
///
/// # Returns
/// JSON response with "Forgot password link sent to the email, if an account is registered and verified with that email."
///
/// # Example Response
/// ```json
/// {
///     "message": "Forgot password link sent to the email, if an account is registered and verified with that email."
/// }
/// ```
async fn forgot_password_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<ForgotPasswordInfo>,
) -> Result<impl IntoResponse, Error> {
    // Get the user based on the email
    let mut conn = state.get_database_connection().await?;
    if let Ok(user) = database::users::filter_by_email(&body.email, &mut conn).await {
        // Generate a new forgot password link
        let new_forgot_password_link = NewForgotPasswordLink::new(user.get_uuid());
        let request = ForgotPasswordRequest {
            username: user.get_username(),
            email: user.get_email(),
            link: new_forgot_password_link.get_link(),
        };

        // Send the email and save the link into the database
        if let Err(status) = state.send_forgot_password(request).await {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, status.message()).into());
        }

        database::forgot_password_links::insert(new_forgot_password_link, &mut conn).await?;
    };

    Ok(Json(Message {
        message: "Forgot password link sent to the email, if an account is registered and verified with that email.".into(),
    }))
}
