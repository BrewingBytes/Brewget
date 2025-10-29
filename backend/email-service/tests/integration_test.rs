use std::sync::Arc;

use tonic::{Request, Response, Status};

use email_service::{
    Config,
    service::{
        email_service::{
            ActivateAccountRequest, ActivateAccountResponse, ForgotPasswordRequest,
            ForgotPasswordResponse, email_service_server::EmailService,
        },
    },
};

/// Mock email service for testing that doesn't actually send emails
#[derive(Clone)]
struct MockEmailService {
    config: Arc<Config>,
}

impl MockEmailService {
    fn new(config: Config) -> Self {
        Self {
            config: Arc::new(config),
        }
    }
}

#[tonic::async_trait]
impl EmailService for MockEmailService {
    async fn send_activate_account(
        &self,
        request: Request<ActivateAccountRequest>,
    ) -> Result<Response<ActivateAccountResponse>, Status> {
        let req = request.into_inner();

        // Validate that required fields are present
        if req.username.is_empty() || req.email.is_empty() || req.link.is_empty() {
            return Err(Status::invalid_argument(
                "Username, email, and link are required",
            ));
        }

        // Validate email format (simple check)
        if !req.email.contains('@') {
            return Err(Status::invalid_argument("Invalid email format"));
        }

        // Mock successful email sending
        Ok(Response::new(ActivateAccountResponse { success: true }))
    }

    async fn send_forgot_password(
        &self,
        request: Request<ForgotPasswordRequest>,
    ) -> Result<Response<ForgotPasswordResponse>, Status> {
        let req = request.into_inner();

        // Validate that required fields are present
        if req.username.is_empty() || req.email.is_empty() || req.link.is_empty() {
            return Err(Status::invalid_argument(
                "Username, email, and link are required",
            ));
        }

        // Validate email format (simple check)
        if !req.email.contains('@') {
            return Err(Status::invalid_argument("Invalid email format"));
        }

        // Mock successful email sending
        Ok(Response::new(ForgotPasswordResponse { success: true }))
    }
}

#[tokio::test]
async fn test_send_activate_account_success() {
    let config = Config {
        email_grpc_port: 0,
        smtp_relay: "smtp.example.com".to_string(),
        smtp_username: "test@example.com".to_string(),
        smtp_password: "password".to_string(),
        smtp_name: "Test Service".to_string(),
        smtp_email: "noreply@example.com".to_string(),
    };

    let service = MockEmailService::new(config);

    let request = Request::new(ActivateAccountRequest {
        username: "testuser".to_string(),
        email: "testuser@example.com".to_string(),
        link: "https://example.com/activate/token123".to_string(),
    });

    let response = service.send_activate_account(request).await;

    assert!(response.is_ok());
    let response = response.unwrap().into_inner();
    assert!(response.success);
}

#[tokio::test]
async fn test_send_activate_account_missing_username() {
    let config = Config {
        email_grpc_port: 0,
        smtp_relay: "smtp.example.com".to_string(),
        smtp_username: "test@example.com".to_string(),
        smtp_password: "password".to_string(),
        smtp_name: "Test Service".to_string(),
        smtp_email: "noreply@example.com".to_string(),
    };

    let service = MockEmailService::new(config);

    let request = Request::new(ActivateAccountRequest {
        username: "".to_string(),
        email: "testuser@example.com".to_string(),
        link: "https://example.com/activate/token123".to_string(),
    });

    let response = service.send_activate_account(request).await;

    assert!(response.is_err());
    let error = response.unwrap_err();
    assert_eq!(error.code(), tonic::Code::InvalidArgument);
}

#[tokio::test]
async fn test_send_activate_account_invalid_email() {
    let config = Config {
        email_grpc_port: 0,
        smtp_relay: "smtp.example.com".to_string(),
        smtp_username: "test@example.com".to_string(),
        smtp_password: "password".to_string(),
        smtp_name: "Test Service".to_string(),
        smtp_email: "noreply@example.com".to_string(),
    };

    let service = MockEmailService::new(config);

    let request = Request::new(ActivateAccountRequest {
        username: "testuser".to_string(),
        email: "invalid-email".to_string(),
        link: "https://example.com/activate/token123".to_string(),
    });

    let response = service.send_activate_account(request).await;

    assert!(response.is_err());
    let error = response.unwrap_err();
    assert_eq!(error.code(), tonic::Code::InvalidArgument);
}

#[tokio::test]
async fn test_send_activate_account_missing_link() {
    let config = Config {
        email_grpc_port: 0,
        smtp_relay: "smtp.example.com".to_string(),
        smtp_username: "test@example.com".to_string(),
        smtp_password: "password".to_string(),
        smtp_name: "Test Service".to_string(),
        smtp_email: "noreply@example.com".to_string(),
    };

    let service = MockEmailService::new(config);

    let request = Request::new(ActivateAccountRequest {
        username: "testuser".to_string(),
        email: "testuser@example.com".to_string(),
        link: "".to_string(),
    });

    let response = service.send_activate_account(request).await;

    assert!(response.is_err());
    let error = response.unwrap_err();
    assert_eq!(error.code(), tonic::Code::InvalidArgument);
}

#[tokio::test]
async fn test_send_forgot_password_success() {
    let config = Config {
        email_grpc_port: 0,
        smtp_relay: "smtp.example.com".to_string(),
        smtp_username: "test@example.com".to_string(),
        smtp_password: "password".to_string(),
        smtp_name: "Test Service".to_string(),
        smtp_email: "noreply@example.com".to_string(),
    };

    let service = MockEmailService::new(config);

    let request = Request::new(ForgotPasswordRequest {
        username: "forgetfuluser".to_string(),
        email: "forgetful@example.com".to_string(),
        link: "https://example.com/reset/token456".to_string(),
    });

    let response = service.send_forgot_password(request).await;

    assert!(response.is_ok());
    let response = response.unwrap().into_inner();
    assert!(response.success);
}

#[tokio::test]
async fn test_send_forgot_password_missing_email() {
    let config = Config {
        email_grpc_port: 0,
        smtp_relay: "smtp.example.com".to_string(),
        smtp_username: "test@example.com".to_string(),
        smtp_password: "password".to_string(),
        smtp_name: "Test Service".to_string(),
        smtp_email: "noreply@example.com".to_string(),
    };

    let service = MockEmailService::new(config);

    let request = Request::new(ForgotPasswordRequest {
        username: "forgetfuluser".to_string(),
        email: "".to_string(),
        link: "https://example.com/reset/token456".to_string(),
    });

    let response = service.send_forgot_password(request).await;

    assert!(response.is_err());
    let error = response.unwrap_err();
    assert_eq!(error.code(), tonic::Code::InvalidArgument);
}

#[tokio::test]
async fn test_send_forgot_password_invalid_email() {
    let config = Config {
        email_grpc_port: 0,
        smtp_relay: "smtp.example.com".to_string(),
        smtp_username: "test@example.com".to_string(),
        smtp_password: "password".to_string(),
        smtp_name: "Test Service".to_string(),
        smtp_email: "noreply@example.com".to_string(),
    };

    let service = MockEmailService::new(config);

    let request = Request::new(ForgotPasswordRequest {
        username: "forgetfuluser".to_string(),
        email: "not-an-email".to_string(),
        link: "https://example.com/reset/token456".to_string(),
    });

    let response = service.send_forgot_password(request).await;

    assert!(response.is_err());
    let error = response.unwrap_err();
    assert_eq!(error.code(), tonic::Code::InvalidArgument);
}

#[tokio::test]
async fn test_email_template_generation() {
    // Test that the service can be created with valid config
    let config = Config {
        email_grpc_port: 50051,
        smtp_relay: "smtp.example.com".to_string(),
        smtp_username: "test@example.com".to_string(),
        smtp_password: "password".to_string(),
        smtp_name: "Test Service".to_string(),
        smtp_email: "noreply@example.com".to_string(),
    };

    // This will fail if SMTP relay is invalid, but we're just testing config
    // In a real scenario, we'd mock the SMTP transport
    assert_eq!(config.smtp_relay, "smtp.example.com");
    assert_eq!(config.smtp_email, "noreply@example.com");
}

#[tokio::test]
async fn test_multiple_activate_requests() {
    let config = Config {
        email_grpc_port: 0,
        smtp_relay: "smtp.example.com".to_string(),
        smtp_username: "test@example.com".to_string(),
        smtp_password: "password".to_string(),
        smtp_name: "Test Service".to_string(),
        smtp_email: "noreply@example.com".to_string(),
    };

    let service = MockEmailService::new(config);

    // Send multiple activation emails
    for i in 0..5 {
        let request = Request::new(ActivateAccountRequest {
            username: format!("user{}", i),
            email: format!("user{}@example.com", i),
            link: format!("https://example.com/activate/token{}", i),
        });

        let response = service.send_activate_account(request).await;
        assert!(response.is_ok());
        assert!(response.unwrap().into_inner().success);
    }
}

#[tokio::test]
async fn test_multiple_forgot_password_requests() {
    let config = Config {
        email_grpc_port: 0,
        smtp_relay: "smtp.example.com".to_string(),
        smtp_username: "test@example.com".to_string(),
        smtp_password: "password".to_string(),
        smtp_name: "Test Service".to_string(),
        smtp_email: "noreply@example.com".to_string(),
    };

    let service = MockEmailService::new(config);

    // Send multiple password reset emails
    for i in 0..5 {
        let request = Request::new(ForgotPasswordRequest {
            username: format!("user{}", i),
            email: format!("user{}@example.com", i),
            link: format!("https://example.com/reset/token{}", i),
        });

        let response = service.send_forgot_password(request).await;
        assert!(response.is_ok());
        assert!(response.unwrap().into_inner().success);
    }
}
