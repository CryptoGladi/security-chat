use chrono::Local;
use crate_proto::authentication::{
    CheckTokenRequest, CheckTokenResponse, LoginRequest, LoginResponse, NicknameIsTakenRequest,
    NicknameIsTakenResponse, RegistrationRequest, RegistrationResponse,
};
use crate_proto::Authentication;
use database::check_user;
use database::models::NewUser;
use database::schema::users::dsl::users;
use database::DbPool;
use diesel_async::RunQueryDsl;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use log::info;
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use tonic::metadata::{Ascii, MetadataValue};
use tonic::{Request, Response, Status};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub nickname: String,

    /// Expiration Time
    pub exp: chrono::DateTime<Local>,
}

#[derive(Debug)]
pub struct AuthenticationService {
    pub db_pool: DbPool,
    pub secret: Vec<u8>,
    pub lifetime_for_tokens: chrono::Duration,
    pub len_for_access_token: usize,
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

fn check_token(access_token: &str, secret: &[u8]) -> Result<(), Status> {
    let Ok(token_data) =
        jsonwebtoken::decode::<Claims>(access_token, &DecodingKey::from_secret(secret), &{
            let mut validation = Validation::new(Algorithm::HS256); // TODO change algorithm
            validation.validate_exp = false;
            validation.required_spec_claims = HashSet::new();

            validation
        })
    else {
        return Err(Status::permission_denied("YOUR TOKEN IS INVALID"));
    };

    if Local::now() >= token_data.claims.exp {
        return Err(Status::permission_denied("your token is out of date"));
    }

    Ok(())
}

pub fn grpc_intercept<T>(request: Request<T>, secret: &[u8]) -> Result<Request<T>, Status> {
    let access_token = get_from_metadata(&request, "access_token")?;
    check_token(access_token.to_str().unwrap(), secret)?;

    Ok(request)
}

#[tonic::async_trait]
impl Authentication for AuthenticationService {
    async fn registration(
        &self,
        request: Request<RegistrationRequest>,
    ) -> Result<Response<RegistrationResponse>, Status> {
        info!("Got a request for `registration`: {:?}", request.get_ref());
        let mut db = self.db_pool.get().await.unwrap();

        let refresh_token: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(self.len_for_access_token)
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
            .map_err(|_| Status::already_exists("this nickname already exists"))?;

        let claims = Claims {
            nickname: request.get_ref().nickname.clone(),
            exp: Local::now() + self.lifetime_for_tokens,
        };

        let access_token = get_new_access_token(claims, &self.secret)?;
        Ok(Response::new(RegistrationResponse {
            access_token,
            refresh_token,
        }))
    }

    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        info!("Got a request for `login`: {:?}", request.get_ref());
        let mut db = self.db_pool.get().await.unwrap();

        check_user(
            &mut db,
            &request.get_ref().nickname,
            &request.get_ref().refresh_token,
        )
        .await?;

        let claims = Claims {
            nickname: request.get_ref().nickname.clone(),
            exp: Local::now() + self.lifetime_for_tokens,
        };

        let access_token = get_new_access_token(claims, &self.secret)?;
        Ok(Response::new(LoginResponse { access_token }))
    }

    async fn check_token(
        &self,
        request: Request<CheckTokenRequest>,
    ) -> Result<Response<CheckTokenResponse>, Status> {
        info!("Got a request for `check_token`: {:?}", request.get_ref());

        return Ok(Response::new(CheckTokenResponse {
            is_valid: check_token(&request.get_ref().access_token, &self.secret).is_ok(),
        }));
    }

    async fn nickname_is_taken(
        &self,
        request: Request<NicknameIsTakenRequest>,
    ) -> Result<Response<NicknameIsTakenResponse>, Status> {
        info!(
            "Got a request for `nickname_is_taken`: {:?}",
            request.get_ref()
        );
        let mut db = self.db_pool.get().await.unwrap();

        Ok(Response::new(NicknameIsTakenResponse {
            is_taken: !database::get_user_by_nickname(&mut db, &request.get_ref().nickname)
                .await
                .is_empty(),
        }))
    }
}
