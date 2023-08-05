use super::{*, impl_message::Message};
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
    pub(crate) fn nofity(&mut self, raw: RawNotification) -> Result<Notification, Error> {        
        let event = match raw.notice.unwrap() {
            crate_proto::Notice::NewMessage(s) => {
                Event::NewMessage(self.decrypt_message(s, Nickname(raw.by_nickname.clone()))?)
            }
        };
        
        Ok(Notification { by_nickname: Nickname(raw.by_nickname), event })
    }
}