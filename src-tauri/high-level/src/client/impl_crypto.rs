pub mod aes_key_for_accept;

use super::*;
pub use aes_key_for_accept::AesKeyForAccept;

impl Client {
    pub async fn send_crypto(&mut self, nickname_from: Nickname) -> Result<(), Error> {
        info!("run send_crypto");
        if self.raw_client.data.nickname == *nickname_from {
            return Err(Error::NicknameSame(nickname_from));
        }
        if self.config.order_adding_crypto.contains_key(&nickname_from) {
            return Err(Error::NicknameSame(nickname_from));
        }

        let secret = self.raw_client.send_aes_key(&nickname_from).await?;
        let secret_def = unsafe { EphemeralSecretDef::from(secret) };

        self.config
            .order_adding_crypto
            .insert(nickname_from, secret_def);
        self.save()?;
        Ok(())
    }

    pub async fn get_cryptos_for_accept(&mut self) -> Result<Vec<AesKeyForAccept>, Error> {
        info!("run get_cryptos_for_accept");
        let aes_info = self.raw_client.get_aes_keys().await?;
        Ok(aes_info.into_iter().map(AesKeyForAccept).collect())
    }

    pub async fn accept_all_cryptos(&mut self) -> Result<(), Error> {
        info!("run accept_all_cryptos");
        let mut aes_info = self.get_cryptos_for_accept().await?;

        for i in aes_info.iter_mut() {
            i.accept(self).await?;
        }

        Ok(())
    }

    pub async fn update_cryptos(&mut self) -> Result<(), Error> {
        info!("run update_cryptos");
        let keys_info = self.raw_client.get_aes_keys().await?;

        for i in keys_info {
            trace!("iter key_info: {:?}", i);
            let nickname_from = Nickname(i.nickname_from.clone());
            let (Some(nickname_from_public_key), Some(ephemeral_secret_def)) = (
                &i.nickname_from_public_key,
                self.config.order_adding_crypto.get(&nickname_from),
            ) else {
                if cfg!(debug_assertions) {
                    //panic!(
                    //    "break update_cryptos! iter: {:?}, order_adding_crypto: {:?}, nickname_from: {}",
                    //    i, self.config.order_adding_crypto, nickname_from
                    //);
                } else {
                    error!(
                        "break update_cryptos! iter: {:?}, order_adding_crypto: {:?}, nickname_from: {}",
                        i, self.config.order_adding_crypto, nickname_from
                    );
                }
                
                continue;
            };

            let secret = unsafe { ephemeral_secret_def.clone().get() };

            let shared_secret = get_shared_secret(
                &secret,
                &PublicKey::from_sec1_bytes(&nickname_from_public_key[..]).unwrap(),
            );
            let aes = Aes::with_shared_key(shared_secret);

            self.config
                .storage_crypto
                .write()
                .unwrap()
                .add(nickname_from.clone(), aes)?;

            self.config
                .order_adding_crypto
                .remove(&nickname_from)
                .unwrap();
            self.raw_client.delete_key(i.id).await?;
        }

        self.save()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::impl_message::Message;
    use crate::test_utils::get_client;
    use test_log::test;

    #[test(tokio::test)]
    async fn add_crypto_via_subscribe() {
        let (_paths, _, mut client_to) = get_client().await;
        let (_paths, _, mut client_from) = get_client().await;

        let recv_from = client_from.subscribe().await.unwrap();
        let recv_to = client_to.subscribe().await.unwrap();

        client_to
            .send_crypto(client_from.get_nickname())
            .await
            .unwrap();

        let notification = recv_from.recv().await.unwrap();

        let notification::Event::NewSentAcceptAesKey(mut key_for_accept) = notification.event
        else {
            panic!();
        };
        key_for_accept.accept(&mut client_from).await.unwrap();

        let notification = recv_to.recv().await.unwrap();

        if let notification::Event::NewAcceptAesKey(key) = notification.event {
            assert_eq!(key_for_accept.0.id, key.id);
            assert_eq!(key_for_accept.0.nickname_from, key.nickname_from);
            assert_eq!(key_for_accept.0.nickname_to, key.nickname_to);
            assert_eq!(
                key_for_accept.0.nickname_to_public_key,
                key.nickname_to_public_key
            );
            assert!(key.nickname_from_public_key.is_some());
            assert!(key_for_accept.0.nickname_from_public_key.is_none());

            client_to.update_cryptos().await.unwrap();
        } else {
            panic!();
        }

        const TEXT_MESSAGE: &str = "MESSAGE";

        client_to
            .send_message(
                client_from.get_nickname(),
                Message {
                    text: TEXT_MESSAGE.to_string(),
                },
            )
            .await
            .unwrap();

        let notification = recv_from.recv().await.unwrap();
        if let notification::Event::NewMessage(message) = notification.event {
            assert_eq!(message.body.text, TEXT_MESSAGE);
        } else {
            panic!()
        }
    }

    #[test(tokio::test)]
    async fn shared_keys() {
        let (_paths, _, mut client_to) = get_client().await;
        let (_paths, _, mut client_from) = get_client().await;

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
                .read()
                .unwrap()
                .get(&client_from.get_nickname())
                .unwrap(),
            client_from
                .config
                .storage_crypto
                .read()
                .unwrap()
                .get(&client_to.get_nickname())
                .unwrap()
        );
    }

    #[test(tokio::test)]
    async fn clear_keys_for_accept_after_adding() {
        let (_paths, _, mut client_to) = get_client().await;
        let (_paths, _, mut client_from) = get_client().await;

        client_to
            .send_crypto(client_from.get_nickname())
            .await
            .unwrap();

        client_from.accept_all_cryptos().await.unwrap();
        client_to.update_cryptos().await.unwrap();

        assert_eq!(client_from.get_cryptos_for_accept().await.unwrap().len(), 0);
        assert_eq!(client_to.get_cryptos_for_accept().await.unwrap().len(), 0);
    }
}
