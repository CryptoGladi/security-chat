use super::*;

#[derive(Debug)]
pub struct AddCrypto;

impl Command<ClientError> for AddCrypto {
    fn get_id(&self) -> &'static str {
        "add_crypto"
    }

    fn run(&mut self, client: &Client, command: &str) -> Result<(), ClientError> {
        Ok(())
    }
}
