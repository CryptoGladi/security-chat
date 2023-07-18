use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("generate key for client")]
    GenerateClient(ring::error::Unspecified)
}