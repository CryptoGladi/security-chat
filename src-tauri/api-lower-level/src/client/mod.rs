//! Main module for API

use crate::authentication::tokens::{AccessToken, RefreshToken};
use crate::authentication::AuthenticationClient;
use crate::client::impl_crypto::ecdh::{EphemeralSecret, ToEncodedPoint};
use crate_proto::{
    AesKeyInfo, DeleteAesKeyRequest, GetLatestMessagesReply, GetLatestMessagesRequest, Message,
    Notification, SecurityChatClient, SendAesKeyRequest, SendMessageRequest,
    SetUserFromAesKeyRequest,
};
use error::Error;
use http::uri::Uri;
use log::{debug, trace};
use max_size::{MAX_LEN_MESSAGE, MAX_LIMIT_GET_MESSAGES};
use serde::{Deserialize, Serialize};
use tonic::transport::Channel;
use tonic::{Response, Streaming};

pub mod error;
pub mod impl_crypto;
pub mod impl_message;
pub mod max_size;

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct DataForAutification {
    pub nickname: String,
    pub refresh_token: RefreshToken,
}

#[derive(Debug)]
pub struct Client {
    pub data_for_autification: DataForAutification,
    pub grpc: SecurityChatClient<Channel>,
    pub access_token: AccessToken,
}

impl Client {
    fn add_access_token_to_metadata<T>(&self, mut request: tonic::Request<T>) -> tonic::Request<T> {
        request
            .metadata_mut()
            .insert("access_token", self.access_token.0.parse().unwrap());

        request
    }

    /// Init [gRPC](https://grpc.io/) connect and enable compression
    pub async fn grpc_connect(address: Uri) -> Result<SecurityChatClient<Channel>, Error> {
        trace!("run `grpc_connect` to address: {}", address);

        let channel = Channel::builder(address).connect().await?;
        let api = SecurityChatClient::new(channel);

        Ok(api)
    }

    /// Registration new account
    ///
    /// Please, save `data_for_autification` to secure storage
    ///
    /// Check nickname via [`Client::nickname_is_taken`]
    pub async fn registration(nickname: &str, address: Uri) -> Result<Self, Error> {
        debug!("run `registration` with nickname: {}", nickname);

        let mut authentication = AuthenticationClient::connect(address.clone()).await?;
        let tokens = authentication.registration(nickname.to_string()).await?;

        let api = Self::grpc_connect(address).await?;

        Ok(Self {
            data_for_autification: DataForAutification {
                nickname: nickname.to_string(),
                refresh_token: tokens.refresh_token,
            },
            grpc: api,
            access_token: tokens.access_token,
        })
    }

    pub async fn login(
        address: Uri,
        nickname: String,
        refresh_token: RefreshToken,
    ) -> Result<Self, Error> {
        debug!("run `login`");

        let mut authentication = AuthenticationClient::connect(address.clone()).await?;
        let access_token = authentication
            .login(nickname.clone(), refresh_token.clone())
            .await?;

        let api = Self::grpc_connect(address).await?;

        Ok(Self {
            data_for_autification: DataForAutification {
                nickname,
                refresh_token,
            },
            grpc: api,
            access_token,
        })
    }

    pub async fn subscribe_to_notifications(
        &mut self,
    ) -> Result<Response<Streaming<Notification>>, Error> {
        trace!("run `subscribe_to_notifications`");

        let request = tonic::Request::new(());
        Ok(self
            .grpc
            .subscribe(self.add_access_token_to_metadata(request))
            .await?)
    }

    pub async fn nickname_is_taken(nickname: &str, address: Uri) -> Result<bool, Error> {
        trace!("run `nickname_is_taken` with nickname: {}", nickname);
        Ok(AuthenticationClient::nickname_is_taken(address, nickname.to_string()).await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::get_client;
    use fcore::test_utils::*;

    #[tokio::test]
    async fn grpc_connect() {
        let _grpc = Client::grpc_connect(ADDRESS_SERVER.parse().unwrap())
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn subscribe_to_notifications() {
        let mut client = get_client().await.unwrap();

        let _notification = client.subscribe_to_notifications().await.unwrap();
    }

    #[tokio::test]
    async fn registration() {
        let nickname = get_rand_string(20);
        let client = Client::registration(&nickname, ADDRESS_SERVER.parse().unwrap())
            .await
            .unwrap();

        log::info!("client info: {client:?}");
        assert!(!client.data_for_autification.refresh_token.is_empty());
    }

    #[tokio::test]
    async fn login() {
        let nickname = get_rand_string(20);
        let client = Client::registration(&nickname, ADDRESS_SERVER.parse().unwrap())
            .await
            .unwrap();

        let data_for_auth = client.data_for_autification.clone();
        drop(client);

        let _client = Client::login(
            ADDRESS_SERVER.parse().unwrap(),
            data_for_auth.nickname.clone(),
            data_for_auth.refresh_token,
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn nickname_is_taken() {
        let nickname = get_rand_string(20);
        let result = Client::nickname_is_taken(&nickname, ADDRESS_SERVER.parse().unwrap())
            .await
            .unwrap();

        assert!(!result);

        let _client = Client::registration(&nickname, ADDRESS_SERVER.parse().unwrap())
            .await
            .unwrap();
        let result = Client::nickname_is_taken(&nickname, ADDRESS_SERVER.parse().unwrap())
            .await
            .unwrap();

        assert!(result);
    }
}
