pub mod error;
pub(crate) mod crypto;

pub struct Client {
    pub crypto: crypto::Crypto,
    pub username: String
}

impl Client {
    async fn registration(username: String) {

        todo!()
    }
}
