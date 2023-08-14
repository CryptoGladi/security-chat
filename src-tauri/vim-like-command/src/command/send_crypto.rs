use high_level::client::storage_crypto::Nickname;
use super::*;

#[derive(Debug)]
pub struct SendCrypto;

#[async_trait]
impl Command<ClientError> for SendCrypto {
    fn get_id(&self) -> &'static str {
        "send_crypto"
    }

    async fn run(&self, client: &mut Client, args: &[&str]) -> Result<(), ClientError> {
        let nickname = args[1];
        client.send_crypto(Nickname(nickname.to_string())).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::*;
    use test_log::test;
    use super::*;

    #[test(tokio::test)]
    async fn run() {
        let (_temp_dir, _, mut client_to) = get_client().await;
        let (_temp_dir, _, client_from) = get_client().await;

        let command = format!("send_crypto {}", client_from.get_nickname());

        let send_crypto = SendCrypto;
        send_crypto.run(&mut client_to, &command.split_whitespace().collect::<Vec<&str>>()).await.unwrap();
    }
}