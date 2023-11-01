use super::{storage_crypto::StorageCrypto, *};
use lower_level::client::crypto::EncryptedMessage;
use serde::{Deserialize, Serialize};

// BODY
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Message {
    pub text: String,
    pub reply: Option<i64>,
}

impl Message {
    pub fn new(text: String) -> Self {
        Message { text, reply: None }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MessageInfo {
    pub body: Message,
    pub sender: String,
    pub id: i64,
}

impl Client {
    pub async fn send_message(
        &mut self,
        nickname_from: String,
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

        // self.cache.put(&nickname_from, &encryptred_data).await?; TODO
        self.raw_client
            .send_message(
                nickname_from,
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
        nickname: String,
        limit: i64,
    ) -> Result<Vec<MessageInfo>, Error> {
        self.raw_get_last_message(vec![nickname], limit).await
    }

    pub(crate) fn decrypt_message(
        storage_crypto: &StorageCrypto,
        message: crate_proto::Message,
        nickname_from: String,
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
    ) -> Result<Vec<MessageInfo>, Error> {
        let messages = self
            .raw_client
            .get_latest_messages(nicknames, limit)
            .await?;

        Ok(messages
            .messages
            .into_iter()
            .map(|x| {
                let nickname = if x.sender_nickname == *self.get_nickname() {
                    x.recipient_nickname.clone()
                } else {
                    x.sender_nickname.clone()
                };

                (
                    Client::decrypt_message(
                        &self.config.storage_crypto.read().unwrap(),
                        x.body.clone().unwrap(),
                        nickname,
                    )
                    .unwrap(),
                    x,
                )
            })
            .map(|x| MessageInfo {
                body: x.0,
                sender: x.1.sender_nickname,
                id: x.1.id,
            })
            .collect::<Vec<MessageInfo>>())
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
            .collect();

        // TODO
        Ok(self
            .raw_get_last_message(nicknames, 1)
            .await?
            .into_iter()
            .map(|x| x.body)
            .collect())
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
                    Message::new(TEXT_MESSAGE.to_string()),
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
                Message::new(TEST_MESSAGE.to_string()),
            )
            .await
            .unwrap();

        let new_event = recv.recv().await.unwrap();
        println!("new event: {:?}", new_event);

        match new_event.event {
            notification::Event::NewMessage(message) => assert_eq!(message.body.text, TEST_MESSAGE),
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
            .send_message(client_from.get_nickname(), Message::new("ss".to_string()))
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
                .send_message(client_from.get_nickname(), Message::new(text))
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
            .send_message(client_from.get_nickname(), Message::new("ss".to_string()))
            .await
            .unwrap();
        client_from
            .send_message(client_to.get_nickname(), Message::new("tttt".to_string()))
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
            .send_message(client_to.get_nickname(), Message::new("test".to_string()))
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
            let new_message = Message::new("manyyy".to_string());
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
        assert_eq!(
            messages
                .into_iter()
                .map(|x| x.body)
                .collect::<Vec<Message>>(),
            sent_messages
        );
    }

    #[test(tokio::test)]
    async fn send_message_with_reply() {
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
                Message::new("test message".to_string()),
            )
            .await
            .unwrap();
        let id = client_to
            .get_messages_for_user(client_from.get_nickname(), 1)
            .await
            .unwrap()[0]
            .id;

        client_to
            .send_message(
                client_from.get_nickname(),
                Message {
                    text: "test message with reply".to_string(),
                    reply: Some(id),
                },
            )
            .await
            .unwrap();

        let messages = client_from
            .get_messages_for_user(client_to.get_nickname(), 3)
            .await
            .unwrap();
        assert_eq!(messages[0].body.reply, Some(id));
    }
}
