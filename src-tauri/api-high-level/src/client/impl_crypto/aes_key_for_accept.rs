use super::{debug, error, get_shared_secret, Aes, Client, PublicKey};
use crate::client::error::Error;
use api_lower_level::client::impl_crypto::error::Error as CryptoError;
use crate_proto::AesKeyInfo;

#[derive(Debug, Clone, PartialEq)]
pub struct AesKeyForAccept(pub AesKeyInfo);

impl AesKeyForAccept {
    fn key_is_already_accepted(&mut self) -> bool {
        self.0.nickname_from_public_key.is_some()
    }

    fn check_key_is_already_accepted(&mut self) -> Result<(), Error> {
        if self.key_is_already_accepted() {
            error!(
                "key is already is accepted from nickname: {}",
                self.0.nickname_from
            );

            return Err(Error::Crypto(CryptoError::KeyAlreadyAccepted(
                self.0.nickname_from.clone(),
            )));
        }

        Ok(())
    }

    pub async fn accept(&mut self, client: &mut Client) -> Result<(), Error> {
        debug!("run accept with id: {}", self.0.id);
        self.check_key_is_already_accepted()?;

        let secret = client.lower_level_client.set_aes_key(self.0.id).await?;
        let public_key =
            PublicKey::from_sec1_bytes(&self.0.nickname_to_public_key.clone()[..]).unwrap();
        let shared = get_shared_secret(&secret, &public_key);
        let aes = Aes::with_shared_key(&shared);

        client
            .config
            .storage_crypto
            .write()
            .unwrap()
            .add(self.0.nickname_to.clone(), aes)?;

        Ok(())
    }

    pub async fn delete(&mut self, client: &mut Client) -> Result<(), Error> {
        debug!("run delete with id: {}", self.0.id);

        self.check_key_is_already_accepted()?;
        client.lower_level_client.delete_key(self.0.id).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{AesKeyForAccept, CryptoError};
    use crate::client::error::Error;
    use crate::client::Client;
    use crate::test_utils::get_client;
    use test_log::test;

    async fn iter_function(
        client_to: &mut Client,
        client_from: &mut Client,
    ) -> Vec<AesKeyForAccept> {
        client_to
            .send_crypto(client_from.get_nickname())
            .await
            .unwrap();

        client_from.get_cryptos_for_accept().await.unwrap()
    }

    #[test(tokio::test)]
    async fn delete() {
        let (_paths, _, mut client_to) = get_client().await;
        let (_paths, _, mut client_from) = get_client().await;

        for mut key in iter_function(&mut client_to, &mut client_from).await {
            key.delete(&mut client_from).await.unwrap();
        }

        assert!(client_from
            .get_cryptos_for_accept()
            .await
            .unwrap()
            .is_empty());
    }

    #[test(tokio::test)]
    async fn delete_with_panic_key_is_already_accepted() {
        let (_paths, _, mut client_to) = get_client().await;
        let (_paths, _, mut client_from) = get_client().await;

        for mut key in iter_function(&mut client_to, &mut client_from).await {
            key.accept(&mut client_from).await.unwrap();
        }

        let mut iter = client_from.get_cryptos_for_accept().await.unwrap();
        assert!(!iter.is_empty());

        for key in &mut iter {
            let _nickname = client_to.get_nickname();
            assert!(matches!(
                key.delete(&mut client_from).await,
                Err(Error::Crypto(CryptoError::KeyAlreadyAccepted(_nickname)))
            ));
        }
    }

    #[test(tokio::test)]
    async fn accept() {
        let (_paths, _, mut client_to) = get_client().await;
        let (_paths, _, mut client_from) = get_client().await;

        client_to
            .send_crypto(client_from.get_nickname())
            .await
            .unwrap();
        for x in &mut client_from.get_cryptos_for_accept().await.unwrap() {
            x.accept(&mut client_from).await.unwrap();
        }

        assert_eq!(
            client_from.get_all_users().unwrap(),
            vec![client_to.get_nickname()]
        );
    }
}
