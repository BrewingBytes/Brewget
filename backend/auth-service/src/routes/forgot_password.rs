use std::sync::Arc;

use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};

use crate::{
    AppState, database,
    grpc::email_service::service::ForgotPasswordRequest,
    models::{
        forgot_password_link::NewForgotPasswordLink,
        request::forgot_password_info::ForgotPasswordInfo,
        response::{Error, TranslationKey, TranslationKeyMessage},
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
/// JSON response with translation key "FORGOT_PASSWORD_LINK_SENT"
///
/// # Example Response
/// ```json
/// {
///     "translation_key": "FORGOT_PASSWORD_LINK_SENT"
/// }
/// ```
async fn forgot_password_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<ForgotPasswordInfo>,
) -> Result<impl IntoResponse, Error> {
    tracing::info!("Forgot password request for email: {}", body.email);

    // Verify captcha token
    tracing::debug!("Verifying captcha token for forgot password");
    crate::utils::captcha::verify_turnstile(&body.captcha_token, &state.config.turnstile_secret)
        .await
        .map_err(|_| -> Error {
            tracing::warn!(
                "Captcha verification failed for forgot password: {}",
                body.email
            );
            (
                StatusCode::BAD_REQUEST,
                TranslationKey::CaptchaVerificationFailed,
            )
                .into()
        })?;

    // Clone necessary data for the async processing
    let email = body.email.clone();
    let state_clone = state.clone();

    tokio::spawn(async move {
        let pool = state_clone.get_database_pool();
        if let Ok(user) = database::users::filter_by_email(&email, pool).await {
            tracing::debug!("User found for forgot password: {}", user.get_uuid());
            let new_forgot_password_link = NewForgotPasswordLink::new(user.get_uuid());
            if database::forgot_password_links::insert(new_forgot_password_link.clone(), pool)
                .await
                .is_ok()
            {
                tracing::debug!("Forgot password link created, sending email to: {}", email);
                // Prepare and send email
                let request = ForgotPasswordRequest {
                    username: user.get_username(),
                    email: user.get_email(),
                    link: new_forgot_password_link.get_link(&state_clone.config),
                };

                if let Err(e) = state_clone.send_forgot_password(request).await {
                    tracing::error!("Failed to send forgot password email: {}", e);
                } else {
                    tracing::info!("Forgot password email sent successfully to: {}", email);
                }
            } else {
                tracing::error!(
                    "Failed to insert forgot password link for user: {}",
                    user.get_uuid()
                );
            }
        } else {
            tracing::debug!("No user found for forgot password email: {}", email);
        }
    });

    Ok(Json(TranslationKeyMessage {
        translation_key: TranslationKey::ForgotPasswordLinkSent,
    }))
}
