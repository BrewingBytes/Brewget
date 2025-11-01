use std::sync::Arc;

use handlebars::Handlebars;
use lettre::{
    Message, SmtpTransport, Transport, message::MultiPart,
    transport::smtp::authentication::Credentials,
};
use serde_json::json;
use tonic::{Request, Response, Result, Status};
use tracing::{error, info, instrument};

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

/// Email creation and sending errors
#[derive(Debug)]
pub enum EmailError {
    /// Error parsing email address
    AddressParse(String),
    /// Error rendering email template
    TemplateRender(String),
    /// Error creating email message
    MessageBuild(String),
    /// Error sending email via SMTP
    SmtpSend(String),
}

impl std::fmt::Display for EmailError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmailError::AddressParse(msg) => write!(f, "Failed to parse email address: {}", msg),
            EmailError::TemplateRender(msg) => {
                write!(f, "Failed to render email template: {}", msg)
            }
            EmailError::MessageBuild(msg) => write!(f, "Failed to build email message: {}", msg),
            EmailError::SmtpSend(msg) => write!(f, "Failed to send email: {}", msg),
        }
    }
}

impl std::error::Error for EmailError {}

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
    #[instrument(skip(config), fields(smtp_relay = %config.smtp_relay))]
    pub fn new(config: Arc<Config>) -> Result<Self, Box<dyn std::error::Error>> {
        info!("Initializing email service with SMTP configuration");

        let creds = Credentials::new(config.smtp_username.clone(), config.smtp_password.clone());
        let mailer = SmtpTransport::starttls_relay(&config.smtp_relay)
            .map_err(|e| {
                error!(error = %e, smtp_relay = %config.smtp_relay, "Failed to create SMTP transport");
                e
            })?
            .credentials(creds)
            .build();

        info!("Email service initialized successfully");
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
    /// * `Err(EmailError)` - Error occurred during message creation
    #[instrument(skip(self, request), fields(email = %request.email, username = %request.username))]
    async fn create_activate_account_mail(
        &self,
        request: &ActivateAccountRequest,
    ) -> std::result::Result<Message, EmailError> {
        info!("Creating activation account email message");

        let m = Message::builder()
            .from(
                format!("{} <{}>", self.config.smtp_name, self.config.smtp_email)
                    .parse()
                    .map_err(|e| {
                        error!(error = ?e, from_email = %self.config.smtp_email, "Failed to parse 'from' email address");
                        EmailError::AddressParse(format!("Invalid 'from' address: {}", e))
                    })?,
            )
            .to(format!("{} <{}>", request.username, request.email)
                .parse()
                .map_err(|e| {
                    error!(error = ?e, to_email = %request.email, "Failed to parse 'to' email address");
                    EmailError::AddressParse(format!("Invalid 'to' address: {}", e))
                })?)
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
            .map_err(|e| {
                error!(error = %e, "Failed to render activation email template");
                EmailError::TemplateRender(e.to_string())
            })?;

        info!("Successfully created activation account email message");
        m.multipart(MultiPart::alternative_plain_html(plain, html))
            .map_err(|e| {
                error!(error = ?e, "Failed to create multipart email message");
                EmailError::MessageBuild(e.to_string())
            })
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
    /// * `Err(EmailError)` - Error occurred during message creation
    #[instrument(skip(self, request), fields(email = %request.email, username = %request.username))]
    async fn create_forgot_password_mail(
        &self,
        request: &ForgotPasswordRequest,
    ) -> std::result::Result<Message, EmailError> {
        info!("Creating forgot password email message");

        let m = Message::builder()
            .from(
                format!("{} <{}>", self.config.smtp_name, self.config.smtp_email)
                    .parse()
                    .map_err(|e| {
                        error!(error = ?e, from_email = %self.config.smtp_email, "Failed to parse 'from' email address");
                        EmailError::AddressParse(format!("Invalid 'from' address: {}", e))
                    })?,
            )
            .to(format!("{} <{}>", request.username, request.email)
                .parse()
                .map_err(|e| {
                    error!(error = ?e, to_email = %request.email, "Failed to parse 'to' email address");
                    EmailError::AddressParse(format!("Invalid 'to' address: {}", e))
                })?)
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
            .map_err(|e| {
                error!(error = %e, "Failed to render forgot password email template");
                EmailError::TemplateRender(e.to_string())
            })?;

        info!("Successfully created forgot password email message");
        m.multipart(MultiPart::alternative_plain_html(plain, html))
            .map_err(|e| {
                error!(error = ?e, "Failed to create multipart email message");
                EmailError::MessageBuild(e.to_string())
            })
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
    /// * `Err(EmailError)` - Error occurred during email sending
    #[instrument(skip(self, message), fields(subject = ?message.headers().get_raw("Subject")))]
    fn send_email(&self, message: Message) -> std::result::Result<(), EmailError> {
        info!("Sending email via SMTP");

        self.mailer
            .send(&message)
            .map_err(|e| {
                error!(error = %e, "Failed to send email via SMTP transport");
                EmailError::SmtpSend(e.to_string())
            })
            .map(|response| {
                info!(smtp_code = ?response.code(), "Email sent successfully via SMTP");
            })
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
    #[instrument(skip(self, request))]
    async fn send_activate_account(
        &self,
        request: Request<ActivateAccountRequest>,
    ) -> Result<Response<ActivateAccountResponse>, Status> {
        let req = request.into_inner();
        info!(
            email = %req.email,
            username = %req.username,
            "Received request to send activation email"
        );

        let message = self.create_activate_account_mail(&req).await.map_err(|e| {
            error!(
                email = %req.email,
                username = %req.username,
                error = %e,
                "Failed to create activation email"
            );
            Status::internal(format!("Could not create email: {}", e))
        })?;

        self.send_email(message).map_err(|e| {
            error!(
                email = %req.email,
                username = %req.username,
                error = %e,
                "Failed to send activation email"
            );
            Status::internal(format!("Could not send email: {}", e))
        })?;

        info!(
            email = %req.email,
            username = %req.username,
            "Activation email sent successfully"
        );
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
    #[instrument(skip(self, request))]
    async fn send_forgot_password(
        &self,
        request: Request<ForgotPasswordRequest>,
    ) -> Result<Response<ForgotPasswordResponse>, Status> {
        let req = request.into_inner();
        info!(
            email = %req.email,
            username = %req.username,
            "Received request to send forgot password email"
        );

        let message = self.create_forgot_password_mail(&req).await.map_err(|e| {
            error!(
                email = %req.email,
                username = %req.username,
                error = %e,
                "Failed to create forgot password email"
            );
            Status::internal(format!("Could not create email: {}", e))
        })?;

        self.send_email(message).map_err(|e| {
            error!(
                email = %req.email,
                username = %req.username,
                error = %e,
                "Failed to send forgot password email"
            );
            Status::internal(format!("Could not send email: {}", e))
        })?;

        info!(
            email = %req.email,
            username = %req.username,
            "Forgot password email sent successfully"
        );
        let reply = ForgotPasswordResponse { success: true };
        Ok(Response::new(reply))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Renders the activation account email template with the given link
    ///
    /// This is a helper function for testing template rendering logic.
    ///
    /// # Arguments
    ///
    /// * `activation_link` - The activation link to include in the email
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - Successfully rendered HTML template
    /// * `Err(String)` - Error occurred during template rendering
    fn render_activate_account_template(activation_link: &str) -> Result<String, String> {
        Handlebars::new()
            .render_template(
                ACTIVATE_ACCOUNT_TEMPLATE,
                &json!({"activation_link": activation_link}),
            )
            .map_err(|e| format!("Template rendering error: {}", e))
    }

    /// Renders the forgot password email template with the given link
    ///
    /// This is a helper function for testing template rendering logic.
    ///
    /// # Arguments
    ///
    /// * `forgot_password_link` - The password reset link to include in the email
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - Successfully rendered HTML template
    /// * `Err(String)` - Error occurred during template rendering
    fn render_forgot_password_template(forgot_password_link: &str) -> Result<String, String> {
        Handlebars::new()
            .render_template(
                FORGOT_PASSWORD_TEMPLATE,
                &json!({"forgot_password_link": forgot_password_link}),
            )
            .map_err(|e| format!("Template rendering error: {}", e))
    }

    #[test]
    fn test_render_activate_account_template() {
        let activation_link = "https://example.com/activate?token=abc123";
        let result = render_activate_account_template(activation_link);

        assert!(result.is_ok());
        let rendered = result.unwrap();
        // Template uses {{activation_link}} which Handlebars will replace
        // Just verify the template renders successfully and contains HTML structure
        assert!(rendered.contains("html"));
        assert!(rendered.contains("Activate"));
    }

    #[test]
    fn test_render_activate_account_template_with_special_chars() {
        let activation_link = "https://example.com/activate?token=abc123&param=value";
        let result = render_activate_account_template(activation_link);

        assert!(result.is_ok());
        let rendered = result.unwrap();
        // Handlebars escapes special characters by default
        assert!(rendered.contains("abc123"));
    }

    #[test]
    fn test_render_forgot_password_template() {
        let reset_link = "https://example.com/reset?token=xyz789";
        let result = render_forgot_password_template(reset_link);

        assert!(result.is_ok());
        let rendered = result.unwrap();
        // Template uses {{forgot_password_link}} which Handlebars will replace
        // Just verify the template renders successfully and contains HTML structure
        assert!(rendered.contains("html"));
        assert!(rendered.contains("Reset"));
    }

    #[test]
    fn test_render_forgot_password_template_with_special_chars() {
        let reset_link = "https://example.com/reset?token=xyz789&param=value";
        let result = render_forgot_password_template(reset_link);

        assert!(result.is_ok());
        let rendered = result.unwrap();
        assert!(rendered.contains("xyz789"));
    }

    #[test]
    fn test_render_activate_account_template_empty_link() {
        let result = render_activate_account_template("");
        assert!(result.is_ok());
        // Even with empty link, template should render
        let rendered = result.unwrap();
        assert!(rendered.contains("html"));
    }

    #[test]
    fn test_render_forgot_password_template_empty_link() {
        let result = render_forgot_password_template("");
        assert!(result.is_ok());
        // Even with empty link, template should render
        let rendered = result.unwrap();
        assert!(rendered.contains("html"));
    }
}
