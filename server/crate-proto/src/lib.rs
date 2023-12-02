#[allow(non_snake_case)]
pub mod security_chat {
    tonic::include_proto!("security_chat");
}

pub mod authentication {
    tonic::include_proto!("authentication");
}

pub use authentication::authentication_server::{Authentication, AuthenticationServer};
pub use security_chat::security_chat_server::{SecurityChat, SecurityChatServer};
