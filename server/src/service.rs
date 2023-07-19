use crate::database;
use crate::models::*;
use crate::schema::users::dsl::*;
use diesel::prelude::*;
use log::debug;
use log::error;
use log::info;
use security_chat::security_chat_server::SecurityChat;
use security_chat::RegistrationRequest;
use security_chat::Status as ProtocolStatus;
use tonic::Request;
use tonic::Response;
use tonic::Status;
use self::security_chat::NicknameIsTakenReply;
use self::security_chat::NicknameIsTakenRequest;

pub mod security_chat {
    tonic::include_proto!("security_chat");
}

#[derive(Default)]
pub struct SecurityChatService;

#[tonic::async_trait]
impl SecurityChat for SecurityChatService {
    async fn registration(
        &self,
        request: Request<RegistrationRequest>,
    ) -> Result<Response<ProtocolStatus>, Status> {
        info!("Got a request for registration: {:?}", request);
        let mut db = database::establish_connection();

        let new_user = NewUser {
            nickname: request.get_ref().nickname.as_str(),
        };

        let Ok(_) = diesel::insert_into(users)
        .values(&new_user)
        .execute(&mut db) else {
            return Ok(Response::new(ProtocolStatus::default())); // TODO
        };

        Ok(Response::new(ProtocolStatus::default()))
    }

    async fn nickname_is_taken(
        &self,
        request: Request<NicknameIsTakenRequest>,
    ) -> Result<Response<NicknameIsTakenReply>, Status> {
        info!("Got a request for nickname_is_taken: {:?}", request);
        let mut db = database::establish_connection();

        return match users.filter(nickname.eq("")).limit(1).select(User::as_select()).load(&mut db) {
            Ok(e) => Ok(Response::new(NicknameIsTakenReply {
                is_taken: e.len() >= 1
            })),
            Err(e) => {
                error!("database error in nickname_is_taken: {:?}", e);

                Ok(Response::new(NicknameIsTakenReply {
                    is_taken: false
                }))
            }
        };
    }
}
