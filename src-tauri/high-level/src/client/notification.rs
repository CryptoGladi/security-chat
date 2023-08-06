use super::{*, impl_message::Message, storage_crypto::StorageCrypto};
use crate_proto::Notification as RawNotification;

#[derive(Debug, Clone)]
pub enum Event {
    NewMessage(Message)
}

#[derive(Debug, Clone)]
pub struct Notification {
    pub by_nickname: Nickname,
    pub event: Event
}

impl Client {
    pub(crate) fn nofity(storage_crypto: StorageCrypto, raw: RawNotification) -> Result<Notification, Error> {        
        let event = match raw.notice.unwrap() {
            crate_proto::Notice::NewMessage(message) => {
                Event::NewMessage(Client::decrypt_message(storage_crypto, message, Nickname(raw.by_nickname.clone()))?)
            }
        };
        
        Ok(Notification { by_nickname: Nickname(raw.by_nickname), event })
    }
}