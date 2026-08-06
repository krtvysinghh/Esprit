use thiserror::Error;

pub type Result<T> = std::result::Result<T, EspritError>;

#[derive(Debug, Error)]
pub enum EspritError {
    #[error("{0}")]
    Message(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}
