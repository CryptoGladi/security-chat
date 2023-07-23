pub mod config;
pub(crate) mod crypto;
pub mod error;

use self::error::Error;
use crate::client::security_chat::{LoginRequest, NicknameIsTakenRequest, RegistrationRequest};
use crypto::Crypto;
use security_chat::security_chat_client::SecurityChatClient;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tonic::codec::CompressionEncoding;
use tonic::transport::Channel;

pub const ADDRESS_SERVER: &str = "http://[::1]:2052";

pub mod security_chat {
    tonic::include_proto!("security_chat");
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientData {
    pub cryptos_strorage: HashMap<String, Crypto>,
    pub nickname: String,
    pub auth_key: String,
}

#[derive(Debug)]
pub struct Client {
    pub data: ClientData,
    pub api: SecurityChatClient<Channel>,
}

impl Client {
    async fn api_connect() -> Result<SecurityChatClient<Channel>, Error> {
        let channel = Channel::builder(ADDRESS_SERVER.parse().unwrap())
            .connect()
            .await
            .unwrap();

        let api = SecurityChatClient::new(channel)
            .send_compressed(CompressionEncoding::Gzip)
            .accept_compressed(CompressionEncoding::Gzip);

        Ok(api)
    }

    pub async fn registration(nickname: &str) -> Result<Self, Error> {
        let mut api = Client::api_connect().await?;
        let request = tonic::Request::new(RegistrationRequest {
            nickname: nickname.to_string(),
        });

        let status = api.registration(request).await?;

        // TODO
        if !status.get_ref().authkey.is_empty() {
            Ok(Self {
                data: ClientData {
                    cryptos_strorage: HashMap::default(),
                    nickname: nickname.to_string(),
                    auth_key: status.get_ref().authkey.clone(),
                },
                api,
            })
        } else {
            Err(Error::NicknameIsTaken)
        }
    }

    pub async fn login(nickname: &str, authkey: &str) -> Result<bool, Error> {
        let mut api = Client::api_connect().await?;
        let request = tonic::Request::new(LoginRequest {
            nickname: nickname.to_string(),
            authkey: authkey.to_string(),
        });

        let status = api.login(request).await?;

        Ok(status.get_ref().is_successful)
    }
}

pub async fn nickname_is_taken(nickname: &str) -> Result<bool, Error> {
    let mut api = Client::api_connect().await?;
    let request = tonic::Request::new(NicknameIsTakenRequest {
        nickname: nickname.to_string(),
    });

    let response = api.nickname_is_taken(request).await?;
    Ok(response.get_ref().is_taken)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn registration() {
        let client = Client::registration("test_nickname").await.unwrap();
        println!("client info: {:?}", client);

        assert_eq!(client.data.auth_key.is_empty(), false);
    }

    #[tokio::test]
    async fn nickname_is_taken() {
        let result = super::nickname_is_taken("nickname_dont_taken")
            .await
            .unwrap();
        assert_eq!(result, false);
    }

    #[tokio::test]
    async fn login() {
        let client = Client::registration("nickname_for_login").await.unwrap();
        let nickname = client.data.nickname.clone();
        let auth_key = client.data.auth_key.clone();
        assert_eq!(auth_key.is_empty(), false);

        drop(client);

        let is_successful = Client::login(&nickname, &auth_key).await.unwrap();
        assert_eq!(is_successful, true);
    }
}
