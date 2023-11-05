//! Module for processing notifications

use super::impl_crypto::AesKeyForAccept;
use super::impl_message::MessageInfo;
use super::{storage_crypto::StorageCrypto, *};
use crate_proto::AesKeyInfo;
use crate_proto::Notice::{NewAcceptAesKey, NewMessage, NewSendAesKey};
use crate_proto::Notification as RawNotification;

#[derive(Debug, Clone)]
pub enum Event {
    NewMessage(MessageInfo),
    NewSentAcceptAesKey(AesKeyForAccept),
    NewAcceptAesKey(AesKeyInfo),
}

#[derive(Debug, Clone)]
pub struct Notification {
    pub by_nickname: String,
    pub event: Event,
}

impl Client {
    pub(crate) fn nofity(
        storage_crypto: &StorageCrypto,
        raw: RawNotification,
    ) -> Result<Notification, Error> {
        let event = match raw.notice.unwrap() {
            NewMessage(message) => Event::NewMessage(MessageInfo {
                body: Client::decrypt_message(
                    storage_crypto,
                    message.message.unwrap(),
                    raw.by_nickname.clone(),
                )?,
                sender: raw.by_nickname.clone(),
                id: message.id,
            }),
            NewSendAesKey(info) => Event::NewSentAcceptAesKey(AesKeyForAccept(info)),
            NewAcceptAesKey(info) => Event::NewAcceptAesKey(info),
        };

        Ok(Notification {
            by_nickname: raw.by_nickname,
            event,
        })
    }
}
