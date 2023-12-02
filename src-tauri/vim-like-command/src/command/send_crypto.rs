//! Command for send crypto
//!
//! # Example
//!
//! `send_crypto test_nickname`

use super::{async_trait, Client, Command, Debug, Error};

#[derive(Debug)]
pub struct SendCrypto;

#[async_trait]
impl Command<Error> for SendCrypto {
    fn get_id(&self) -> &'static str {
        "send_crypto"
    }

    async fn run(&self, client: &mut Client, args: &[&str]) -> Result<(), Error> {
        let Some(nickname) = args.get(1) else {
            return Err(Error::Other("nickname is invalid"));
        };

        client.send_crypto((*nickname).to_string()).await?;
        client.save_config().unwrap();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use api_high_level::client::impl_message::Message;
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
        let test_message = "testing message";

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
        client_to.refresh_cryptos().await.unwrap();

        client_to
            .send_message(
                client_from.get_nickname(),
                Message::new(test_message.to_string()),
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
            test_message
        );
    }
}
