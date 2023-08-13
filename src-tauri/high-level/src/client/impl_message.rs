use super::{storage_crypto::StorageCrypto, *};
use lower_level::client::crypto::EncryptedMessage;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Message {
    pub text: String,
}

impl Client {
    pub async fn send_message(
        &mut self,
        nickname_from: Nickname,
        message: Message,
    ) -> Result<(), Error> {
        info!("run send_message");

        if self.get_nickname() == nickname_from {
            return Err(Error::SendMessageToYourself);
        }

        let aes = *self
            .config
            .storage_crypto
            .read()
            .unwrap()
            .get(&nickname_from)?;
        let bincode = bincode::serialize(&message)?;
        let encryptred_data = aes.encrypt(&bincode[..])?;

        self.raw_client
            .send_message(
                nickname_from.0,
                crate_proto::Message {
                    body: encryptred_data.data,
                    nonce: encryptred_data.nonce.to_vec(),
                },
            )
            .await?;

        Ok(())
    }

    pub async fn get_messages_for_user(
        &mut self,
        nickname: Nickname,
        limit: i64,
    ) -> Result<Vec<Message>, Error> {
        self.raw_get_last_message(vec![nickname.0], limit).await
    }

    pub(crate) fn decrypt_message(
        storage_crypto: &StorageCrypto,
        message: crate_proto::Message,
        nickname_from: Nickname,
    ) -> Result<Message, Error> {
        info!("run decrypt_message");
        let aes = storage_crypto.get(&nickname_from)?;
        let decrypted_body = aes.decrypt(&EncryptedMessage {
            data: message.body,
            nonce: message.nonce.try_into().unwrap(),
        })?;

        let message = bincode::deserialize(&decrypted_body)?;
        Ok(message)
    }

    async fn raw_get_last_message(
        &mut self,
        nicknames: Vec<String>,
        limit: i64,
    ) -> Result<Vec<Message>, Error> {
        let messages = self
            .raw_client
            .get_latest_messages(nicknames, limit)
            .await?;

        Ok(messages
            .messages
            .into_iter()
            .map(|x| {
                let nickname = Nickname::from(if x.sender_nickname == *self.get_nickname() {
                    x.recipient_nickname
                } else {
                    x.sender_nickname
                });

                Client::decrypt_message(
                    &self.config.storage_crypto.read().unwrap(),
                    x.body.unwrap(),
                    nickname,
                )
                .unwrap()
            })
            .collect::<Vec<Message>>())
    }

    pub async fn get_all_last_message(&mut self) -> Result<Vec<Message>, Error> {
        info!("run `get_all_last_message`");
        let nicknames = self
            .config
            .storage_crypto
            .read()
            .unwrap()
            .0
            .keys()
            .cloned()
            .map(|x| x.0)
            .collect();

        self.raw_get_last_message(nicknames, 1).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::impl_message::Message;
    use crate::test_utils::get_client;
    use test_log::test;

    #[test(tokio::test)]
    async fn send_many_message_with_subscribe() {
        let (_paths, _, mut client_to) = get_client().await;
        let (_paths, _, mut client_from) = get_client().await;
        client_to
            .send_crypto(client_from.get_nickname())
            .await
            .unwrap();

        client_from.accept_all_cryptos().await.unwrap();
        client_to.update_cryptos().await.unwrap();

        const TEXT_MESSAGE: &str = "MANY MESSAGES";
        const LEN: usize = 50;

        let recv = client_from.subscribe().await.unwrap();

        for _ in 0..LEN {
            client_to
                .send_message(
                    client_from.get_nickname(),
                    Message {
                        text: TEXT_MESSAGE.to_string(),
                    },
                )
                .await
                .unwrap();

            let new_event = recv.recv().await.unwrap();
            println!("new event: {:?}", new_event);
        }
    }

    #[test(tokio::test)]
    async fn send_message_with_subscribe() {
        let (_paths, _, mut client_to) = get_client().await;
        let (_paths, _, mut client_from) = get_client().await;

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
            notification::Event::NewMessage(message) => assert_eq!(message.text, TEST_MESSAGE),
            _ => panic!("event is invalid"),
        }
    }

    #[test(tokio::test)]
    async fn get_one_last_message() {
        let (_paths, _, mut client_to) = get_client().await;
        let (_paths, _, mut client_from) = get_client().await;

        client_to
            .send_crypto(client_from.get_nickname())
            .await
            .unwrap();

        client_from.accept_all_cryptos().await.unwrap();
        client_to.update_cryptos().await.unwrap();

        client_to
            .send_message(
                client_from.get_nickname(),
                Message {
                    text: "ss".to_owned(),
                },
            )
            .await
            .unwrap();
        let last_messages = client_from.get_all_last_message().await.unwrap();

        assert_eq!(last_messages[0].text, "ss");
        assert_eq!(last_messages.len(), 1);
    }

    #[test(tokio::test)]
    async fn get_one_last_messages_but_many_send() {
        let (_paths, _, mut client_to) = get_client().await;
        let (_paths, _, mut client_from) = get_client().await;

        client_to
            .send_crypto(client_from.get_nickname())
            .await
            .unwrap();

        client_from.accept_all_cryptos().await.unwrap();
        client_to.update_cryptos().await.unwrap();

        for (i, _) in (0..100).enumerate() {
            let text = format!("x: {}", i);
            client_to
                .send_message(client_from.get_nickname(), Message { text })
                .await
                .unwrap();
        }

        let last_messages = client_from.get_all_last_message().await.unwrap();

        assert_eq!(last_messages[0].text, "x: 99");
        assert_eq!(last_messages.len(), 1);
    }

    #[test(tokio::test)]
    async fn get_one_last_messages_with_client_from_last() {
        let (_paths, _, mut client_to) = get_client().await;
        let (_paths, _, mut client_from) = get_client().await;

        client_to
            .send_crypto(client_from.get_nickname())
            .await
            .unwrap();

        client_from.accept_all_cryptos().await.unwrap();
        client_to.update_cryptos().await.unwrap();

        client_to
            .send_message(
                client_from.get_nickname(),
                Message {
                    text: "ss".to_owned(),
                },
            )
            .await
            .unwrap();
        client_from
            .send_message(
                client_to.get_nickname(),
                Message {
                    text: "tttt".to_owned(),
                },
            )
            .await
            .unwrap();
        let last_messages = client_from.get_all_last_message().await.unwrap();

        assert_eq!(last_messages[0].text, "tttt");
        assert_eq!(last_messages.len(), 1);
    }

    #[test(tokio::test)]
    async fn send_message_to_yourself() {
        let (_paths, _, mut client_to) = get_client().await;
        let error = client_to
            .send_message(
                client_to.get_nickname(),
                Message {
                    text: "test".to_string(),
                },
            )
            .await
            .err()
            .unwrap();

        assert!(matches!(error, Error::SendMessageToYourself));
    }

    #[test(tokio::test)]
    async fn get_many_last_messages() {
        let (_paths, _, mut client_to) = get_client().await;
        let (_paths, _, mut client_from) = get_client().await;

        client_to
            .send_crypto(client_from.get_nickname())
            .await
            .unwrap();

        client_from.accept_all_cryptos().await.unwrap();
        client_to.update_cryptos().await.unwrap();

        let mut sent_messages = vec![];
        for _ in 0..100 {
            let new_message = Message {
                text: "manyyy".to_owned(),
            };
            sent_messages.push(new_message.clone());

            client_to
                .send_message(client_from.get_nickname(), new_message)
                .await
                .unwrap();
        }

        let messages = client_from
            .get_messages_for_user(client_to.get_nickname(), 100)
            .await
            .unwrap();

        assert_eq!(messages.len(), sent_messages.len());
        assert_eq!(messages, sent_messages);
    }
}
