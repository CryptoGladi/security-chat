use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("transport problem: `{0}`")]
    Transport(#[from] tonic::transport::Error),

    #[error("api problem: `{0}`")]
    Api(#[from] tonic::Status),
}
