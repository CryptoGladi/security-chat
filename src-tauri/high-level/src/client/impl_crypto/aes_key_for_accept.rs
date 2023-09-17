use super::*;
use crate_proto::AesKeyInfo;
use lower_level::client::crypto::CryptoError;

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
        info!("run accept with id: {}", self.0.id);
        self.check_key_is_already_accepted()?;

        let secret = client.raw_client.set_aes_key(&self.0).await?;
        let public_key =
            PublicKey::from_sec1_bytes(&self.0.nickname_to_public_key.clone()[..]).unwrap();
        let shared = get_shared_secret(&secret, &public_key);
        let aes = Aes::with_shared_key(shared);

        client
            .config
            .storage_crypto
            .write()
            .unwrap()
            .add(Nickname(self.0.nickname_to.clone()), aes)?;
        client.save()?;

        Ok(())
    }

    pub async fn delete(&mut self, client: &mut Client) -> Result<(), Error> {
        debug!("run delete with id: {}", self.0.id);
        self.check_key_is_already_accepted()?;

        client.raw_client.delete_key(self.0.id).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::client::error::Error;
    use crate::test_utils::get_client;
    use lower_level::client::crypto::CryptoError;
    use test_log::test;

    #[test(tokio::test)]
    async fn delete() {
        let (_paths, _, mut client_to) = get_client().await;
        let (_paths, _, mut client_from) = get_client().await;

        client_to
            .send_crypto(client_from.get_nickname())
            .await
            .unwrap();
        for x in client_from
            .get_cryptos_for_accept()
            .await
            .unwrap()
            .iter_mut()
        {
            x.delete(&mut client_from).await.unwrap();
        }

        assert!(client_from
            .get_cryptos_for_accept()
            .await
            .unwrap()
            .is_empty());
    }

    #[test[tokio::test]]
    async fn delete_with_panic_key_is_already_accepted() {
        let (_paths, _, mut client_to) = get_client().await;
        let (_paths, _, mut client_from) = get_client().await;

        client_to
            .send_crypto(client_from.get_nickname())
            .await
            .unwrap();

        let mut iter = client_from.get_cryptos_for_accept().await.unwrap();
        for key in iter.iter_mut() {
            key.accept(&mut client_from).await.unwrap();
        }

        let mut iter = client_from.get_cryptos_for_accept().await.unwrap();
        assert!(!iter.is_empty());

        for key in iter.iter_mut() {
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
        for x in client_from
            .get_cryptos_for_accept()
            .await
            .unwrap()
            .iter_mut()
        {
            x.accept(&mut client_from).await.unwrap();
        }

        assert_eq!(
            client_from.get_all_users().unwrap(),
            vec![client_to.get_nickname()]
        );
    }
}
