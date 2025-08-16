use tonic::{Request, Response, Result, Status};

use crate::service::service::{
    ActivateAccountRequest, ActivateAccountResponse, email_service_server::EmailService,
};

pub mod service {
    tonic::include_proto!("email_service");
}

#[derive(Default)]
pub struct Service {}

#[tonic::async_trait]
impl EmailService for Service {
    async fn send_activate_account(
        &self,
        request: Request<ActivateAccountRequest>,
    ) -> Result<Response<ActivateAccountResponse>, Status> {
        println!("Got {:?}", request);

        let reply = ActivateAccountResponse { success: true };
        Ok(Response::new(reply))
    }
}
