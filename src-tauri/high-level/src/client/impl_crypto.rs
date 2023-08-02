use super::*;
use lower_level::client::security_chat::AesKeyInfo;

#[derive(Debug)]
pub struct AesKeyForAccept(pub AesKeyInfo);

impl AesKeyForAccept {
    #[tracing::instrument(skip(self))]
    pub async fn accept(&mut self, client: &mut Client) -> Result<(), Error> {
        tracing::info!("run");
        let secret = client.raw_client.set_aes_key(&self.0).await?;
        let public_key =
            PublicKey::from_sec1_bytes(&self.0.nickname_to_public_key.clone()[..]).unwrap();
        let shared = get_shared_secret(&secret, &public_key);
        let aes = Aes::with_shared_key(shared);

        client
            .config
            .storage_crypto
            .insert(Nickname(self.0.nickname_to.clone()), aes);
        Ok(())
    }
}

impl Client {
    #[tracing::instrument(skip(self))]
    pub async fn send_crypto(&mut self, nickname_from: Nickname) -> Result<(), Error> {
        tracing::info!("run");
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
        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_cryptos_for_accept(&mut self) -> Result<Vec<AesKeyForAccept>, Error> {
        tracing::info!("run");
        let aes_info = self.raw_client.get_aes_keys().await?;
        Ok(aes_info.into_iter().map(AesKeyForAccept).collect())
    }

    #[tracing::instrument(skip(self))]
    pub async fn update_cryptos(&mut self) -> Result<(), Error> {
        tracing::info!("run");
        let keys_info = self.raw_client.get_aes_keys().await?;

        for i in keys_info
            .iter()
            .filter(|x| x.nickname_from_public_key.is_some())
        {
            let nickname_from = Nickname(i.nickname_from.clone());
            let secret = unsafe {
                self.config
                    .order_adding_crypto
                    .get(&nickname_from)
                    .unwrap()
                    .clone()
                    .get()
            };

            let shared_secret = get_shared_secret(
                &secret,
                &PublicKey::from_sec1_bytes(&i.nickname_from_public_key.clone().unwrap()[..])
                    .unwrap(),
            );
            let aes = Aes::with_shared_key(shared_secret);

            self.config
                .storage_crypto
                .insert(nickname_from.clone(), aes);

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
