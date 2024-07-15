use thiserror::Error;

#[derive(Debug, Error)]
pub enum SharedError {
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    FetchError(#[from] reqwest::Error),
}
