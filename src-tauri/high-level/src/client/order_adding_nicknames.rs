use super::user::Nickname;
use lower_level::client::crypto::ecdh::EphemeralSecret;
use serde::{Serialize, Deserialize};
use lower_level::client::ClientData;

#[derive(Serialize, Deserialize)]
pub struct OrderAddingNicknames {
    order: Vec<(Nickname, EphemeralSecret)>
}