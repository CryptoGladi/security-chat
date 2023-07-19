use crate::models::*;
use crate::schema::users::dsl::*;
use diesel::prelude::*;
use diesel::PgConnection;
use log::info;
use security_chat::security_chat_server::SecurityChat;
use security_chat::RegistrationRequest;
use security_chat::Status as ProtocolStatus;
use std::sync::Mutex;
use tonic::Request;
use tonic::Response;
use tonic::Status;

pub mod security_chat {
    tonic::include_proto!("security_chat");
}

pub struct SecurityChatService {
    pub db: Mutex<PgConnection>,
}

#[tonic::async_trait]
impl SecurityChat for SecurityChatService {
    async fn registration(
        &self,
        request: Request<RegistrationRequest>,
    ) -> Result<Response<ProtocolStatus>, Status> {
        info!("Got a request for registration: {:?}", request);

        let new_user = NewUser {
            username: request.get_ref().username.as_str(),
        };

        let Ok(_) = diesel::insert_into(users)
        .values(&new_user)
        .execute(&mut *self.db.lock().unwrap()) else {
            return Ok(Response::new(ProtocolStatus::default())); // TODO
        };

        Ok(Response::new(ProtocolStatus::default()))
    }
}
