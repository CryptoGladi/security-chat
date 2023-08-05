use crate_proto::Message;
use super::*;

impl Client {
    pub async fn send_message(&mut self, nickname_from: Nickname, message: Message) -> Result<(), Error> {
        let aes = self.config.storage_crypto.get(&nickname_from)?;
        let encryptred_text = aes.encrypt(&message.text[..])?;

        self.raw_client.send_message(nickname_from.0, Message { text: encryptred_text }).await?;
        Ok(())
    }
}
