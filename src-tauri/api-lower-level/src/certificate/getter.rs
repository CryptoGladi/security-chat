//! Module for obtaining information for a [certificate](Certificate)

use crate::certificate::connection_parameters::ConnectionParameters;
use crate::certificate::Certificate;
use async_trait::async_trait;
use std::path::PathBuf;

pub mod impl_json;

#[async_trait]
pub trait Getter {
    async fn get(&self, path: PathBuf, connection_parameters: ConnectionParameters) -> Certificate;
}

pub async fn simple_get<T: Getter>(
    getter: &T,
    path: PathBuf,
    connection_parameters: ConnectionParameters,
) -> Certificate {
    getter.get(path, connection_parameters).await
}

#[cfg(test)]
mod tests {
    use super::Getter;
    use crate::certificate::connection_parameters::ConnectionParameters;
    use crate::certificate::Certificate;
    use async_trait::async_trait;
    use std::path::PathBuf;

    fn get_test_certificate() -> Certificate {
        Certificate {
            link: "test_link".to_string(),
            hash: "sha512".to_string(),
            ..Default::default()
        }
    }

    struct TestGetter;

    #[async_trait]
    impl Getter for TestGetter {
        async fn get(
            &self,
            path: PathBuf,
            connection_parameters: ConnectionParameters,
        ) -> Certificate {
            Certificate {
                path,
                connection_parameters,
                ..get_test_certificate()
            }
        }
    }

    #[tokio::test]
    async fn get() {
        let test_getter = TestGetter;
        let certificate = test_getter
            .get(
                PathBuf::from("/home/gladi/hentai"),
                ConnectionParameters::default(),
            )
            .await;

        assert_eq!(
            certificate,
            Certificate {
                link: "test_link".to_string(),
                hash: "sha512".to_string(),
                path: PathBuf::from("/home/gladi/hentai"),
                connection_parameters: ConnectionParameters::default(),
            }
        );
    }

    #[tokio::test]
    async fn simple_get() {
        let certificate = super::simple_get(
            &TestGetter,
            PathBuf::from("/home/gladi/loli"),
            ConnectionParameters::default(),
        )
        .await;

        assert_eq!(
            certificate,
            Certificate {
                link: "test_link".to_string(),
                hash: "sha512".to_string(),
                path: PathBuf::from("/home/gladi/loli"),
                connection_parameters: ConnectionParameters::default()
            }
        );
    }
}
