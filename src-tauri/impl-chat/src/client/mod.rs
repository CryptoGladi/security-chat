pub mod config;
pub(crate) mod crypto;
pub mod error;

use self::error::Error;
use crate::client::security_chat::{NicknameIsTakenRequest, RegistrationRequest};
use crypto::Crypto;
use security_chat::security_chat_client::SecurityChatClient;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
    pub async fn registration(nickname: String) -> Result<Self, Error> {
        if nickname_is_taken(nickname.clone()).await? {
            return Err(Error::NicknameIsTaken);
        }

        let mut api = SecurityChatClient::connect(ADDRESS_SERVER).await?;
        let request = tonic::Request::new(RegistrationRequest {
            nickname: nickname.clone(),
        });

        let status = api.registration(request).await?;

        // TODO
        if !status.get_ref().authkey.is_empty() {
            Ok(Self {
                data: ClientData {
                    cryptos_strorage: HashMap::default(),
                    nickname,
                    auth_key: status.get_ref().authkey.clone(),
                },
                api,
            })
        } else {
            Err(Error::NicknameIsTaken)
        }
    }

    pub async fn login(nickname: String, authkey: String) -> Result<Self, Error> {
        todo!()
    }
}

pub async fn nickname_is_taken(nickname: String) -> Result<bool, Error> {
    let mut api = SecurityChatClient::connect(ADDRESS_SERVER).await?;
    let request = tonic::Request::new(NicknameIsTakenRequest { nickname });

    let response = api.nickname_is_taken(request).await?;
    Ok(response.get_ref().is_taken)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn registration() {
        let client = Client::registration("test_nickname".to_string())
            .await
            .unwrap();
        println!("client info: {:?}", client);
    }

    #[tokio::test]
    async fn nickname_is_taken() {
        let result = super::nickname_is_taken("nickname_dont_taken".to_string())
            .await
            .unwrap();
        assert_eq!(result, false);
    }
}
