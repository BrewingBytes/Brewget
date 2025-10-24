use std::sync::Arc;

use axum::{Json, Router, extract::State, response::IntoResponse, routing::post};

use crate::{
    AppState, database,
    grpc::email_service::service::ForgotPasswordRequest,
    models::{
        forgot_password_link::NewForgotPasswordLink,
        request::forgot_password_info::ForgotPasswordInfo,
        response::{Error, Message},
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
    // Clone necessary data for the async processing
    let email = body.email.clone();
    let state_clone = state.clone();

    tokio::spawn(async move {
        let pool = state_clone.get_database_pool();
        if let Ok(user) = database::users::filter_by_email(&email, pool).await {
            let new_forgot_password_link = NewForgotPasswordLink::new(user.get_uuid());
            if database::forgot_password_links::insert(new_forgot_password_link.clone(), pool)
                .await
                .is_ok()
            {
                // Prepare and send email
                let request = ForgotPasswordRequest {
                    username: user.get_username(),
                    email: user.get_email(),
                    link: new_forgot_password_link.get_link(&state_clone.config),
                };

                if let Err(e) = state_clone.send_forgot_password(request).await {
                    println!("Failed to send forgot password email: {}", e);
                }
            }
        }
    });

    Ok(Json(Message {
        message: "Forgot password link sent to the email, if an account is registered and verified with that email.".into(),
    }))
}
