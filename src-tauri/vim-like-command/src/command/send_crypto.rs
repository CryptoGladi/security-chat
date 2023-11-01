//! Command for send crypto
//!
//! # Example
//!
//! `send_crypto test_nickname`

use super::*;
use high_level::client::storage_crypto::Nickname;

#[derive(Debug)]
pub struct SendCrypto;

#[async_trait]
impl Command<CommandError> for SendCrypto {
    fn get_id(&self) -> &'static str {
        "send_crypto"
    }

    async fn run(&self, client: &mut Client, args: &[&str]) -> Result<(), CommandError> {
        let Some(nickname) = args.get(1) else {
            return Err(CommandError::Other("nickname is invalid"));
        };

        client
            .send_crypto(Nickname((*nickname).to_string()))
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use high_level::client::impl_message::Message;
    use test_log::test;

    #[test(tokio::test)]
    #[should_panic(expected = r#"Other("nickname is invalid")"#)]
    async fn nickname_is_invalid() {
        let (_temp_dir, _, mut client_to) = get_client().await;
        let command = "send_crypto ";

        let send_crypto = SendCrypto;

        send_crypto
            .run(
                &mut client_to,
                &command.split_whitespace().collect::<Vec<&str>>(),
            )
            .await
            .unwrap();
    }

    #[test(tokio::test)]
    async fn raw_run() {
        let (_temp_dir, _, mut client_to) = get_client().await;
        let (_temp_dir, _, client_from) = get_client().await;

        let command = format!("send_crypto {}", client_from.get_nickname());

        let send_crypto = SendCrypto;
        send_crypto
            .run(
                &mut client_to,
                &command.split_whitespace().collect::<Vec<&str>>(),
            )
            .await
            .unwrap();
    }

    #[test(tokio::test)]
    async fn run_via_runner() {
        let (_temp_dir, _, mut client_to) = get_client().await;
        let (_temp_dir, _, mut client_from) = get_client().await;
        const TEST_MESSAGE: &str = "testing message";

        let mut runner = RunnerBuilder::new()
            .commands(vec![&SendCrypto])
            .build()
            .unwrap();

        runner
            .run(
                &mut client_to,
                &format!("send_crypto {}", client_from.get_nickname()),
            )
            .await
            .unwrap();
        client_from.accept_all_cryptos().await.unwrap();
        client_to.update_cryptos().await.unwrap();

        client_to
            .send_message(
                client_from.get_nickname(),
                Message::new(TEST_MESSAGE.to_string()),
            )
            .await
            .unwrap();
        assert_eq!(
            client_from
                .get_messages_for_user(client_to.get_nickname(), 1)
                .await
                .unwrap()[0]
                .body
                .text,
            TEST_MESSAGE
        );
    }
}
