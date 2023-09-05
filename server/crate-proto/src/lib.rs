#[allow(non_snake_case)]
pub mod security_chat {
    tonic::include_proto!("security_chat");
}

pub use security_chat::security_chat_server::{SecurityChat, SecurityChatServer};
