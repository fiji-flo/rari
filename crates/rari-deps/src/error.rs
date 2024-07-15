use rari_shared::error::SharedError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DepsError {
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    SharedError(#[from] SharedError),
    #[error("no version for webref")]
    WebRefMissingVersionError,
    #[error("no tarball for webref")]
    WebRefMissingTarballError,
}
