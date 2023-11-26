use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("fern problem in init: {0}")]
    FernInit(#[from] fern::InitError),

    #[error("fern problem in build: {0}")]
    FernBuild(#[from] log::SetLoggerError),

    #[error("io problem: {0}")]
    IO(#[from] std::io::Error),
}

pub type LoggerResult<T> = Result<T, Error>;
