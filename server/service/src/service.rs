use crate::database::{self, get_user_by_id, get_user_by_nickname, DbPool};
use crate::models::{Message as DbMessage, *};
use crate::schema;
use crate::schema::chat_messages::dsl::{chat_messages, created_at, recipient_id, sender_id};
use crate::schema::order_add_keys::dsl::*;
use crate::schema::users::dsl::{nickname, users};
use crate_proto::security_chat::*;
use crate_proto::SecurityChat;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use log::{error, info};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

type MessageProducer = tokio::sync::broadcast::Sender<Notification>;
type MessageConsumer = tokio::sync::broadcast::Receiver<Notification>;

#[derive(Debug)]
pub struct SecurityChatService {
    pub db_pool: DbPool,
    pub producer: MessageProducer,
    pub consumer: MessageConsumer,
}

#[tonic::async_trait]
impl SecurityChat for SecurityChatService {
    async fn send_aes_key(
        &self,
        request: Request<SendAesKeyRequest>,
    ) -> Result<Response<()>, Status> {
        info!("Got a request for `send_aes_key`: {:?}", request.get_ref());
        let mut db = self.db_pool.get().await.unwrap();
        let nickname_for_check = request.get_ref().clone().nickname_to.unwrap().nickname;
        let authkey_for_check = request.get_ref().clone().nickname_to.unwrap().authkey;

        let user_to =
            database::check_user(&mut db, &nickname_for_check, &authkey_for_check).await?;

        let user_from = users
            .filter(nickname.eq(request.get_ref().nickname_from.clone()))
            .select(User::as_select())
            .load(&mut db)
            .await
            .unwrap();

        if user_to.id == user_from[0].id {
            return Err(tonic::Status::invalid_argument("user_to same user_from"));
        }

        let new_aes_key = NewKey {
            user_to_id: user_to.id,
            user_from_id: user_from[0].id,
            user_to_public_key: request.get_ref().public_key.clone(),
        };

        let key_info: Key = diesel::insert_into(order_add_keys)
            .values(&new_aes_key)
            .get_result(&mut db)
            .await
            .unwrap();

        self.producer
            .send(Notification {
                nickname_from: user_from[0].nickname.clone(),
                by_nickname: user_to.nickname.clone(),
                notice: Some(notification::Notice::NewSendAesKey(AesKeyInfo {
                    id: key_info.id,
                    nickname_to: user_to.nickname.clone(),
                    nickname_from: user_from[0].nickname.clone(),
                    nickname_to_public_key: request.get_ref().public_key.clone(),
                    nickname_from_public_key: None,
                })),
            })
            .unwrap();

        Ok(Response::new(()))
    }

    async fn get_aes_key(
        &self,
        request: Request<GetAesKeyRequest>,
    ) -> Result<Response<GetAesKeyReply>, Status> {
        info!("Got a request for `get_aes_key`: {:?}", request.get_ref());
        let mut db = self.db_pool.get().await.unwrap();
        let user_for_check = request.get_ref().clone().nickname.unwrap();
        let user = database::check_user(&mut db, &user_for_check.nickname, &user_for_check.authkey)
            .await?;

        let keys = order_add_keys
            .filter(user_to_id.eq(user.id))
            .or_filter(user_from_id.eq(user.id))
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

        return Ok(Response::new(GetAesKeyReply { info }));
    }

    async fn set_user_from_aes_key(
        &self,
        request: Request<SetUserFromAesKeyRequest>,
    ) -> Result<Response<()>, Status> {
        info!(
            "Got a request for `set_user_from_aes_key`: {:?}",
            request.get_ref()
        );
        let mut db = self.db_pool.get().await.unwrap();
        let user_for_check = request.get_ref().clone().nickname.unwrap();
        let _ = database::check_user(&mut db, &user_for_check.nickname, &user_for_check.authkey)
            .await?;

        let key: Key = diesel::update(order_add_keys)
            .filter(id.eq(request.get_ref().id))
            .set(user_from_public_key.eq(request.get_ref().public_key.clone()))
            .get_result(&mut db)
            .await
            .unwrap();

        let user_to = &get_user_by_id(&mut db, key.user_to_id).await[0];
        let user_from = &get_user_by_id(&mut db, key.user_from_id).await[0];

        self.producer
            .send(Notification {
                nickname_from: user_to.nickname.clone(),
                by_nickname: user_from.nickname.clone(),
                notice: Some(notification::Notice::NewAcceptAesKey(AesKeyInfo {
                    id: key.id,
                    nickname_to: user_to.nickname.clone(),
                    nickname_from: user_from.nickname.clone(),
                    nickname_to_public_key: key.user_to_public_key,
                    nickname_from_public_key: key.user_from_public_key,
                })),
            })
            .unwrap();

        Ok(Response::new(()))
    }

    async fn delete_aes_key(
        &self,
        request: Request<DeleteAesKeyRequest>,
    ) -> Result<Response<()>, Status> {
        info!(
            "Got a request for `delete_aes_key`: {:?}",
            request.get_ref()
        );
        let mut db = self.db_pool.get().await.unwrap();
        let user_for_check = request.get_ref().clone().nickname.unwrap();
        let _ = database::check_user(&mut db, &user_for_check.nickname, &user_for_check.authkey)
            .await?;

        diesel::delete(order_add_keys.filter(id.eq(request.get_ref().id)))
            .execute(&mut db)
            .await
            .unwrap();

        Ok(Response::new(()))
    }

    async fn check_valid(
        &self,
        request: Request<CheckValidRequest>,
    ) -> Result<Response<CheckValidReply>, Status> {
        info!("Got a request for `check_valid`: {:?}", request.get_ref());
        let mut db = self.db_pool.get().await.unwrap();

        let user = database::check_user(
            &mut db,
            &request.get_ref().nickname,
            &request.get_ref().authkey,
        )
        .await;

        Ok(Response::new(CheckValidReply {
            is_valid: user.is_ok(),
        }))
    }

    type SubscribeStream = ReceiverStream<Result<Notification, Status>>;

    async fn subscribe(
        &self,
        request: Request<Check>,
    ) -> Result<Response<Self::SubscribeStream>, Status> {
        info!("new subscribe: {:?}", request.get_ref());
        let mut db = self.db_pool.get().await.unwrap();
        let user_for_check = request.get_ref().clone();
        let _ = database::check_user(&mut db, &user_for_check.nickname, &user_for_check.authkey)
            .await?;

        let (tx, rx) = mpsc::channel(4);
        let mut notification = self.producer.subscribe();

        tokio::spawn(async move {
            loop {
                let n = notification.recv().await.unwrap();

                if n.nickname_from == user_for_check.nickname && tx.send(Ok(n)).await.is_err() {
                    break;
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn send_message(
        &self,
        request: Request<SendMessageRequest>,
    ) -> Result<Response<()>, Status> {
        info!("Got a request for `send_message`: {:?}", request.get_ref());
        let mut db = self.db_pool.get().await.unwrap();
        let user_for_check = request.get_ref().clone().nickname.unwrap();
        let _ = database::check_user(&mut db, &user_for_check.nickname, &user_for_check.authkey)
            .await?;

        let user_sender = &get_user_by_nickname(&mut db, &user_for_check.nickname).await[0];
        let user_recipient =
            &get_user_by_nickname(&mut db, &request.get_ref().nickname_from).await[0];

        let new_message = AddMessage {
            sender_id: user_sender.id,
            recipient_id: user_recipient.id,
            message_body: request.get_ref().message.clone().unwrap().body,
            nonce: request.get_ref().message.clone().unwrap().nonce,
        };

        let ids: Vec<i64> = diesel::insert_into(chat_messages)
            .values(new_message)
            .returning(schema::chat_messages::id)
            .get_results(&mut db)
            .await
            .unwrap();

        self.producer
            .send(Notification {
                nickname_from: request.get_ref().nickname_from.clone(),
                by_nickname: user_for_check.nickname.clone(),
                notice: Some(notification::Notice::NewMessage(MessageWithId {
                    message: request.get_ref().clone().message,
                    id: ids[0],
                })),
            })
            .unwrap();

        Ok(Response::new(()))
    }

    async fn get_latest_messages(
        &self,
        request: Request<GetLatestMessagesRequest>,
    ) -> Result<Response<GetLatestMessagesReply>, Status> {
        info!(
            "Got a request for `get_latest_messages`: {:?}",
            request.get_ref()
        );

        let mut db = self.db_pool.get().await.unwrap();
        let user_for_check = request.get_ref().clone().nickname.unwrap();
        let _ = database::check_user(&mut db, &user_for_check.nickname, &user_for_check.authkey)
            .await?;

        let user_sender = &get_user_by_nickname(&mut db, &user_for_check.nickname).await[0];

        let mut result = GetLatestMessagesReply::default();

        for nickname_for_get in request.get_ref().nickname_for_get.iter() {
            let user_recipient = &get_user_by_nickname(&mut db, nickname_for_get).await[0];

            let messages = chat_messages
                .or_filter(sender_id.eq(user_sender.id))
                .or_filter(sender_id.eq(user_recipient.id))
                .or_filter(recipient_id.eq(user_recipient.id))
                .or_filter(recipient_id.eq(user_sender.id))
                .limit(request.get_ref().get_limit)
                .order(created_at.desc())
                .select(DbMessage::as_select())
                .load(&mut db)
                .await
                .unwrap();

            for message in messages.iter() {
                result.messages.push(MessageInfo {
                    body: Some(crate_proto::security_chat::Message {
                        body: message.message_body.clone(),
                        nonce: message.nonce.clone(),
                    }),
                    sender_nickname: get_user_by_id(&mut db, message.sender_id).await[0]
                        .nickname
                        .clone(),
                    recipient_nickname: get_user_by_id(&mut db, message.recipient_id).await[0]
                        .nickname
                        .clone(),
                    id: message.id,
                });
            }
        }

        Ok(Response::new(result))
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

        diesel::insert_into(users)
            .values(&new_user)
            .execute(&mut db)
            .await
            .unwrap();

        Ok(Response::new(RegistrationReply {
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
