use crate_proto::authentication::{JwtReply, LoginRequest, RegistrationRequest};
use crate_proto::Authentication;
use database::check_user;
use database::models::NewUser;
use database::schema::users::dsl::users;
use database::DbPool;
use diesel_async::RunQueryDsl;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use log::debug;
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use tonic::metadata::{Ascii, MetadataValue};
use tonic::{Request, Response, Status};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub nickname: String,
}

#[derive(Debug)]
pub struct AuthenticationServer<'a> {
    pub db_pool: DbPool,
    pub secret: &'a [u8],
}

pub fn get_new_access_token(claims: Claims, secret: &[u8]) -> Result<String, Status> {
    let Ok(access_token) = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    ) else {
        return Err(Status::internal("jsonwebtoken::encode"));
    };

    Ok(access_token)
}

fn get_from_metadata<'a>(
    request: &'a Request<()>,
    key: &'a str,
) -> Result<&'a MetadataValue<Ascii>, Status> {
    let Some(value) = request.metadata().get(key) else {
        return Err(Status::unauthenticated(format!(
            "not found '{key}' in metadata"
        )));
    };

    Ok(value)
}

pub fn grpc_intercept(request: Request<()>) -> Result<Request<()>, Status> {
    let access_token = get_from_metadata(&request, "access_token")?;

    if jsonwebtoken::decode::<Claims>(
        access_token.to_str().unwrap(),
        &DecodingKey::from_secret("secret".as_ref()), // TODO get secret
        &Validation::new(Algorithm::HS256),
    )
    .is_err()
    {
        return Err(Status::permission_denied("YOUR TOKEN IS INVALID"));
    }

    Ok(request)
}

#[tonic::async_trait]
impl Authentication for AuthenticationServer<'static> {
    async fn registration(
        &self,
        request: Request<RegistrationRequest>,
    ) -> Result<Response<JwtReply>, Status> {
        debug!("Got a request for `registration`: {:?}", request.get_ref());
        let mut db = self.db_pool.get().await.unwrap();

        let refresh_token: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(40)
            .map(char::from)
            .collect();

        let new_user = NewUser {
            nickname: request.get_ref().nickname.as_str(),
            authkey: &refresh_token,
        };

        diesel::insert_into(users)
            .values(&new_user)
            .execute(&mut db)
            .await
            .unwrap();

        let claims = Claims {
            nickname: request.get_ref().nickname.clone(),
        };

        let access_token = get_new_access_token(claims, self.secret)?;
        Ok(Response::new(JwtReply { access_token }))
    }

    async fn login(&self, request: Request<LoginRequest>) -> Result<Response<JwtReply>, Status> {
        debug!("Got a request for `login`: {:?}", request.get_ref());
        let mut db = self.db_pool.get().await.unwrap();

        check_user(
            &mut db,
            &request.get_ref().nickname,
            &request.get_ref().refresh_token,
        )
        .await?;

        let claims = Claims {
            nickname: request.get_ref().nickname.clone(),
        };

        let access_token = get_new_access_token(claims, self.secret)?;
        Ok(Response::new(JwtReply { access_token }))
    }
}
