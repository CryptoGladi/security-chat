use crate::client::crypto::ecdh::{EphemeralSecret, ToEncodedPoint};
use crate::utils::MustBool;
use crate_proto::*;
use error::Error;
use http::uri::Uri;
use max_size::*;
use serde::{Deserialize, Serialize};
use tonic::codec::CompressionEncoding;
use tonic::transport::Channel;
use tonic::{Response, Streaming};

pub mod crypto;
pub mod error;
pub mod impl_aes;
pub mod impl_message;
pub mod max_size;

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
        let channel = Channel::builder(address).connect().await?;

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

    pub async fn check_valid(
        nickname: &str,
        authkey: &str,
        address: Uri,
    ) -> Result<MustBool, Error> {
        let mut api = Client::api_connect(address).await?;
        let request = tonic::Request::new(CheckValidRequest {
            nickname: nickname.to_string(),
            authkey: authkey.to_string(),
        });

        let response = api.check_valid(request).await?;
        Ok(MustBool::new(response.get_ref().is_valid))
    }

    pub async fn subscribe(&mut self) -> Result<Response<Streaming<Notification>>, Error> {
        let request = tonic::Request::new(Check {
            nickname: self.data.nickname.clone(),
            authkey: self.data.auth_key.clone(),
        });

        Ok(self.api.subscribe(request).await?)
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
    async fn too_big_message() {
        let mut client_to = Client::registration(
            &test_utils::get_rand_string(20),
            ADDRESS_SERVER.parse().unwrap(),
        )
        .await
        .unwrap();
        let client_from = Client::registration(
            &test_utils::get_rand_string(20),
            ADDRESS_SERVER.parse().unwrap(),
        )
        .await
        .unwrap();

        let text = test_utils::get_rand_string(MAX_LEN_MESSAGE + 100);
        let error = client_to
            .send_message(
                client_from.data.nickname,
                Message {
                    body: text.into_bytes(),
                    nonce: vec![],
                },
            )
            .await
            .err()
            .unwrap();

        assert!(matches!(error, Error::TooBigMessage));
    }

    #[tokio::test]
    async fn send_message_and_subscribe() {
        let mut client_to = Client::registration(
            &test_utils::get_rand_string(20),
            ADDRESS_SERVER.parse().unwrap(),
        )
        .await
        .unwrap();
        let mut client_from = Client::registration(
            &test_utils::get_rand_string(20),
            ADDRESS_SERVER.parse().unwrap(),
        )
        .await
        .unwrap();
        const TEST_MESSAGE: &[u8] = b"What hath God wrought!";

        let mut notification = client_from.subscribe().await.unwrap();
        client_to
            .send_message(
                client_from.data.nickname.clone(),
                Message {
                    body: TEST_MESSAGE.to_vec(),
                    nonce: vec![],
                },
            )
            .await
            .unwrap();

        let Some(notify) = notification.get_mut().message().await.unwrap() else {
            panic!("not found notification");
        };

        let Notice::NewMessage(new_message) = notify.notice.unwrap() else {
            panic!();
        };

        println!("new_message: {:?}", new_message);
        println!("nickname_from: {}", notify.nickname_from);
        println!("client_from: {}", client_from.data.nickname);
        println!("client_to: {}", client_to.data.nickname);
        assert_eq!(new_message.message.unwrap().body, TEST_MESSAGE);
        assert_eq!(client_from.data.nickname, notify.nickname_from);
    }

    #[tokio::test]
    async fn send_and_get_aes_key() {
        let mut client_to = Client::registration(
            &test_utils::get_rand_string(20),
            ADDRESS_SERVER.parse().unwrap(),
        )
        .await
        .unwrap();
        let mut client_from = Client::registration(
            &test_utils::get_rand_string(20),
            ADDRESS_SERVER.parse().unwrap(),
        )
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
        let nickname = test_utils::get_rand_string(20);
        let client = Client::registration(&nickname, ADDRESS_SERVER.parse().unwrap())
            .await
            .unwrap();
        println!("client info: {:?}", client);

        assert!(!client.data.auth_key.is_empty());
    }

    #[tokio::test]
    async fn nickname_is_taken() {
        let nickname = test_utils::get_rand_string(20);
        let result = super::nickname_is_taken(&nickname, ADDRESS_SERVER.parse().unwrap())
            .await
            .unwrap();

        assert!(!result);

        let _client = Client::registration(&nickname, ADDRESS_SERVER.parse().unwrap())
            .await
            .unwrap();
        let result = super::nickname_is_taken(&nickname, ADDRESS_SERVER.parse().unwrap())
            .await
            .unwrap();

        assert!(result);
    }

    async fn check_limit(size: i64) {
        let nickname = test_utils::get_rand_string(20);
        let mut client = Client::registration(&nickname, ADDRESS_SERVER.parse().unwrap())
            .await
            .unwrap();

        client
            .get_latest_messages(vec![test_utils::get_rand_string(20)], size)
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic]
    async fn get_latest_messages_to_big_limit() {
        const LIMIT: i64 = max_size::MAX_LIMIT_GET_MESSAGES + 100;
        check_limit(LIMIT).await;
    }

    #[tokio::test]
    #[should_panic]
    async fn get_latest_messages_to_zero_limit() {
        const LIMIT: i64 = 0;
        check_limit(LIMIT).await;
    }

    #[tokio::test]
    #[should_panic]
    async fn get_latest_messages_to_negative_limit() {
        const LIMIT: i64 = -1;

        let nickname = test_utils::get_rand_string(20);
        let mut client = Client::registration(&nickname, ADDRESS_SERVER.parse().unwrap())
            .await
            .unwrap();

        client
            .get_latest_messages(vec![test_utils::get_rand_string(20)], LIMIT)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn check_valid() {
        let client = Client::registration(
            &test_utils::get_rand_string(20),
            ADDRESS_SERVER.parse().unwrap(),
        )
        .await
        .unwrap();
        let nickname = client.data.nickname.clone();
        let auth_key = client.data.auth_key.clone();
        assert!(!auth_key.is_empty());

        drop(client);

        let is_successful =
            Client::check_valid(&nickname, &auth_key, ADDRESS_SERVER.parse().unwrap())
                .await
                .unwrap();
        assert!(*is_successful);

        let is_successful = Client::check_valid("dddddd", "dddd", ADDRESS_SERVER.parse().unwrap())
            .await
            .unwrap();
        assert!(!*is_successful);
    }
}
