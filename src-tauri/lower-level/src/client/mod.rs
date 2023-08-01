pub mod crypto;
pub mod error;

use self::error::Error;
use self::security_chat::Check;
use crate::client::crypto::ecdh::{EphemeralSecret, ToEncodedPoint};
use crate::client::security_chat::{
    AesKeyInfo, CheckValidRequest, GetAesKeyRequest, NicknameIsTakenRequest, RegistrationRequest,
    SendAesKeyRequest, SetUserFromAesKeyRequest,
};
use crate::utils::MustBool;
use security_chat::security_chat_client::SecurityChatClient;
use serde::{Deserialize, Serialize};
use tonic::codec::CompressionEncoding;
use tonic::transport::Channel;
use http::uri::Uri;

#[allow(non_snake_case)]
pub mod security_chat {
    tonic::include_proto!("security_chat");
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ClientData {
    pub nickname: String,
    pub auth_key: String,
}

#[derive(Debug)]
pub struct Client {
    pub data: ClientData,
    pub api: SecurityChatClient<Channel>,
}

impl Client {
    pub async fn api_connect(address: Uri) -> Result<SecurityChatClient<Channel>, Error> {
        let channel = Channel::builder(address)
            .connect()
            .await?;

        let api = SecurityChatClient::new(channel)
            .send_compressed(CompressionEncoding::Gzip)
            .accept_compressed(CompressionEncoding::Gzip);

        Ok(api)
    }

    pub async fn registration(nickname: &str, address: Uri) -> Result<Self, Error> {
        let mut api = Client::api_connect(address).await?;
        let request = tonic::Request::new(RegistrationRequest {
            nickname: nickname.to_string(),
        });

        let status = api.registration(request).await?;

        Ok(Self {
            data: ClientData {
                nickname: nickname.to_string(),
                auth_key: status.get_ref().authkey.clone(),
            },
            api,
        })
    }

    pub async fn check_valid(nickname: &str, authkey: &str, address: Uri) -> Result<MustBool, Error> {
        let mut api = Client::api_connect(address).await?;
        let request = tonic::Request::new(CheckValidRequest {
            nickname: nickname.to_string(),
            authkey: authkey.to_string(),
        });

        let response = api.check_valid(request).await?;
        Ok(MustBool::new(response.get_ref().is_valid))
    }

    pub async fn send_aes_key(&mut self, nickname_form: &str) -> Result<EphemeralSecret, Error> {
        assert_ne!(nickname_form, self.data.nickname, "ТЫ СОВСЕМ ЕБНУТЫЙ!?");
        let (secret, public_key) = crypto::ecdh::get_public_info()?;

        let request = tonic::Request::new(SendAesKeyRequest {
            nickname_to: Some(Check {
                nickname: self.data.nickname.clone(),
                authkey: self.data.auth_key.clone(),
            }),
            nickname_from: nickname_form.to_string(),
            public_key: public_key.to_encoded_point(true).as_bytes().to_vec(),
        });

        self.api.send_aes_key(request).await?;

        Ok(secret)
    }

    pub async fn get_aes_keys(&mut self) -> Result<Vec<AesKeyInfo>, Error> {
        let request = tonic::Request::new(GetAesKeyRequest {
            nickname: Some(Check {
                nickname: self.data.nickname.clone(),
                authkey: self.data.auth_key.clone(),
            }),
        });

        let info = self.api.get_aes_key(request).await?;
        Ok(info.get_ref().info.clone())
    }

    pub async fn set_aes_key(&mut self, key_info: &AesKeyInfo) -> Result<EphemeralSecret, Error> {
        let (secret, public_key) = crypto::ecdh::get_public_info()?;
        let request = tonic::Request::new(SetUserFromAesKeyRequest {
            nickname: Some(Check {
                nickname: self.data.nickname.clone(),
                authkey: self.data.auth_key.clone(),
            }),
            id: key_info.id,
            public_key: public_key.to_encoded_point(true).as_bytes().to_vec(),
        });

        self.api.set_user_from_aes_key(request).await.unwrap();
        Ok(secret)
    }
}

pub async fn nickname_is_taken(nickname: &str, address: Uri) -> Result<bool, Error> {
    let mut api = Client::api_connect(address).await?;
    let request = tonic::Request::new(NicknameIsTakenRequest {
        nickname: nickname.to_string(),
    });

    let response = api.nickname_is_taken(request).await?;
    Ok(response.get_ref().is_taken)
}

#[cfg(test)]
mod tests {
    pub const ADDRESS_SERVER: &str = "http://[::1]:2052";
    use super::*;
    use crate::client::crypto::ecdh::PublicKey;
    use crate::test_utils;

    #[tokio::test]
    async fn send_and_get_aes_key() {
        let mut client_to = Client::registration(&test_utils::get_rand_string(), ADDRESS_SERVER.parse().unwrap())
            .await
            .unwrap();
        let mut client_from = Client::registration(&test_utils::get_rand_string(), ADDRESS_SERVER.parse().unwrap())
            .await
            .unwrap();
        println!("client_to data: {:?}", client_to.data);

        let secret_to = client_to
            .send_aes_key(&client_from.data.nickname)
            .await
            .unwrap();
        let keys = client_from.get_aes_keys().await.unwrap();

        println!("keys: {:?}", keys);

        let secter_from = client_from.set_aes_key(&keys[0]).await.unwrap();
        let new_keys = client_from.get_aes_keys().await.unwrap();
        println!("new_keys: {:?}", new_keys);

        let public_from =
            PublicKey::from_sec1_bytes(&new_keys[0].nickname_from_public_key.clone().unwrap()[..])
                .unwrap();
        let public_to =
            PublicKey::from_sec1_bytes(&new_keys[0].nickname_to_public_key.clone()[..]).unwrap();
        let sect = crypto::ecdh::get_shared_secret(&secret_to, &public_from);
        let sss = crypto::ecdh::get_shared_secret(&secter_from, &public_to);

        assert_eq!(sect.0.raw_secret_bytes(), sss.0.raw_secret_bytes());
    }

    #[tokio::test]
    async fn registration() {
        let nickname = test_utils::get_rand_string();
        let client = Client::registration(&nickname, ADDRESS_SERVER.parse().unwrap()).await.unwrap();
        println!("client info: {:?}", client);

        assert_eq!(client.data.auth_key.is_empty(), false);
    }

    #[tokio::test]
    async fn nickname_is_taken() {
        let nickname = test_utils::get_rand_string();
        let result = super::nickname_is_taken(&nickname, ADDRESS_SERVER.parse().unwrap()).await.unwrap();

        assert_eq!(result, false);

        let _client = Client::registration(&nickname, ADDRESS_SERVER.parse().unwrap()).await.unwrap();
        let result = super::nickname_is_taken(&nickname, ADDRESS_SERVER.parse().unwrap()).await.unwrap();

        assert_eq!(result, true);
    }

    #[tokio::test]
    async fn check_valid() {
        let client = Client::registration(&test_utils::get_rand_string(), ADDRESS_SERVER.parse().unwrap())
            .await
            .unwrap();
        let nickname = client.data.nickname.clone();
        let auth_key = client.data.auth_key.clone();
        assert_eq!(auth_key.is_empty(), false);

        drop(client);

        let is_successful = Client::check_valid(&nickname, &auth_key, ADDRESS_SERVER.parse().unwrap()).await.unwrap();
        assert_eq!(*is_successful, true);

        let is_successful = Client::check_valid("dddddd", "dddd", ADDRESS_SERVER.parse().unwrap()).await.unwrap();
        assert_eq!(*is_successful, false);
    }
}
