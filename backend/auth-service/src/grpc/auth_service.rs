pub mod service {
    tonic::include_proto!("auth_service");
}

use std::sync::Arc;

use jsonwebtoken::{DecodingKey, Validation, decode};
use tonic::{Request, Response, Status};

use crate::{AppState, database, models::token_claim::TokenClaim};

use service::{VerifyTokenRequest, VerifyTokenResponse, auth_service_server::AuthService};

/// gRPC service for auth operations
pub struct AuthServiceImpl {
    pub state: Arc<AppState>,
}

impl AuthServiceImpl {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }
}

#[tonic::async_trait]
impl AuthService for AuthServiceImpl {
    /// Verifies a JWT token and returns the user ID if valid
    ///
    /// # Arguments
    /// * `request` - gRPC request containing the token to verify
    ///
    /// # Returns
    /// * `Ok(Response<VerifyTokenResponse>)` - Contains user_id if token is valid, None otherwise
    /// * `Err(Status)` - gRPC error if something went wrong
    async fn verify_token(
        &self,
        request: Request<VerifyTokenRequest>,
    ) -> Result<Response<VerifyTokenResponse>, Status> {
        let token = request.into_inner().token;

        // Try to decode and validate the JWT
        let decoded_token = match decode::<TokenClaim>(
            &token,
            &DecodingKey::from_secret(self.state.config.jwt_secret.as_ref()),
            &Validation::default(),
        ) {
            Ok(token) => token,
            Err(_) => {
                // Invalid token format or signature
                return Ok(Response::new(VerifyTokenResponse { user_id: None }));
            }
        };

        // Check if token exists in database and is not expired
        let conn = &mut match self.state.get_database_connection().await {
            Ok(conn) => conn,
            Err(_) => {
                return Err(Status::internal("Database connection failed"));
            }
        };

        let token_res = match database::tokens::find(&token, conn).await {
            Ok(token) => token,
            Err(_) => {
                // Token not found in database
                return Ok(Response::new(VerifyTokenResponse { user_id: None }));
            }
        };

        // Check if token is expired
        if token_res.is_expired() {
            // Clean up expired token
            let _ = database::tokens::delete_by_token(token_res.get_token(), conn).await;
            return Ok(Response::new(VerifyTokenResponse { user_id: None }));
        }

        // Verify token belongs to the correct user
        if token_res.get_uuid().to_string() != *decoded_token.claims.sub {
            return Ok(Response::new(VerifyTokenResponse { user_id: None }));
        }

        // Token is valid, return user ID
        Ok(Response::new(VerifyTokenResponse {
            user_id: Some(decoded_token.claims.sub.to_string()),
        }))
    }
}
