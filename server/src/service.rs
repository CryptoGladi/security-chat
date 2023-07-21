use self::security_chat::LoginReply;
use self::security_chat::LoginRequest;
use self::security_chat::NicknameIsTakenReply;
use self::security_chat::NicknameIsTakenRequest;
use self::security_chat::RegistrationReply;
use crate::database::DbPool;
use crate::models::*;
use crate::schema::users::dsl::*;
use diesel::prelude::*;
use log::error;
use log::info;
use security_chat::security_chat_server::SecurityChat;
use security_chat::RegistrationRequest;
use security_chat::Status as ProtocolStatus;
use tonic::Request;
use tonic::Response;
use tonic::Status;

pub mod security_chat {
    tonic::include_proto!("security_chat");
}

pub struct SecurityChatService {
    pub db_pool: DbPool
}

#[tonic::async_trait]
impl SecurityChat for SecurityChatService {
    async fn login(
        &self,
        request: Request<LoginRequest>
    ) -> Result<Response<LoginReply>, Status> {
        info!("Got a request for `login`: {:?}", request);
        let mut db = self.db_pool.get().unwrap();

        let user = users
            .filter(nickname.eq(request.get_ref().nickname.clone()))
            .select(User::as_select())
            .load(&mut db)
            .unwrap();
        
        if user[0].authkey == request.get_ref().authkey {

        }

        todo!()
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
            authkey: &uuid_authkey
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
