use super::*;

impl Client {
    pub async fn delete_key(&mut self, id: i64) -> Result<(), Error> {
        let request = tonic::Request::new(DeleteAesKeyRequest {
            nickname: Some(Check {
                nickname: self.data_for_autification.nickname.clone(),
                authkey: self.data_for_autification.auth_key.clone(),
            }),
            id,
        });

        self.grpc.delete_aes_key(request).await?;

        Ok(())
    }

    pub async fn send_aes_key(&mut self, nickname_form: &str) -> Result<EphemeralSecret, Error> {
        assert_ne!(
            nickname_form, self.data_for_autification.nickname,
            "ТЫ СОВСЕМ ЕБНУТЫЙ!?"
        );
        let (secret, public_key) = crypto::ecdh::get_public_info()?;

        let request = tonic::Request::new(SendAesKeyRequest {
            nickname_to: Some(Check {
                nickname: self.data_for_autification.nickname.clone(),
                authkey: self.data_for_autification.auth_key.clone(),
            }),
            nickname_from: nickname_form.to_string(),
            public_key: public_key.to_encoded_point(true).as_bytes().to_vec(),
        });

        self.grpc.send_aes_key(request).await?;

        Ok(secret)
    }

    pub async fn get_aes_keys(&mut self) -> Result<Vec<AesKeyInfo>, Error> {
        let request = tonic::Request::new(GetAesKeyRequest {
            nickname: Some(Check {
                nickname: self.data_for_autification.nickname.clone(),
                authkey: self.data_for_autification.auth_key.clone(),
            }),
        });

        let info = self.grpc.get_aes_key(request).await?;
        Ok(info.get_ref().info.clone())
    }

    pub async fn set_aes_key(&mut self, key_info: &AesKeyInfo) -> Result<EphemeralSecret, Error> {
        let (secret, public_key) = crypto::ecdh::get_public_info()?;
        let request = tonic::Request::new(SetUserFromAesKeyRequest {
            nickname: Some(Check {
                nickname: self.data_for_autification.nickname.clone(),
                authkey: self.data_for_autification.auth_key.clone(),
            }),
            id: key_info.id,
            public_key: public_key.to_encoded_point(true).as_bytes().to_vec(),
        });

        self.grpc.set_user_from_aes_key(request).await.unwrap();
        Ok(secret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::crypto::ecdh::PublicKey;
    use fcore::test_utils::*;

    #[tokio::test]
    async fn send_and_get_aes_key() {
        let mut client_to =
            Client::registration(&get_rand_string(20), ADDRESS_SERVER.parse().unwrap())
                .await
                .unwrap();
        let mut client_from =
            Client::registration(&get_rand_string(20), ADDRESS_SERVER.parse().unwrap())
                .await
                .unwrap();
        println!("client_to data: {:?}", client_to.data_for_autification);

        let secret_to = client_to
            .send_aes_key(&client_from.data_for_autification.nickname)
            .await
            .unwrap();
        let keys = client_from.get_aes_keys().await.unwrap();

        println!("keys: {:?}", keys);

        let secter_from = client_from.set_aes_key(&keys[0]).await.unwrap();
        let new_keys = client_from.get_aes_keys().await.unwrap();
        println!("new_keys: {:?}", new_keys);

        let public_from =
            PublicKey::from_sec1_bytes(&new_keys[0].nickname_from_public_key.clone().unwrap()[..])
                .unwrap();
        let public_to =
            PublicKey::from_sec1_bytes(&new_keys[0].nickname_to_public_key.clone()[..]).unwrap();
        let sect = crypto::ecdh::get_shared_secret(&secret_to, &public_from);
        let sss = crypto::ecdh::get_shared_secret(&secter_from, &public_to);

        assert_eq!(sect.0.raw_secret_bytes(), sss.0.raw_secret_bytes());
    }
}
