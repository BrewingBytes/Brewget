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
        tracing::debug!("Received token verification request");

        // Try to decode and validate the JWT
        let decoded_token = match decode::<TokenClaim>(
            &token,
            &DecodingKey::from_secret(self.state.config.jwt_secret.as_ref()),
            &Validation::default(),
        ) {
            Ok(token) => {
                tracing::debug!("JWT decoded successfully, subject: {}", token.claims.sub);
                token
            }
            Err(e) => {
                tracing::warn!("Failed to decode JWT token: {}", e);
                // Invalid token format or signature
                return Ok(Response::new(VerifyTokenResponse { user_id: None }));
            }
        };

        // Check if token exists in database and is not expired
        let pool = self.state.get_database_pool();

        let token_res = match database::tokens::find(&token, pool).await {
            Ok(token) => {
                tracing::debug!("Token found in database for user: {}", token.get_uuid());
                token
            }
            Err(_) => {
                tracing::warn!("Token not found in database");
                // Token not found in database
                return Ok(Response::new(VerifyTokenResponse { user_id: None }));
            }
        };

        // Check if token is expired
        if token_res.is_expired() {
            tracing::info!("Token expired for user: {}, cleaning up", token_res.get_uuid());
            // Clean up expired token
            let _ = database::tokens::delete_by_token(token_res.get_token(), pool).await;
            return Ok(Response::new(VerifyTokenResponse { user_id: None }));
        }

        // Verify token belongs to the correct user
        if token_res.get_uuid().to_string() != *decoded_token.claims.sub {
            tracing::warn!(
                "Token user mismatch. Database: {}, JWT: {}",
                token_res.get_uuid(),
                decoded_token.claims.sub
            );
            return Ok(Response::new(VerifyTokenResponse { user_id: None }));
        }

        // Token is valid, return user ID
        let user_id = decoded_token.claims.sub.to_string();
        tracing::info!("Token verified successfully for user: {}", user_id);
        Ok(Response::new(VerifyTokenResponse {
            user_id: Some(user_id),
        }))
    }
}
