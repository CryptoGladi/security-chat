use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("problem in creating a temporary folder: {0}")]
    TempDir(std::io::Error),

    #[error("downloader: {0}")]
    Downloader(#[from] downloader::Error),

    #[error("io problem: {0}")]
    IO(#[from] std::io::Error),
}
