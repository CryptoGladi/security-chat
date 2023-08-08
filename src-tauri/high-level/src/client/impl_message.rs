use super::{storage_crypto::StorageCrypto, *};
use lower_level::client::crypto::EncryptedMessage;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub text: String,
}

impl Client {
    pub async fn send_message(
        &mut self,
        nickname_from: Nickname,
        message: Message,
    ) -> Result<(), Error> {
        info!("run send_message");
        let aes = self.config.storage_crypto.get(&nickname_from)?;
        let bincode = bincode::serialize(&message)?;
        let encryptred_data = aes.encrypt(&bincode[..])?;

        self.raw_client
            .send_message(
                nickname_from.0,
                crate_proto::Message {
                    body: encryptred_data.data,
                    nonce: encryptred_data.nonce.to_vec(),
                },
            )
            .await?;
        Ok(())
    }

    pub(crate) fn decrypt_message(
        storage_crypto: &StorageCrypto,
        message: crate_proto::Message,
        nickname_from: Nickname,
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
}
