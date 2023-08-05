use super::*;

impl Client {
    pub async fn delete_key(&mut self, id: i64) -> Result<(), Error> {
        let request = tonic::Request::new(DeleteAesKeyRequest {
            nickname: Some(Check {
                nickname: self.data.nickname.clone(),
                authkey: self.data.auth_key.clone(),
            }),
            id,
        });

        self.api.delete_aes_key(request).await?;

        Ok(())
    }

    pub async fn send_aes_key(&mut self, nickname_form: &str) -> Result<EphemeralSecret, Error> {
        assert_ne!(nickname_form, self.data.nickname, "ТЫ СОВСЕМ ЕБНУТЫЙ!?");
        let (secret, public_key) = crypto::ecdh::get_public_info()?;

        let request = tonic::Request::new(SendAesKeyRequest {
            nickname_to: Some(Check {
                nickname: self.data.nickname.clone(),
                authkey: self.data.auth_key.clone(),
            }),
            nickname_from: nickname_form.to_string(),
            public_key: public_key.to_encoded_point(true).as_bytes().to_vec(),
        });

        self.api.send_aes_key(request).await?;

        Ok(secret)
    }

    pub async fn get_aes_keys(&mut self) -> Result<Vec<AesKeyInfo>, Error> {
        let request = tonic::Request::new(GetAesKeyRequest {
            nickname: Some(Check {
                nickname: self.data.nickname.clone(),
                authkey: self.data.auth_key.clone(),
            }),
        });

        let info = self.api.get_aes_key(request).await?;
        Ok(info.get_ref().info.clone())
    }

    pub async fn set_aes_key(&mut self, key_info: &AesKeyInfo) -> Result<EphemeralSecret, Error> {
        let (secret, public_key) = crypto::ecdh::get_public_info()?;
        let request = tonic::Request::new(SetUserFromAesKeyRequest {
            nickname: Some(Check {
                nickname: self.data.nickname.clone(),
                authkey: self.data.auth_key.clone(),
            }),
            id: key_info.id,
            public_key: public_key.to_encoded_point(true).as_bytes().to_vec(),
        });

        self.api.set_user_from_aes_key(request).await.unwrap();
        Ok(secret)
    }
}
