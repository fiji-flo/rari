use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, PartialEq, Clone, Copy, Error)]
pub enum EnvError {
    #[error("CONTENT_ROOT must be set")]
    NoContent,
    #[error("TRANSLATED_CONTENT_ROOT must be set")]
    NoTranslatedContent,
    #[error("BUILD_OUT_ROOT must be set")]
    NoBuildOut,
}

#[derive(Debug, Error)]
#[error("io error: {source} ({path})")]
pub struct RariIoError {
    pub path: PathBuf,
    pub source: std::io::Error,
}
