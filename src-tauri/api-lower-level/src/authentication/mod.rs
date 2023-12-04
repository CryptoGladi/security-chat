//! Module that is responsible **ONLY** for authentication

use crate_proto::authentication::{
    CheckTokenRequest, LoginRequest, NicknameIsTakenRequest, RegistrationRequest,
};
use crate_proto::AuthenticationClient as GRPCAuthenticationClient;
use error::Error;
use http::Uri;
use log::trace;
use tokens::{AccessToken, RefreshToken, Tokens};
use tonic::codec::CompressionEncoding;
use tonic::transport::Channel;

pub mod error;
pub mod tokens;

const COMPRESSION_ENCODING: CompressionEncoding = CompressionEncoding::Gzip;

#[allow(clippy::module_name_repetitions)]
pub struct AuthenticationClient {
    grpc_api: GRPCAuthenticationClient<Channel>,
}

impl AuthenticationClient {
    /// Init [gRPC](https://grpc.io/) connect to authentication server
    pub async fn connect(address: Uri) -> Result<Self, Error> {
        trace!("run `grpc_connect` to address: {address}");

        let channel = Channel::builder(address).connect().await?;
        let grpc_api = GRPCAuthenticationClient::new(channel)
            .send_compressed(COMPRESSION_ENCODING)
            .accept_compressed(COMPRESSION_ENCODING);

        Ok(Self { grpc_api })
    }

    pub async fn registration(&mut self, nickname: String) -> Result<Tokens, Error> {
        trace!("run `registration` for nickname: {nickname}");

        let request = RegistrationRequest { nickname };
        let response = self.grpc_api.registration(request).await?;

        Ok(Tokens {
            refresh_token: response.get_ref().refresh_token.clone(),
            access_token: AccessToken(response.get_ref().access_token.clone()),
        })
    }

    pub async fn login(
        &mut self,
        nickname: String,
        refresh_token: RefreshToken,
    ) -> Result<AccessToken, Error> {
        trace!("run `login` for nickname: {nickname}");

        let request = LoginRequest {
            nickname,
            refresh_token,
        };

        let response = self.grpc_api.login(request).await?;

        Ok(AccessToken(response.get_ref().access_token.clone()))
    }

    pub async fn check_valid(&mut self, access_token: AccessToken) -> Result<bool, Error> {
        trace!("run `check_valid`");

        let request = CheckTokenRequest {
            access_token: access_token.0,
        };

        let response = self.grpc_api.check_token(request).await?;
        Ok(response.get_ref().is_valid)
    }

    pub async fn nickname_is_taken(address: Uri, nickname: String) -> Result<bool, Error> {
        trace!("run `nickname_is_taken` for nickname: {nickname}");
        let mut client = Self::connect(address).await?;

        let request = NicknameIsTakenRequest { nickname };
        let response = client.grpc_api.nickname_is_taken(request).await?;

        Ok(response.get_ref().is_taken)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fcore::test_utils::*;

    #[tokio::test]
    async fn connect() {
        AuthenticationClient::connect(ADDRESS_SERVER.parse().unwrap())
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn registration() {
        let mut client = AuthenticationClient::connect(ADDRESS_SERVER.parse().unwrap())
            .await
            .unwrap();

        let random_nickname = get_rand_string(20);
        let tokens = client.registration(random_nickname).await.unwrap();

        log::info!("access_token: {}", tokens.access_token);
        log::info!("refresh: {}", tokens.refresh_token);

        assert!(client
            .check_valid(tokens.access_token.clone())
            .await
            .unwrap());

        assert!(!client
            .check_valid(AccessToken(tokens.refresh_token.clone()))
            .await
            .unwrap());
    }

    #[tokio::test]
    async fn registration_in_same_nickname() {
        let mut client = AuthenticationClient::connect(ADDRESS_SERVER.parse().unwrap())
            .await
            .unwrap();

        let random_nickname = get_rand_string(20);
        let _tokens = client.registration(random_nickname.clone()).await.unwrap();
        assert!(client.registration(random_nickname).await.is_err());
    }

    #[tokio::test]
    async fn check_valid() {
        let mut client = AuthenticationClient::connect(ADDRESS_SERVER.parse().unwrap())
            .await
            .unwrap();

        assert!(!client
            .check_valid(AccessToken("INVALID_TOKEN".to_string()))
            .await
            .unwrap());
    }

    #[tokio::test]
    async fn login() {
        let mut client = AuthenticationClient::connect(ADDRESS_SERVER.parse().unwrap())
            .await
            .unwrap();

        let random_nickname = get_rand_string(20);
        let old_tokens = client.registration(random_nickname.clone()).await.unwrap();

        let new_access_token = client
            .login(random_nickname, old_tokens.refresh_token)
            .await
            .unwrap();

        assert!(client.check_valid(new_access_token.clone()).await.unwrap());
        assert_ne!(old_tokens.access_token, new_access_token);
    }

    #[tokio::test]
    async fn incorrect_login() {
        let mut client = AuthenticationClient::connect(ADDRESS_SERVER.parse().unwrap())
            .await
            .unwrap();

        let random_nickname = get_rand_string(20);

        assert!(client
            .login(random_nickname, "INVALID_TOKEN".to_string())
            .await
            .is_err());
    }

    #[tokio::test]
    async fn nickname_is_taken() {
        let mut client = AuthenticationClient::connect(ADDRESS_SERVER.parse().unwrap())
            .await
            .unwrap();

        let random_nickname = get_rand_string(20);
        let _tokens = client.registration(random_nickname.clone()).await.unwrap();

        assert!(AuthenticationClient::nickname_is_taken(
            ADDRESS_SERVER.parse().unwrap(),
            random_nickname
        )
        .await
        .unwrap());

        assert!(!AuthenticationClient::nickname_is_taken(
            ADDRESS_SERVER.parse().unwrap(),
            get_rand_string(20)
        )
        .await
        .unwrap());
    }
}
