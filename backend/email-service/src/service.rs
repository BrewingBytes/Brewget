use std::sync::Arc;

use handlebars::Handlebars;
use lettre::{
    Message, SmtpTransport, Transport, message::MultiPart,
    transport::smtp::authentication::Credentials,
};
use serde_json::json;
use tonic::{Request, Response, Result, Status};

use crate::{
    config::Config,
    service::email_service::{
        ActivateAccountRequest, ActivateAccountResponse, ForgotPasswordRequest,
        ForgotPasswordResponse, email_service_server::EmailService,
    },
};

/// HTML template for account activation emails
const ACTIVATE_ACCOUNT_TEMPLATE: &str = include_str!("../emails/activate_account_template.html");
/// HTML template for password reset emails
const FORGOT_PASSWORD_TEMPLATE: &str = include_str!("../emails/forgot_password_template.html");

/// Protocol Buffers definitions for the email service
pub mod email_service {
    tonic::include_proto!("email_service");
}

/// Email service implementation
///
/// This struct contains the configuration and SMTP transport needed to send emails.
/// It implements the gRPC EmailService trait to handle email sending requests.
///
/// # Fields
///
/// * `config` - Shared configuration containing SMTP settings
/// * `mailer` - SMTP transport for sending emails
pub struct Service {
    config: Arc<Config>,
    mailer: SmtpTransport,
}

impl Service {
    /// Creates a new email service instance
    ///
    /// This function initializes the SMTP transport with the provided configuration
    /// and sets up authentication credentials for sending emails.
    ///
    /// # Arguments
    ///
    /// * `config` - Shared configuration containing SMTP settings
    ///
    /// # Returns
    ///
    /// * `Ok(Service)` - Successfully created email service instance
    /// * `Err(Box<dyn std::error::Error>)` - Error occurred during SMTP setup
    ///
    /// # Errors
    ///
    /// This function can return errors in the following cases:
    /// - Invalid SMTP relay hostname
    /// - SMTP transport configuration failure
    ///
    /// # Example
    ///
    /// ```rust
    /// use email_service::{Config, service::Service};
    /// use std::sync::Arc;
    ///
    /// let config = Arc::new(Config::init());
    /// let service = Service::new(config)?;
    /// ```
    pub fn new(config: Arc<Config>) -> Result<Self, Box<dyn std::error::Error>> {
        let creds = Credentials::new(config.smtp_username.clone(), config.smtp_password.clone());
        let mailer = SmtpTransport::starttls_relay(&config.smtp_relay)?
            .credentials(creds)
            .build();

        Ok(Self { config, mailer })
    }

    /// Creates an account activation email message
    ///
    /// This function generates both plain text and HTML versions of the activation email
    /// using the Handlebars template engine.
    ///
    /// # Arguments
    ///
    /// * `request` - The activation account request containing user details and activation link
    ///
    /// # Returns
    ///
    /// * `Ok(Message)` - Successfully created email message
    /// * `Err(())` - Error occurred during message creation
    async fn create_activate_account_mail(
        &self,
        request: &ActivateAccountRequest,
    ) -> Result<Message, ()> {
        let m = Message::builder()
            .from(
                format!("{} <{}>", self.config.smtp_name, self.config.smtp_email)
                    .parse()
                    .map_err(|_| ())?,
            )
            .to(format!("{} <{}>", request.username, request.email)
                .parse()
                .map_err(|_| ())?)
            .subject("Activate your account");

        let plain = format!(
            "Use the following link to activate your account: {}",
            request.link
        );

        let html = Handlebars::new()
            .render_template(
                ACTIVATE_ACCOUNT_TEMPLATE,
                &json!({"activation_link": request.link}),
            )
            .map_err(|_| ())?;

        m.multipart(MultiPart::alternative_plain_html(plain, html))
            .map_err(|_| ())
    }

    /// Creates a password reset email message
    ///
    /// This function generates both plain text and HTML versions of the password reset email
    /// using the Handlebars template engine.
    ///
    /// # Arguments
    ///
    /// * `request` - The forgot password request containing user details and reset link
    ///
    /// # Returns
    ///
    /// * `Ok(Message)` - Successfully created email message
    /// * `Err(())` - Error occurred during message creation
    async fn create_forgot_password_mail(
        &self,
        request: &ForgotPasswordRequest,
    ) -> Result<Message, ()> {
        let m = Message::builder()
            .from(
                format!("{} <{}>", self.config.smtp_name, self.config.smtp_email)
                    .parse()
                    .map_err(|_| ())?,
            )
            .to(format!("{} <{}>", request.username, request.email)
                .parse()
                .map_err(|_| ())?)
            .subject("Reset your password");

        let plain = format!(
            "Use the following link to reset your password: {}",
            request.link
        );

        let html = Handlebars::new()
            .render_template(
                FORGOT_PASSWORD_TEMPLATE,
                &json!({"forgot_password_link": request.link}),
            )
            .map_err(|_| ())?;

        m.multipart(MultiPart::alternative_plain_html(plain, html))
            .map_err(|_| ())
    }

    /// Sends an email using the configured SMTP transport
    ///
    /// This function uses the pre-configured SMTP transport to send the provided email message.
    ///
    /// # Arguments
    ///
    /// * `message` - The email message to send
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Email sent successfully
    /// * `Err(())` - Error occurred during email sending
    fn send_email(&self, message: Message) -> Result<(), ()> {
        self.mailer.send(&message).map_err(|_| ()).map(|_| ())
    }
}

#[tonic::async_trait]
impl EmailService for Service {
    /// Sends an account activation email
    ///
    /// This gRPC endpoint handles requests to send account activation emails to users.
    /// It creates an HTML email with an activation link and sends it via SMTP.
    ///
    /// # Arguments
    ///
    /// * `request` - gRPC request containing activation account details
    ///
    /// # Returns
    ///
    /// * `Ok(Response<ActivateAccountResponse>)` - Success response indicating email was sent
    /// * `Err(Status)` - gRPC error status if email sending failed
    ///
    /// # Request Fields
    ///
    /// * `username` - The username of the user
    /// * `email` - The email address to send the activation email to
    /// * `link` - The activation link to include in the email
    ///
    /// # Response Fields
    ///
    /// * `success` - Boolean indicating whether the email was sent successfully
    async fn send_activate_account(
        &self,
        request: Request<ActivateAccountRequest>,
    ) -> Result<Response<ActivateAccountResponse>, Status> {
        let message = self
            .create_activate_account_mail(&request.into_inner())
            .await
            .map_err(|_| Status::internal("Could not create email."))?;

        self.send_email(message)
            .map_err(|_| Status::internal("Could not send email."))?;

        let reply = ActivateAccountResponse { success: true };
        Ok(Response::new(reply))
    }

    /// Sends a password reset email
    ///
    /// This gRPC endpoint handles requests to send password reset emails to users.
    /// It creates an HTML email with a password reset link and sends it via SMTP.
    ///
    /// # Arguments
    ///
    /// * `request` - gRPC request containing forgot password details
    ///
    /// # Returns
    ///
    /// * `Ok(Response<ForgotPasswordResponse>)` - Success response indicating email was sent
    /// * `Err(Status)` - gRPC error status if email sending failed
    ///
    /// # Request Fields
    ///
    /// * `username` - The username of the user
    /// * `email` - The email address to send the password reset email to
    /// * `link` - The password reset link to include in the email
    ///
    /// # Response Fields
    ///
    /// * `success` - Boolean indicating whether the email was sent successfully
    async fn send_forgot_password(
        &self,
        request: Request<ForgotPasswordRequest>,
    ) -> Result<Response<ForgotPasswordResponse>, Status> {
        let message = self
            .create_forgot_password_mail(&request.into_inner())
            .await
            .map_err(|_| Status::internal("Could not create email."))?;

        self.send_email(message)
            .map_err(|_| Status::internal("Could not send email."))?;

        let reply = ForgotPasswordResponse { success: true };
        Ok(Response::new(reply))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use handlebars::Handlebars;

    #[test]
    fn test_activate_account_template_renders() {
        let handlebars = Handlebars::new();
        let activation_link = "https://example.com/activate?token=abc123";
        
        let result = handlebars.render_template(
            ACTIVATE_ACCOUNT_TEMPLATE,
            &json!({"activation_link": activation_link}),
        );
        
        assert!(result.is_ok());
        let html = result.unwrap();
        // Check that the template rendered successfully and contains the domain
        assert!(html.contains("example.com"));
        assert!(html.contains("Activate account"));
    }

    #[test]
    fn test_forgot_password_template_renders() {
        let handlebars = Handlebars::new();
        let reset_link = "https://example.com/reset?token=xyz789";
        
        let result = handlebars.render_template(
            FORGOT_PASSWORD_TEMPLATE,
            &json!({"forgot_password_link": reset_link}),
        );
        
        assert!(result.is_ok());
        let html = result.unwrap();
        // Check that the template rendered successfully and contains the domain
        assert!(html.contains("example.com"));
        assert!(html.contains("Reset password"));
    }

    #[test]
    fn test_activate_account_template_empty_link() {
        let handlebars = Handlebars::new();
        let empty_link = "";
        
        let result = handlebars.render_template(
            ACTIVATE_ACCOUNT_TEMPLATE,
            &json!({"activation_link": empty_link}),
        );
        
        // Template should still render even with empty link
        assert!(result.is_ok());
    }

    #[test]
    fn test_forgot_password_template_special_characters() {
        let handlebars = Handlebars::new();
        // Test with URL that has special characters
        let reset_link = "https://example.com/reset?token=xyz&user=test%40example.com";
        
        let result = handlebars.render_template(
            FORGOT_PASSWORD_TEMPLATE,
            &json!({"forgot_password_link": reset_link}),
        );
        
        assert!(result.is_ok());
        let html = result.unwrap();
        // The link should be properly escaped in HTML
        assert!(html.contains("example.com"));
    }

    #[test]
    fn test_activate_account_template_valid_html() {
        let handlebars = Handlebars::new();
        let activation_link = "https://brewget.com/activate?token=test123";
        
        let result = handlebars.render_template(
            ACTIVATE_ACCOUNT_TEMPLATE,
            &json!({"activation_link": activation_link}),
        );
        
        assert!(result.is_ok());
        let html = result.unwrap();
        // Check for basic HTML structure
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("<html"));
        assert!(html.contains("</html>"));
    }

    #[test]
    fn test_forgot_password_template_valid_html() {
        let handlebars = Handlebars::new();
        let reset_link = "https://brewget.com/reset?token=test456";
        
        let result = handlebars.render_template(
            FORGOT_PASSWORD_TEMPLATE,
            &json!({"forgot_password_link": reset_link}),
        );
        
        assert!(result.is_ok());
        let html = result.unwrap();
        // Check for basic HTML structure
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("<html"));
        assert!(html.contains("</html>"));
    }
}
