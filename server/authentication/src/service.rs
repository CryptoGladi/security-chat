use crate_proto::authentication::{JwtReply, LoginRequest, RegistrationRequest};
use crate_proto::security_chat::CheckValidReply;
use crate_proto::Authentication;
use database::models::NewUser;
use database::models::{Message as DbMessage, *};
use database::schema;
use database::schema::chat_messages::dsl::{chat_messages, created_at, recipient_id, sender_id};
use database::schema::order_add_keys::dsl::*;
use database::schema::users::dsl::{nickname, users};
use database::DbPool;
use database::{get_user_by_id, get_user_by_nickname};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use log::{debug, info};
use serde::{Deserialize, Serialize};
use tonic::{Request, Response, Status};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    nickname: String,
}

#[derive(Debug)]
pub struct AuthenticationServer {
    pub db_pool: DbPool,
}

#[tonic::async_trait]
impl Authentication for AuthenticationServer {
    async fn registration(
        &self,
        request: Request<RegistrationRequest>,
    ) -> Result<Response<JwtReply>, Status> {
        debug!("Got a request for `registration`: {:?}", request.get_ref());
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

        let claims = Claims {
            nickname: request.get_ref().nickname.clone(),
        };

        let Ok(jwt) = jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(uuid_authkey.as_bytes()),
        ) else {
            return Err(Status::internal("jsonwebtoken::encode"));
        };

        Ok(Response::new(JwtReply { jwt }))
    }

    async fn login(&self, request: Request<LoginRequest>) -> Result<Response<JwtReply>, Status> {
        info!("Got a request for `check_valid`: {:?}", request.get_ref());
        let mut db = self.db_pool.get().await.unwrap();

        let user_info = get_user_by_nickname(&mut db, &request.get_ref().nickname).await;

        let Ok(_claims) = jsonwebtoken::decode::<Claims>(
            &request.get_ref().jwt,
            &DecodingKey::from_secret(user_info[0].authkey.as_bytes()),
            &Validation::new(Algorithm::HS256),
        ) else {
            return Err(Status::permission_denied("your authkey not valid"));
        };

        todo!();
    }
}
