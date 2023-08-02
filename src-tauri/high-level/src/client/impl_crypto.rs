use super::*;

impl Client {
    pub async fn send_crypto(&mut self, nickname_from: Nickname) -> Result<(), Error> {
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

    pub async fn accept_crypto(&mut self) -> Result<(), Error> {
        Ok(())
    }

    pub async fn get_cryptos(&mut self) -> Result<(), Error> {
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
                    .unwrap().clone()
                    .get()
            };

            self.config
                .order_adding_crypto
                .remove(&nickname_from)
                .unwrap();

            let shared_secret = get_shared_secret(&secret, &PublicKey::from_sec1_bytes(&i.nickname_from_public_key.clone().unwrap()[..]).unwrap());
            let aes = Aes::with_shared_key(shared_secret);

            self.config
                .storage_crypto
                .insert(nickname_from, aes);
        }

        self.save()?;
        Ok(())
    }
}