use self::security_chat::{
    AcceptAesKeyReply, AcceptAesKeyRequest, AesKeyInfo, CheckValidReply, CheckValidRequest,
    DeleteAesKeyReply, DeleteAesKeyRequest, GetAesKeyReply, GetAesKeyRequest, NicknameIsTakenReply,
    NicknameIsTakenRequest, RegistrationReply, SendAesKeyReply, SendAesKeyRequest,
    SetUserFromAesKeyReply, SetUserFromAesKeyRequest,
};
use crate::database::{get_user_by_id, DbPool};
use crate::models::*;
use crate::schema::order_add_keys::dsl::*;
use crate::schema::users::dsl::{nickname, users};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use log::{error, info};
use security_chat::security_chat_server::SecurityChat;
use security_chat::{RegistrationRequest, Status as ProtocolStatus};
use tonic::{Request, Response, Status};

#[allow(non_snake_case)]
pub mod security_chat {
    tonic::include_proto!("security_chat");
}

pub struct SecurityChatService {
    pub db_pool: DbPool,
}

#[tonic::async_trait]
impl SecurityChat for SecurityChatService {
    async fn send_aes_key(
        &self,
        request: Request<SendAesKeyRequest>,
    ) -> Result<Response<SendAesKeyReply>, Status> {
        info!("Got a request for `send_aes_key`: {:?}", request.get_ref());
        let mut db = self.db_pool.get().await.unwrap();
        let nickname_for_check = request.get_ref().clone().nickname_to.unwrap().nickname;
        let authkey_for_check = request.get_ref().clone().nickname_to.unwrap().authkey;

        let user_to = users
            .filter(nickname.eq(nickname_for_check))
            .select(User::as_select())
            .load(&mut db)
            .await
            .unwrap();
        let user_from = users
            .filter(nickname.eq(request.get_ref().nickname_from.clone()))
            .select(User::as_select())
            .load(&mut db)
            .await
            .unwrap();

        if user_to.is_empty() || user_from.is_empty() {
            return Ok(Response::new(SendAesKeyReply {
                is_successful: false,
            }));
        } else if user_to[0].authkey != authkey_for_check {
            return Ok(Response::new(SendAesKeyReply {
                is_successful: false,
            }));
        }

        let new_aes_key = NewKey {
            user_to_id: user_to[0].id,
            user_from_id: user_from[0].id,
            user_to_public_key: request.get_ref().public_key.clone(),
        };
        diesel::insert_into(order_add_keys)
            .values(&new_aes_key)
            .execute(&mut db)
            .await
            .unwrap();

        return Ok(Response::new(SendAesKeyReply {
            is_successful: true,
        }));
    }

    async fn get_aes_key(
        &self,
        request: Request<GetAesKeyRequest>,
    ) -> Result<Response<GetAesKeyReply>, Status> {
        info!("Got a request for `get_aes_key`: {:?}", request.get_ref());
        let mut db = self.db_pool.get().await.unwrap();
        let user_for_check = request.get_ref().clone().nickname.unwrap();

        let user = users
            .filter(nickname.eq(user_for_check.nickname)) // filter(nickname.eq(user.nickname) and authkey.eq(user.authkey))
            .select(User::as_select())
            .load(&mut db)
            .await
            .unwrap();

        if user.is_empty() {
            return Ok(Response::new(GetAesKeyReply {
                is_successful: false,
                info: vec![],
            }));
        } else if user[0].authkey != user_for_check.authkey {
            return Ok(Response::new(GetAesKeyReply {
                is_successful: false,
                info: vec![],
            }));
        }

        let keys = order_add_keys
            .filter(user_to_id.eq(user[0].id))
            .or_filter(user_from_id.eq(user[0].id))
            .select(Key::as_select())
            .load(&mut db)
            .await
            .unwrap();

        let mut info = vec![];
        for x in keys {
            let user_to = &get_user_by_id(&mut db, x.user_to_id).await[0];
            let user_from = &get_user_by_id(&mut db, x.user_from_id).await[0];

            info.push(AesKeyInfo {
                id: x.id,
                nickname_to: user_to.nickname.clone(),
                nickname_from: user_from.nickname.clone(),
                nickname_to_public_key: x.user_to_public_key,
                nickname_from_public_key: x.user_from_public_key,
            });
        }

        return Ok(Response::new(GetAesKeyReply {
            is_successful: true,
            info,
        }));
    }

    async fn set_user_from_aes_key(
        &self,
        request: Request<SetUserFromAesKeyRequest>,
    ) -> Result<Response<SetUserFromAesKeyReply>, Status> {
        info!(
            "Got a request for `set_user_from_aes_key`: {:?}",
            request.get_ref()
        );
        let mut db = self.db_pool.get().await.unwrap();
        let user_for_check = request.get_ref().clone().nickname.unwrap();

        let user = users
            .filter(nickname.eq(user_for_check.nickname)) // filter(nickname.eq(user.nickname) and authkey.eq(user.authkey))
            .select(User::as_select())
            .load(&mut db)
            .await
            .unwrap();

        if user.is_empty() {
            return Ok(Response::new(SetUserFromAesKeyReply {}));
        } else if user[0].authkey != user_for_check.authkey {
            return Ok(Response::new(SetUserFromAesKeyReply {}));
        }

        diesel::update(order_add_keys)
            .filter(id.eq(request.get_ref().id.clone()))
            .set((
                user_from_accepted.eq(true),
                user_from_public_key.eq(request.get_ref().public_key.clone()),
            ))
            .execute(&mut db).await.unwrap();

        Ok(Response::new(SetUserFromAesKeyReply {}))
    }

    async fn accept_aes_key(
        &self,
        request: Request<AcceptAesKeyRequest>,
    ) -> Result<Response<AcceptAesKeyReply>, Status> {
        info!(
            "Got a request for `accept_aes_key`: {:?}",
            request.get_ref()
        );
        let mut db = self.db_pool.get().await.unwrap();
        let user_for_check = request.get_ref().clone().nickname.unwrap();

        let user = users
            .filter(nickname.eq(user_for_check.nickname)) // filter(nickname.eq(user.nickname) and authkey.eq(user.authkey))
            .select(User::as_select())
            .load(&mut db)
            .await
            .unwrap();

        if user.is_empty() {
            return Ok(Response::new(AcceptAesKeyReply {}));
        } else if user[0].authkey != user_for_check.authkey {
            return Ok(Response::new(AcceptAesKeyReply {}));
        }

        diesel::delete(order_add_keys.filter(id.eq(request.get_ref().id))).execute(&mut db).await.unwrap();
        Ok(Response::new(AcceptAesKeyReply {}))
    }

    async fn delete_aes_key(
        &self,
        request: Request<DeleteAesKeyRequest>,
    ) -> Result<Response<DeleteAesKeyReply>, Status> {
        info!(
            "Got a request for `delete_aes_key`: {:?}",
            request.get_ref()
        );
        let mut db = self.db_pool.get().await.unwrap();
        let user_for_check = request.get_ref().clone().nickname.unwrap();

        let user = users
            .filter(nickname.eq(user_for_check.nickname)) // filter(nickname.eq(user.nickname) and authkey.eq(user.authkey))
            .select(User::as_select())
            .load(&mut db)
            .await
            .unwrap();

            if user.is_empty() {
                return Ok(Response::new(DeleteAesKeyReply {}));
            } else if user[0].authkey != user_for_check.authkey {
                return Ok(Response::new(DeleteAesKeyReply {}));
            }

            diesel::delete(order_add_keys.filter(id.eq(request.get_ref().id))).execute(&mut db).await.unwrap();

        Ok(Response::new(DeleteAesKeyReply {  }))
    }

    async fn check_valid(
        &self,
        request: Request<CheckValidRequest>,
    ) -> Result<Response<CheckValidReply>, Status> {
        info!("Got a request for `check_valid`: {:?}", request.get_ref());
        let mut db = self.db_pool.get().await.unwrap();

        let Ok(user) = users
            .filter(nickname.eq(request.get_ref().nickname.clone()))
            .select(User::as_select())
            .load(&mut db).await
            else {
                return Ok(Response::new(CheckValidReply {
                    is_successful: false
                }));
        };

        if user.is_empty() {
            return Ok(Response::new(CheckValidReply {
                is_successful: false,
            }));
        }

        Ok(Response::new(CheckValidReply {
            is_successful: user[0].authkey == request.get_ref().authkey,
        }))
    }

    async fn registration(
        &self,
        request: Request<RegistrationRequest>,
    ) -> Result<Response<RegistrationReply>, Status> {
        info!("Got a request for `registration`: {:?}", request.get_ref());
        let mut db = self.db_pool.get().await.unwrap();

        let uuid_authkey = uuid::Uuid::new_v4().to_string();
        let new_user = NewUser {
            nickname: request.get_ref().nickname.as_str(),
            authkey: &uuid_authkey,
        };

        let Ok(_) = diesel::insert_into(users)
        .values(&new_user)
        .execute(&mut db).await else {
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
        info!(
            "Got a request for `nickname_is_taken`: {:?}",
            request.get_ref()
        );
        let mut db = self.db_pool.get().await.unwrap();

        return match users
            .filter(nickname.eq(request.get_ref().nickname.clone()))
            .limit(1)
            .select(User::as_select())
            .load(&mut db)
            .await
        {
            Ok(e) => Ok(Response::new(NicknameIsTakenReply {
                is_taken: !e.is_empty(),
            })),
            Err(e) => {
                error!("database error in nickname_is_taken: {:?}", e);

                Ok(Response::new(NicknameIsTakenReply { is_taken: false }))
            }
        };
    }
}
