//! Crate for [`tonic::include_proto`]

pub use security_chat::notification::Notice;
pub use security_chat::*;
pub use security_chat_client::SecurityChatClient;

#[allow(non_snake_case)]
pub mod security_chat {
    tonic::include_proto!("security_chat");
}
