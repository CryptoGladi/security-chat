//! Main module for API

use crate::client::impl_crypto::ecdh::{EphemeralSecret, ToEncodedPoint};
use crate_proto::*;
use error::Error;
use http::uri::Uri;
use log::*;
use max_size::*;
use serde::{Deserialize, Serialize};
use tonic::codec::CompressionEncoding;
use tonic::transport::Channel;
use tonic::{Response, Streaming};

pub mod error;
pub mod impl_crypto;
pub mod impl_message;
pub mod max_size;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct DataForAutification {
    pub nickname: String,
    pub auth_key: String,
}

#[derive(Debug)]
pub struct Client {
    pub data_for_autification: DataForAutification,
    pub grpc: SecurityChatClient<Channel>,
}

impl Client {
    /// Init [gRPC](https://grpc.io/) connect and enable compression
    pub async fn grpc_connect(address: Uri) -> Result<SecurityChatClient<Channel>, Error> {
        trace!("run `grpc_connect` to address: {}", address);

        let channel = Channel::builder(address).connect().await?;

        let api = SecurityChatClient::new(channel)
            .send_compressed(CompressionEncoding::Gzip)
            .accept_compressed(CompressionEncoding::Gzip);

        Ok(api)
    }

    /// Registration new account
    ///
    /// Please, save `data_for_autification` to secure storage
    ///
    /// Check nickname via [`Client::nickname_is_taken`]
    pub async fn registration(nickname: &str, address: Uri) -> Result<Self, Error> {
        debug!("run `registration` with nickname: {}", nickname);

        let mut api = Client::grpc_connect(address).await?;
        let request = tonic::Request::new(RegistrationRequest {
            nickname: nickname.to_string(),
        });

        let status = api.registration(request).await?;

        Ok(Self {
            data_for_autification: DataForAutification {
                nickname: nickname.to_string(),
                auth_key: status.get_ref().authkey.clone(),
            },
            grpc: api,
        })
    }

    pub async fn check_account_valid(
        nickname: &str,
        authkey: &str,
        address: Uri,
    ) -> Result<bool, Error> {
        trace!("check account valid with nickname: {}", nickname);

        let mut api = Client::grpc_connect(address).await?;
        let request = tonic::Request::new(CheckValidRequest {
            nickname: nickname.to_string(),
            authkey: authkey.to_string(),
        });

        let response = api.check_valid(request).await?;
        Ok(response.get_ref().is_valid)
    }

    pub async fn subscribe_to_notifications(
        &mut self,
    ) -> Result<Response<Streaming<Notification>>, Error> {
        trace!("run `subscribe_to_notifications`");

        let request = tonic::Request::new(Check {
            nickname: self.data_for_autification.nickname.clone(),
            authkey: self.data_for_autification.auth_key.clone(),
        });

        Ok(self.grpc.subscribe(request).await?)
    }

    pub async fn nickname_is_taken(nickname: &str, address: Uri) -> Result<bool, Error> {
        trace!("run `nickname_is_taken` with nickname: {}", nickname);

        let mut api = Client::grpc_connect(address).await?;
        let request = tonic::Request::new(NicknameIsTakenRequest {
            nickname: nickname.to_string(),
        });

        let response = api.nickname_is_taken(request).await?;
        Ok(response.get_ref().is_taken)
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
        println!("client info: {:?}", client);

        assert!(!client.data_for_autification.auth_key.is_empty());
    }

    #[tokio::test]
    async fn nickname_is_taken() {
        let nickname = get_rand_string(20);
        let result = super::Client::nickname_is_taken(&nickname, ADDRESS_SERVER.parse().unwrap())
            .await
            .unwrap();

        assert!(!result);

        let _client = Client::registration(&nickname, ADDRESS_SERVER.parse().unwrap())
            .await
            .unwrap();
        let result = super::Client::nickname_is_taken(&nickname, ADDRESS_SERVER.parse().unwrap())
            .await
            .unwrap();

        assert!(result);
    }

    #[tokio::test]
    async fn check_account_valid() {
        let client = get_client().await.unwrap();
        let nickname = client.data_for_autification.nickname.clone();
        let auth_key = client.data_for_autification.auth_key.clone();
        assert!(!auth_key.is_empty());

        drop(client);

        let is_successful =
            Client::check_account_valid(&nickname, &auth_key, ADDRESS_SERVER.parse().unwrap())
                .await
                .unwrap();
        assert!(is_successful);

        let is_successful =
            Client::check_account_valid("dddddd", "dddd", ADDRESS_SERVER.parse().unwrap())
                .await
                .unwrap();
        assert!(!is_successful);
    }
}
