//! Module for encryption key management.
//!
//! # Includes:
//!
//! 1. Generation and exchange of keys between users
//! 2. Deletion of keys
//! and so on...
//!
//! # Algorithm
//!
//! - [ECDH](https://en.wikipedia.org/wiki/Elliptic-curve_Diffie%E2%80%93Hellman)
//! - [AES](https://en.wikipedia.org/wiki/Advanced_Encryption_Standard)
//! - [Diffie–Hellman key exchange](https://en.wikipedia.org/wiki/Diffie%E2%80%93Hellman_key_exchange)
//!
//! Short: the algorithm repeats itself ECDH, after creating AES.
//! [ECDH is very cool](https://crypto.stackexchange.com/questions/61248/aes-and-ecdh-key)
//!
//! 1. User "A" wants to transfer (in fact, no key transfer takes place.
//! Only a common passphrase is exchanged) the encryption key to user "B".
//! 2. User "A" executes the function [`Client::send_aes_key`].
//! It involves generating a passphrase and sending it to the **server**
//! 3. User "B" executes the function [`Client::get_aes_keys`].
//! It is needed to get the passphrase of user "A".
//! 4. User "B" creates his part of the passphrase for user "A" through the [`Client::set_aes_key`] function
//! 5. Now User "A" and user "B" have two identical passphrases!
//! This can be used to create a symmetric encryption key (e.g. AES).
//!
//! This algorithm allows for privacy (like [Telegram's secure chats](https://core.telegram.org/blackberry/secretchats)),
//! but only requires one user to be online

pub mod aes;
pub mod ecdh;
pub mod error;

use super::{
    debug, impl_crypto, AesKeyInfo, Check, Client, DeleteAesKeyRequest, EphemeralSecret, Error,
    GetAesKeyRequest, SendAesKeyRequest, SetUserFromAesKeyRequest, ToEncodedPoint,
};

impl Client {
    /// Delete key for **server**
    pub async fn delete_key(&mut self, id: i64) -> Result<(), Error> {
        debug!("run `delete_key` by id: {}", id);

        let request = tonic::Request::new(DeleteAesKeyRequest {
            nickname: Some(Check {
                nickname: self.data_for_autification.nickname.clone(),
                authkey: self.data_for_autification.tokens.refresh_token.clone(),
            }),
            id,
        });

        self.grpc.delete_aes_key(request).await?;

        Ok(())
    }

    /// Generate and send key to **server**
    ///
    /// # Panics
    ///
    /// Can't send to yourself
    pub async fn send_aes_key(&mut self, nickname_form: &str) -> Result<EphemeralSecret, Error> {
        debug!("run `send_aes_key` by nickname_from: {}", nickname_form);

        assert_ne!(
            nickname_form, self.data_for_autification.nickname,
            "ТЫ СОВСЕМ ЕБНУТЫЙ!?"
        );

        let (secret, public_key) = impl_crypto::ecdh::get_public_info()?;

        let request = tonic::Request::new(SendAesKeyRequest {
            nickname_to: Some(Check {
                nickname: self.data_for_autification.nickname.clone(),
                authkey: self.data_for_autification.tokens.refresh_token.clone(),
            }),
            nickname_from: nickname_form.to_string(),
            public_key: public_key.to_encoded_point(true).as_bytes().to_vec(),
        });

        self.grpc.send_aes_key(request).await?;

        Ok(secret)
    }

    /// Get all keys from **server**
    pub async fn get_aes_keys(&mut self) -> Result<Vec<AesKeyInfo>, Error> {
        debug!("run `get_aes_key`");

        let request = tonic::Request::new(GetAesKeyRequest {
            nickname: Some(Check {
                nickname: self.data_for_autification.nickname.clone(),
                authkey: self.data_for_autification.tokens.refresh_token.clone(),
            }),
        });

        let info = self.grpc.get_aes_key(request).await?;
        Ok(info.get_ref().info.clone())
    }

    /// Send my **second part** of the key
    pub async fn set_aes_key(&mut self, key_id: i64) -> Result<EphemeralSecret, Error> {
        debug!("run `set_aes_key` by key_id: {}", key_id);

        let (secret, public_key) = impl_crypto::ecdh::get_public_info()?;
        let request = tonic::Request::new(SetUserFromAesKeyRequest {
            nickname: Some(Check {
                nickname: self.data_for_autification.nickname.clone(),
                authkey: self.data_for_autification.tokens.refresh_token.clone(),
            }),
            id: key_id,
            public_key: public_key.to_encoded_point(true).as_bytes().to_vec(),
        });

        self.grpc.set_user_from_aes_key(request).await?;
        Ok(secret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::impl_crypto::ecdh::PublicKey;
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
        log::info!("client_to data: {:?}", client_to.data_for_autification);

        let secret_to = client_to
            .send_aes_key(&client_from.data_for_autification.nickname)
            .await
            .unwrap();
        let keys = client_from.get_aes_keys().await.unwrap();

        log::info!("keys: {keys:?}");

        let secter_from = client_from.set_aes_key(keys[0].id).await.unwrap();
        let new_keys = client_from.get_aes_keys().await.unwrap();
        log::info!("new_keys: {new_keys:?}");

        let public_from =
            PublicKey::from_sec1_bytes(&new_keys[0].nickname_from_public_key.clone().unwrap()[..])
                .unwrap();
        let public_to =
            PublicKey::from_sec1_bytes(&new_keys[0].nickname_to_public_key.clone()[..]).unwrap();
        let sect = impl_crypto::ecdh::get_shared_secret(&secret_to, &public_from);
        let sss = impl_crypto::ecdh::get_shared_secret(&secter_from, &public_to);

        assert_eq!(sect.raw_secret_bytes(), sss.raw_secret_bytes());
    }
}
