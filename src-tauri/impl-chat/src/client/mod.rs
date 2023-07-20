pub(crate) mod crypto;
pub mod error;

use self::error::Error;
use crate::client::security_chat::{NicknameIsTakenRequest, RegistrationRequest};
use crypto::Crypto;
use security_chat::security_chat_client::SecurityChatClient;
use std::collections::HashMap;
use tonic::transport::Channel;

pub const ADDRESS_SERVER: &str = "http://[::1]:2052";

pub mod security_chat {
    tonic::include_proto!("security_chat");
}

pub struct Client {
    pub cryptos_strorage: HashMap<String, Crypto>,
    pub nickname: String,
    pub auth_key: String,
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

        if status.get_ref().code == 0 {
            Ok(Self {
                cryptos_strorage: HashMap::default(),
                nickname,
                auth_key: "sasa".to_string(), // TODO
                api,
            })
        } else {
            Err(Error::NicknameIsTaken)
        }
    }
}

pub async fn nickname_is_taken(nickname: String) -> Result<bool, Error> {
    let mut api = SecurityChatClient::connect(ADDRESS_SERVER).await?;
    let request = tonic::Request::new(NicknameIsTakenRequest { nickname });

    let response = api.nickname_is_taken(request).await?;
    println!("{:?}", response.get_ref().is_taken);
    Ok(response.get_ref().is_taken)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn registration() {
        let _client = Client::registration("test_nickname".to_string())
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn nickname_is_taken() {
        super::nickname_is_taken("nickname dont taken".to_string())
            .await
            .unwrap();
    }
}
