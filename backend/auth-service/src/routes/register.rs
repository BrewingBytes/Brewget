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

/// Handles new user registration requests
///
/// Creates new user accounts after validating registration information
///
/// # Flow
/// 1. Validates username length (> 3 chars)
/// 2. Validates password strength (> 7 chars)
/// 3. Validates email format
/// 4. Checks for existing username/email
/// 5. Creates new user record
/// 6. Returns success message
///
/// # Arguments
/// * `state` - Application state containing config and DB connection
/// * `body` - JSON request body containing registration information
///
/// # Returns
/// * `Ok(Json<Message>)` - Success message on account creation
/// * `Err(Error)` - Validation or database errors
///
/// # Example Request
/// ```json
/// {
///     "username": "newuser",
///     "password": "password123",
///     "email": "user@example.com"
/// }
/// ```
///
/// # Example Response
/// ```json
/// {
///     "message": "Account has been created."
/// }
/// ```
pub async fn register_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<RegisterInfo>,
) -> Result<impl IntoResponse, Error> {
    // Validate username length
    if body.username.len() <= 3 {
        return Err((
            StatusCode::BAD_REQUEST,
            "Username length cannot be less or equal to 3 characters.",
        )
            .into());
    }

    // Validate password length
    if body.password.len() <= 7 {
        return Err((
            StatusCode::BAD_REQUEST,
            "Password length cannot be less or equal to 7 characters.",
        )
            .into());
    }

    // Validate email format
    if !email_address::EmailAddress::is_valid(&body.email) {
        return Err((StatusCode::BAD_REQUEST, "Email address is not valid.").into());
    }

    // Check for existing username or email
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

    // Create new user record
    let new_user =
        NewUser::new(&body.username, &body.password, &body.email).map_err(|_| -> Error {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Could not create account.",
            )
                .into()
        })?;

    diesel::insert_into(users)
        .values(new_user)
        .execute(&mut state.db.get().await?)
        .await?;

    // Return success message
    Ok(Json(Message {
        message: "Account has been created.".into(),
    }))
}
