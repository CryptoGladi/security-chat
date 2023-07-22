use self::security_chat::{
    LoginReply, LoginRequest, NicknameIsTakenReply, NicknameIsTakenRequest, RegistrationReply,
};
use crate::database::DbPool;
use crate::models::*;
use crate::schema::users::dsl::*;
use diesel::prelude::*;
use log::{error, info};
use security_chat::security_chat_server::SecurityChat;
use security_chat::{RegistrationRequest, Status as ProtocolStatus};
use tonic::{Request, Response, Status};

pub mod security_chat {
    tonic::include_proto!("security_chat");
}

pub struct SecurityChatService {
    pub db_pool: DbPool,
}

#[tonic::async_trait]
impl SecurityChat for SecurityChatService {
    async fn login(&self, request: Request<LoginRequest>) -> Result<Response<LoginReply>, Status> {
        info!("Got a request for `login`: {:?}", request);
        let mut db = self.db_pool.get().unwrap();

        let Ok(user) = users
            .filter(nickname.eq(request.get_ref().nickname.clone()))
            .select(User::as_select())
            .load(&mut db)
            else {
                return Ok(Response::new(LoginReply {
                    is_successful: false
                }));
        };

        Ok(Response::new(LoginReply {
            is_successful: user[0].authkey == request.get_ref().authkey,
        }))
    }

    async fn registration(
        &self,
        request: Request<RegistrationRequest>,
    ) -> Result<Response<RegistrationReply>, Status> {
        info!("Got a request for `registration`: {:?}", request);
        let mut db = self.db_pool.get().unwrap();

        let uuid_authkey = uuid::Uuid::new_v4().to_string();
        let new_user = NewUser {
            nickname: request.get_ref().nickname.as_str(),
            authkey: &uuid_authkey,
        };

        let Ok(_) = diesel::insert_into(users)
        .values(&new_user)
        .execute(&mut db) else {
            return  Ok(Response::new(RegistrationReply { status: Some(ProtocolStatus::default()), authkey: "".to_string() } )); // TODO
        };

        Ok(Response::new(RegistrationReply {
            status: Some(ProtocolStatus::default()),
            authkey: uuid_authkey,
        }))
    }

    async fn nickname_is_taken(
        &self,
        request: Request<NicknameIsTakenRequest>,
    ) -> Result<Response<NicknameIsTakenReply>, Status> {
        info!("Got a request for `nickname_is_taken`: {:?}", request);
        let mut db = self.db_pool.get().unwrap();

        return match users
            .filter(nickname.eq(request.get_ref().nickname.clone()))
            .limit(1)
            .select(User::as_select())
            .load(&mut db)
        {
            Ok(e) => Ok(Response::new(NicknameIsTakenReply {
                is_taken: e.len() >= 1,
            })),
            Err(e) => {
                error!("database error in nickname_is_taken: {:?}", e);

                Ok(Response::new(NicknameIsTakenReply { is_taken: false }))
            }
        };
    }
}
