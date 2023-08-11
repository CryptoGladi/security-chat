use super::*;

impl Client {
    pub async fn send_message(
        &mut self,
        nickname_from: String,
        message: Message,
    ) -> Result<(), Error> {
        if message.body.len() >= MAX_LEN_MESSAGE {
            return Err(Error::TooBigMessage);
        }

        let request = tonic::Request::new(SendMessageRequest {
            nickname: Some(Check {
                nickname: self.data.nickname.clone(),
                authkey: self.data.auth_key.clone(),
            }),
            nickname_from,
            message: Some(message),
        });

        self.api.send_message(request).await?;
        Ok(())
    }

    pub async fn get_latest_messages(
        &mut self,
        nickname_for_get: Vec<String>,
        limit: i64,
    ) -> Result<GetLatestMessagesReply, Error> {
        if limit <= 0 {
            return Err(Error::InvalidArgument("limit <= 0".to_string()));
        }
        if limit > MAX_LIMIT_GET_MESSAGES.try_into().unwrap() {
            return Err(Error::InvalidArgument(
                "limit > MAX_LIMIT_GET_MESSAGES".to_string(),
            ));
        }

        let request = tonic::Request::new(GetLatestMessagesRequest {
            nickname: Some(Check {
                nickname: self.data.nickname.clone(),
                authkey: self.data.auth_key.clone(),
            }),
            get_limit: limit,
            nickname_for_get,
        });

        let result = self.api.get_latest_messages(request).await?;
        Ok(result.into_inner())
    }
}
