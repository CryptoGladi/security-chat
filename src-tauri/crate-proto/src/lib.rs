//! Crate for [`tonic::include_proto`]
//!
#![forbid(unsafe_code)]

pub use security_chat::notification::Notice;
pub use security_chat::*;
pub use security_chat_client::SecurityChatClient;

#[allow(clippy::pedantic)]
#[allow(clippy::min_ident_chars)]
pub mod security_chat {
    tonic::include_proto!("security_chat");
}
