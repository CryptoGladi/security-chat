use self::notification::Notification;
use crate::bincode_config;
use client_config::{ClientConfig, ClientInitConfig};
use error::Error;
use kanal::AsyncReceiver;
use log::info;
use lower_level::client::{
    crypto::{
        ecdh::{get_shared_secret, EphemeralSecretDef, PublicKey},
        Aes,
    },
    Client as RawClient,
};
use storage_crypto::Nickname;

pub mod client_config;
pub mod error;
pub mod impl_crypto;
pub mod impl_message;
pub mod notification;
pub mod storage_crypto;

#[derive(Debug)]
pub struct Client {
    raw_client: RawClient,
    config: ClientConfig,
    init_config: ClientInitConfig,
}

impl Client {
    pub async fn registration(
        nickname: &str,
        init_config: ClientInitConfig,
    ) -> Result<Client, Error> {
        info!("run registration");
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

    pub fn have_account(init_config: ClientInitConfig) -> Result<bool, Error> {
        Ok(init_config.path_to_config_file.is_file())
    }

    pub async fn load(init_config: ClientInitConfig) -> Result<Client, Error> {
        info!("run load");
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

    pub fn save(&self) -> Result<(), Error> {
        info!("run save");
        bincode_config::save(&self.config, &self.init_config.path_to_config_file)?;
        Ok(())
    }

    pub fn get_nickname(&self) -> Nickname {
        Nickname(self.config.client_data.nickname.clone())
    }

    pub async fn subscribe(&mut self) -> Result<AsyncReceiver<Notification>, Error> {
        info!("run subscribe");
        let mut subscribe = self.raw_client.subscribe().await?;
        let (send, recv) = kanal::unbounded_async();
        let storage_crypto = self.config.storage_crypto.clone();

        tokio::spawn(async move {
            loop {
                // TODO при добавление нового ключа storage crypto НЕ ОБНОВЛЯЕТСЯ ВНУТРИ LOOP!
                // storage_crypto.add(Nickname("ss".to_string()), Aes::generate()).unwrap();
                // ЕДИНСТВЕННЫЙ ВЫХОД - при update надо перезапускать subscribe
                let notify = subscribe.get_mut().message().await.unwrap().unwrap();
                if send
                    .send(Client::nofity(&storage_crypto, notify).unwrap())
                    .await
                    .is_err()
                {
                    break;
                }
            }
        });

        Ok(recv)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{client::impl_message::Message, test_utils::get_rand_string};
    use std::path::PathBuf;
    use temp_dir::TempDir;

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
    async fn send_many_message_with_subscribe() {
        let (_paths, _, mut client_to) = get_client!();
        let (_paths, _, mut client_from) = get_client!();
        client_to
            .send_crypto(client_from.get_nickname())
            .await
            .unwrap();

        client_from.accept_all_cryptos().await.unwrap();
        client_to.update_cryptos().await.unwrap();

        const TEST_MESSAGE: &str = "MANY MESSAGES";
        const LEN: usize = 100;

        let recv = client_from.subscribe().await.unwrap();

        for _ in 0..LEN {
            client_to
                .send_message(
                    client_from.get_nickname(),
                    Message {
                        text: TEST_MESSAGE.to_string(),
                    },
                )
                .await
                .unwrap();

            let new_event = recv.recv().await.unwrap();
            println!("new event: {:?}", new_event);
        }
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

        println!("nickname_to: {}", client_to.raw_client.data.nickname);
        println!("nickname_from: {}", client_from.raw_client.data.nickname);

        const TEST_MESSAGE: &str = "Фёдор, я тебя очень сильно люблю";

        let recv = client_from.subscribe().await.unwrap();

        client_to
            .send_message(
                client_from.get_nickname(),
                Message {
                    text: TEST_MESSAGE.to_string(),
                },
            )
            .await
            .unwrap();

        let new_event = recv.recv().await.unwrap();
        println!("new event: {:?}", new_event);

        match new_event.event {
            notification::Event::NewMessage(message) => {
                assert_eq!(message.text, TEST_MESSAGE);
            }
        }
    }

    #[tokio::test]
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
