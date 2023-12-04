use chrono::{DateTime, Local};
use derivative::Derivative;
use jwt::{Header, Token};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};

/// Token for get [`AccessToken`]
///
/// Unlimited lifetime
///
/// **IT IS VERY SECRET VALUE!**
pub type RefreshToken = String;

/// Token for login in your account
#[derive(Derivative, Serialize, Deserialize, Default, Clone, PartialEq)]
#[derivative(Debug)]
pub struct Tokens {
    #[derivative(Debug = "ignore")] // ignore because is secret value
    pub refresh_token: RefreshToken,
    pub access_token: AccessToken,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub nickname: String,

    /// Expiration Time
    pub exp: DateTime<Local>,
}

/// Token for execution on your account.
///
/// There is a lifetime limit! Determined by server
///
/// **IT IS SECRET VALUE!**
///
/// Format: [JWT](https://jwt.io/)
#[derive(Derivative, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct AccessToken(pub String);

impl AccessToken {
    /// Get expiration Time
    pub fn get_exp(&self) -> Result<DateTime<Local>, jwt::Error> {
        let token: Token<Header, Claims, _> = jwt::Token::parse_unverified(&self.0)?;
        Ok(token.claims().exp)
    }

    pub fn get_nickname(&self) -> Result<String, jwt::Error> {
        let token: Token<Header, Claims, _> = jwt::Token::parse_unverified(&self.0)?;
        Ok(token.claims().nickname.clone())
    }
}

impl Display for AccessToken {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl Debug for AccessToken {
    #[allow(clippy::unwrap_in_result)]
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("AccessToken")
            .field("nickname", &self.get_nickname().unwrap())
            .field("exp", &self.get_exp().unwrap())
            .finish()
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::AccessToken;

    /// Test `JWT` token
    ///
    /// Your can testing this by [website](https://jwt.io/)
    pub(crate) mod test_jwt_token {
        pub const RAW: &str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJuaWNrbmFtZSI6IkRCbjJLWlg5TU9Fd0JNM2RIMG9zIiwiZXhwIjoiMjAyMy0xMi0wNVQxNDozMDo0MS41OTgwMzYxODErMDM6MDAifQ.6_gC9KMQVpwG51-jkM7XNgGO3P6CAFkdgcBkFM5Br_s";
        pub const NICKNAME: &str = "DBn2KZX9MOEwBM3dH0os";
        pub const EXP: &str = "2023-12-05 14:30:41.598036181 +03:00";
    }

    #[test]
    fn invalid_token() {
        let access_token = AccessToken("INVALID_TOKEN".to_string());

        assert!(access_token.get_exp().is_err());
        assert!(access_token.get_nickname().is_err());
    }

    #[test]
    fn get_nickname() {
        let access_token = AccessToken(test_jwt_token::RAW.to_string());

        assert_eq!(
            access_token.get_nickname().unwrap(),
            test_jwt_token::NICKNAME
        );
    }

    #[test]
    fn get_exp() {
        let access_token = AccessToken(test_jwt_token::RAW.to_string());

        assert_eq!(
            access_token.get_exp().unwrap().to_string(),
            test_jwt_token::EXP
        );
    }

    #[test]
    fn impl_debug() {
        let access_token = AccessToken(test_jwt_token::RAW.to_string());
        log::info!("{access_token:?}");
    }
}
