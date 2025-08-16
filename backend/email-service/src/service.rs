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
        ActivateAccountRequest, ActivateAccountResponse, email_service_server::EmailService,
    },
};

const ACTIVATE_ACCOUNT_TEMPLATE: &str = include_str!("../emails/activate_account_template.html");

pub mod email_service {
    tonic::include_proto!("email_service");
}

pub struct Service {
    config: Arc<Config>,
    mailer: SmtpTransport,
}

impl Service {
    pub fn new(config: Arc<Config>) -> Result<Self, Box<dyn std::error::Error>> {
        let creds = Credentials::new(config.smtp_username.clone(), config.smtp_password.clone());
        let mailer = SmtpTransport::starttls_relay(&config.smtp_relay)?
            .credentials(creds)
            .build();

        Ok(Self { config, mailer })
    }

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

    fn send_email(&self, message: Message) -> Result<(), ()> {
        self.mailer.send(&message).map_err(|_| ()).map(|_| ())
    }
}

#[tonic::async_trait]
impl EmailService for Service {
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
}
