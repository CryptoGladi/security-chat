use log::info;
use security_chat::security_chat_server::SecurityChat;
use tonic::Request;
use security_chat::RegistrationRequest;
use security_chat::Status as ProtocolStatus;
use tonic::Response;
use tonic::Status;

pub mod security_chat {
    tonic::include_proto!("security_chat"); 
}

#[derive(Debug, Default)]
pub struct SecurityChatService {}

#[tonic::async_trait]
impl SecurityChat for SecurityChatService {
    async fn registration(
        &self,
        request: Request<RegistrationRequest>,
    ) -> Result<Response<ProtocolStatus>, Status> { 
        info!("Got a request in registration: {:?}", request);

        Ok(Response::new(ProtocolStatus::default()))
    }
}