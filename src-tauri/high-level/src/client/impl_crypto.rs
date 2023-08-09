use super::*;
use crate_proto::AesKeyInfo;

#[derive(Debug, Clone, PartialEq)]
pub struct AesKeyForAccept(pub AesKeyInfo);

impl AesKeyForAccept {
    pub async fn accept(&mut self, client: &mut Client) -> Result<(), Error> {
        info!("run accept for AesKeyForAccept");
        debug_assert!(self.0.nickname_from_public_key.is_none());

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
}

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
            let nickname_from = Nickname(i.nickname_from.clone());
            let (Some(nickname_from_public_key), Some(ephemeral_secret_def)) = (i.nickname_from_public_key, self.config.order_adding_crypto.get(&nickname_from)) else {
                break;
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
