//! Module for messages;

use super::{
    trace, Check, Client, Error, GetLatestMessagesReply, GetLatestMessagesRequest, Message,
    SendMessageRequest, MAX_LEN_MESSAGE, MAX_LIMIT_GET_MESSAGES,
};

impl Client {
    pub async fn send_message(
        &mut self,
        nickname_from: String,
        message: Message,
    ) -> Result<(), Error> {
        trace!(
            "run `send_message` for nickname_from: {} and message {:?}",
            nickname_from,
            message
        );

        if message.body.len() >= MAX_LEN_MESSAGE {
            return Err(Error::TooBigMessage);
        }

        let request = tonic::Request::new(SendMessageRequest {
            nickname: Some(Check {
                nickname: self.data_for_autification.nickname.clone(),
                authkey: self.data_for_autification.auth_key.clone(),
            }),
            nickname_from,
            message: Some(message),
        });

        self.grpc.send_message(request).await?;
        Ok(())
    }

    pub async fn get_latest_messages(
        &mut self,
        nickname_for_get: Vec<String>,
        limit: i64,
    ) -> Result<GetLatestMessagesReply, Error> {
        trace!("run `get_latest_messages` for {:?}", nickname_for_get);

        if limit <= 0 {
            return Err(Error::InvalidArgument("limit <= 0"));
        }

        if limit > MAX_LIMIT_GET_MESSAGES {
            return Err(Error::InvalidArgument("limit > MAX_LIMIT_GET_MESSAGES"));
        }

        let request = tonic::Request::new(GetLatestMessagesRequest {
            nickname: Some(Check {
                nickname: self.data_for_autification.nickname.clone(),
                authkey: self.data_for_autification.auth_key.clone(),
            }),
            get_limit: limit,
            nickname_for_get,
        });

        let result = self.grpc.get_latest_messages(request).await?;
        Ok(result.into_inner())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::get_client;
    use crate_proto::Notice;
    use fcore::test_utils::*;

    #[tokio::test]
    async fn too_big_message() {
        let mut client_to = get_client().await.unwrap();
        let client_from = get_client().await.unwrap();

        let text = get_rand_string(MAX_LEN_MESSAGE + 100);
        let error = client_to
            .send_message(
                client_from.data_for_autification.nickname,
                Message {
                    body: text.into_bytes(),
                    nonce: vec![],
                },
            )
            .await
            .err()
            .unwrap();

        assert!(matches!(error, Error::TooBigMessage));
    }

    #[tokio::test]
    #[allow(clippy::panic)]
    async fn send_message_and_subscribe() {
        let mut client_to = get_client().await.unwrap();
        let mut client_from = get_client().await.unwrap();
        let test_message = b"What hath God wrought!";

        let mut notification = client_from.subscribe_to_notifications().await.unwrap();
        client_to
            .send_message(
                client_from.data_for_autification.nickname.clone(),
                Message {
                    body: test_message.to_vec(),
                    nonce: vec![],
                },
            )
            .await
            .unwrap();

        let Some(notify) = notification.get_mut().message().await.unwrap() else {
            panic!("not found notification");
        };

        let Notice::NewMessage(new_message) = notify.notice.unwrap() else {
            panic!("wrong notification");
        };

        log::info!("new_message: {new_message:?}");
        log::info!("nickname_from: {}", notify.nickname_from);

        log::info!(
            "client_from: {}",
            client_from.data_for_autification.nickname
        );

        log::info!("client_to: {}", client_to.data_for_autification.nickname);

        assert_eq!(new_message.message.unwrap().body, test_message);
        assert_eq!(
            client_from.data_for_autification.nickname,
            notify.nickname_from
        );
    }

    async fn check_limit_for_get_latest_messages(size: i64) {
        let nickname = get_rand_string(20);
        let mut client = Client::registration(&nickname, ADDRESS_SERVER.parse().unwrap())
            .await
            .unwrap();

        client
            .get_latest_messages(vec![get_rand_string(20)], size)
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "ss")]
    async fn get_latest_messages_to_big_limit() {
        const LIMIT: i64 = MAX_LIMIT_GET_MESSAGES + 100;
        check_limit_for_get_latest_messages(LIMIT).await;
    }

    #[tokio::test]
    #[should_panic(expected = "ss")]
    async fn get_latest_messages_to_zero_limit() {
        const LIMIT: i64 = 0;
        check_limit_for_get_latest_messages(LIMIT).await;
    }

    #[tokio::test]
    #[should_panic(expected = "ss")]
    async fn get_latest_messages_to_negative_limit() {
        const LIMIT: i64 = -1;
        check_limit_for_get_latest_messages(LIMIT).await;
    }
}
