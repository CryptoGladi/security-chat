use crate::bincode_config;
use client_config::{ClientConfig, ClientInitConfig};
use error::Error;
use lower_level::client::{
    crypto::{
        ecdh::{get_shared_secret, EphemeralSecretDef, PublicKey},
        Aes,
    },
    Client as RawClient,
};
use storage_crypto::Nickname;
use self::notification::Notification;

pub mod client_config;
pub mod error;
pub mod impl_crypto;
pub mod impl_message;
pub mod storage_crypto;
pub mod notification;

#[derive(Debug)]
pub struct Client {
    raw_client: RawClient,
    config: ClientConfig,
    init_config: ClientInitConfig,
}

impl Client {
    #[tracing::instrument]
    pub async fn registration(
        nickname: &str,
        init_config: ClientInitConfig,
    ) -> Result<Client, Error> {
        let raw_client =
            RawClient::registration(nickname, init_config.address_to_server.clone()).await?;

        Ok(Self {
            config: ClientConfig {
                client_data: raw_client.data.clone(),
                ..Default::default()
            },
            init_config,
            raw_client,
        })
    }

    #[tracing::instrument]
    pub async fn load(init_config: ClientInitConfig) -> Result<Client, Error> {
        let config: ClientConfig = bincode_config::load(init_config.path_to_config_file.clone())?;
        let api = RawClient::api_connect(init_config.address_to_server.clone()).await?;

        if !*RawClient::check_valid(
            &config.client_data.nickname,
            &config.client_data.auth_key,
            init_config.address_to_server.clone(),
        )
        .await?
        {
            return Err(Error::AccoutIsInvalid);
        }

        Ok(Self {
            raw_client: RawClient {
                api,
                data: config.client_data.clone(),
            },
            config,
            init_config,
        })
    }

    #[tracing::instrument]
    pub fn save(&self) -> Result<(), Error> {
        bincode_config::save(&self.config, &self.init_config.path_to_config_file)?;
        Ok(())
    }

    #[tracing::instrument]
    pub fn get_nickname(&self) -> Nickname {
        Nickname(self.config.client_data.nickname.clone())
    }

    pub async fn subscribe(&mut self, mut f: impl FnMut(Notification)) -> Result<(), Error> {
        let mut subscribe = self.raw_client.subscribe().await?;

        loop {
            let notify = subscribe.get_mut().message().await.unwrap().unwrap();
            f(Client::nofity(self, notify)?)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{client::impl_message::Message, test_utils::get_rand_string};
    use std::{path::PathBuf, time::Duration, sync::Arc};
    use temp_dir::TempDir;
    use std::sync::Mutex;
    use tracing_test::traced_test;

    pub const ADDRESS_SERVER: &str = "http://[::1]:2052";

    struct PathsForTest {
        _temp_dir: TempDir, // for lifetime
        path_to_config_file: PathBuf,
    }

    impl PathsForTest {
        fn get() -> Self {
            let temp_dir = TempDir::new().unwrap();

            Self {
                path_to_config_file: temp_dir.child("config.bin"),
                _temp_dir: temp_dir,
            }
        }
    }

    macro_rules! get_client {
        () => {{
            let paths = PathsForTest::get();
            let client_config =
                ClientInitConfig::new(paths.path_to_config_file.clone(), ADDRESS_SERVER);
            let client = Client::registration(&get_rand_string(), client_config.clone())
                .await
                .unwrap();

            (paths, client_config, client)
        }};
    }

    #[tokio::test]
    async fn send_message_with_subscribe() {
        let (_paths, _, mut client_to) = get_client!();
        let (_paths, _, mut client_from) = get_client!();

        client_to
            .send_crypto(client_from.get_nickname())
            .await
            .unwrap();

        client_from.accept_all_cryptos().await.unwrap();
        client_to.update_cryptos().await.unwrap();
        let nickname_from = client_from.get_nickname();
        let done = Arc::new(Mutex::new(false));

        println!("nickname_to: {}", client_to.raw_client.data.nickname);
        println!("nickname_from: {}", client_from.raw_client.data.nickname);

        const TEST_MESSAGE: &str = "Фёдор, я тебя очень сильно люблю";

        let done_clone = done.clone();
        tokio::spawn(async move {
            client_from
                .subscribe(|new_notification| {
                    println!("n: {:?}", new_notification);
                    
                    let notification::Event::NewMessage(message) = new_notification.event;
                    if message.text == TEST_MESSAGE.to_string() {
                        *done_clone.lock().unwrap() = true;
                    }
                })
                .await
                .unwrap();
        });
        tokio::time::sleep(Duration::from_secs(1)).await;

        client_to
            .send_message(
                nickname_from,
                Message {
                    text: TEST_MESSAGE.to_string(),
                },
            )
            .await
            .unwrap();
        tokio::time::sleep(Duration::from_secs(1)).await;

        assert_eq!(*done.lock().unwrap(), true);
    }

    #[tokio::test]
    #[traced_test]
    async fn save_and_load() {
        let (_paths, client_config, client) = get_client!();

        client.save().unwrap();

        let loaded_client = Client::load(client_config).await.unwrap();
        println!("loaded_client data: {:#?}", loaded_client.raw_client.data);
        println!("client data: {:#?}", client.raw_client.data);
        assert_eq!(
            loaded_client.raw_client.data.nickname,
            client.raw_client.data.nickname
        )
    }

    #[tokio::test]
    #[traced_test]
    async fn shared_keys() {
        let (_paths, _, mut client_to) = get_client!();
        let (_paths, _, mut client_from) = get_client!();

        client_to
            .send_crypto(client_from.get_nickname())
            .await
            .unwrap();

        client_from.accept_all_cryptos().await.unwrap();
        client_to.update_cryptos().await.unwrap();

        // Проверка ключей
        println!("nickname_to: {}", client_to.get_nickname());
        println!("client_to: {:?}", client_to.config.storage_crypto);
        println!("nickname_from: {}", client_from.get_nickname());
        println!("client_from: {:?}", client_from.config.storage_crypto);
        assert_eq!(
            client_to
                .config
                .storage_crypto
                .get(&client_from.get_nickname())
                .unwrap(),
            client_from
                .config
                .storage_crypto
                .get(&client_to.get_nickname())
                .unwrap()
        );
    }
}
