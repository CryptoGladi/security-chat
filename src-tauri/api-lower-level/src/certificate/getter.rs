//! Module for obtaining information for a [certificate](Certificate)

use crate::certificate::connection_parameters::ConnectionParameters;
use crate::certificate::Certificate;
use async_trait::async_trait;

pub mod impl_json;

#[async_trait]
pub trait Getter {
    async fn get(&self, connection_parameters: ConnectionParameters) -> Certificate;
}

pub async fn simple_get<T: Getter>(
    getter: &T,
    connection_parameters: ConnectionParameters,
) -> Certificate {
    getter.get(connection_parameters).await
}

#[cfg(test)]
mod tests {
    use super::Getter;
    use crate::certificate::connection_parameters::ConnectionParameters;
    use crate::certificate::Certificate;
    use async_trait::async_trait;

    fn get_test_certificate() -> Certificate {
        Certificate {
            link: "test_link".to_string(),
            valid_hash: "sha512".to_string(),
            ..Default::default()
        }
    }

    struct TestGetter;

    #[async_trait]
    impl Getter for TestGetter {
        async fn get(&self, connection_parameters: ConnectionParameters) -> Certificate {
            Certificate {
                connection_parameters,
                ..get_test_certificate()
            }
        }
    }

    #[tokio::test]
    async fn get() {
        let test_getter = TestGetter;
        let certificate = test_getter.get(ConnectionParameters::default()).await;

        assert_eq!(
            certificate,
            Certificate {
                connection_parameters: ConnectionParameters::default(),
                ..get_test_certificate()
            }
        );
    }

    #[tokio::test]
    async fn simple_get() {
        let certificate = super::simple_get(&TestGetter, ConnectionParameters::default()).await;

        assert_eq!(
            certificate,
            Certificate {
                connection_parameters: ConnectionParameters::default(),
                ..get_test_certificate()
            }
        );
    }
}
