use crate_proto::security_chat::*;
use crate_proto::SecurityChat;
use database::models::{Message as DbMessage, *};
use database::schema;
use database::schema::chat_messages::dsl::{chat_messages, created_at, recipient_id, sender_id};
use database::schema::order_add_keys::dsl::*;
use database::schema::users::dsl::{nickname, users};
use database::DbPool;
use database::{get_user_by_id, get_user_by_nickname};
use diesel::internal::derives::multiconnection::chrono;
use diesel::internal::derives::multiconnection::chrono::Local;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::metadata::{Ascii, MetadataValue};
use tonic::{Request, Response, Status};

type MessageProducer = tokio::sync::broadcast::Sender<Notification>;
type MessageConsumer = tokio::sync::broadcast::Receiver<Notification>;

fn get_from_metadata<'a, T>(
    request: &'a Request<T>,
    key: &'a str,
) -> Result<&'a MetadataValue<Ascii>, Status> {
    let Some(value) = request.metadata().get(key) else {
        return Err(Status::unauthenticated(format!(
            "not found '{key}' in metadata"
        )));
    };

    Ok(value)
}

fn get_nickname<T>(request: &Request<T>, secret: &[u8]) -> Result<String, Status> {
    let access_token = get_from_metadata(&request, "access_token")?
        .to_str()
        .unwrap();

    let token_data =
        jsonwebtoken::decode::<Claims>(access_token, &DecodingKey::from_secret(secret), &{
            let mut validation = Validation::new(Algorithm::HS256); // TODO change algorithm
            validation.validate_exp = false;
            validation.required_spec_claims = HashSet::new();

            validation
        })
        .unwrap();

    Ok(token_data.claims.nickname)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub nickname: String,

    /// Expiration Time
    pub exp: chrono::DateTime<Local>,
}

#[derive(Debug)]
pub struct SecurityChatService {
    pub db_pool: DbPool,
    pub producer: MessageProducer,
    pub consumer: MessageConsumer,
    pub secret: Vec<u8>,
}

#[tonic::async_trait]
impl SecurityChat for SecurityChatService {
    async fn send_aes_key(
        &self,
        request: Request<SendAesKeyRequest>,
    ) -> Result<Response<()>, Status> {
        info!("Got a request for `send_aes_key`: {:?}", request.get_ref());
        let mut db = self.db_pool.get().await.unwrap();

        let user_to = &database::get_user_by_nickname(
            &mut db,
            &get_nickname(&request, &self.secret).unwrap(),
        )
        .await[0];

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

    async fn get_aes_key(&self, request: Request<()>) -> Result<Response<GetAesKeyReply>, Status> {
        info!("Got a request for `get_aes_key`: {:?}", request.get_ref());
        let mut db = self.db_pool.get().await.unwrap();
        let user = &database::get_user_by_nickname(
            &mut db,
            &get_nickname(&request, &self.secret).unwrap(),
        )
        .await[0];

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

        diesel::delete(order_add_keys.filter(id.eq(request.get_ref().id)))
            .execute(&mut db)
            .await
            .unwrap();

        Ok(Response::new(()))
    }

    type SubscribeStream = ReceiverStream<Result<Notification, Status>>;

    async fn subscribe(
        &self,
        request: Request<()>,
    ) -> Result<Response<Self::SubscribeStream>, Status> {
        info!("new subscribe: {:?}", request.get_ref());

        let nnickname = get_nickname(&request, &self.secret).unwrap();
        let (tx, rx) = mpsc::channel(4);
        let mut notification = self.producer.subscribe();

        tokio::spawn(async move {
            loop {
                let n = notification.recv().await.unwrap();

                if n.nickname_from == nnickname && tx.send(Ok(n)).await.is_err() {
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
        let nnickname = get_nickname(&request, &self.secret).unwrap();

        let user_sender = &get_user_by_nickname(&mut db, &nnickname).await[0];
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
                by_nickname: nnickname.clone(),
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
        let nnickname = get_nickname(&request, &self.secret).unwrap();

        let user_sender = &get_user_by_nickname(&mut db, &nnickname).await[0];
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
}
