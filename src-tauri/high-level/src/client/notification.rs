use super::impl_crypto::AesKeyForAccept;
use super::{impl_message::Message, storage_crypto::StorageCrypto, *};
use crate_proto::Notice::*;
use crate_proto::Notification as RawNotification;

#[derive(Debug, Clone)]
pub enum Event {
    NewMessage(Message),
    NewAcceptAesKey(AesKeyForAccept),
}

#[derive(Debug, Clone)]
pub struct Notification {
    pub by_nickname: Nickname,
    pub event: Event,
}

impl Client {
    pub(crate) fn nofity(
        storage_crypto: &StorageCrypto,
        raw: RawNotification,
    ) -> Result<Notification, Error> {
        let event = match raw.notice.unwrap() {
            NewMessage(message) => Event::NewMessage(Client::decrypt_message(
                storage_crypto,
                message,
                Nickname(raw.by_nickname.clone()),
            )?),
            NewSendAesKey(info) => Event::NewAcceptAesKey(AesKeyForAccept(info)),
        };

        Ok(Notification {
            by_nickname: Nickname(raw.by_nickname),
            event,
        })
    }
}
