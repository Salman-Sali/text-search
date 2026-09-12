use tantivy::{TantivyError, directory::error::OpenDirectoryError};
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("{0}")]
    TantivyError(TantivyError),
    #[error("{0}")]
    OpenDirectoryError(OpenDirectoryError),
}

impl From<TantivyError> for Error {
    fn from(value: TantivyError) -> Self {
        Self::TantivyError(value)
    }
}

impl From<OpenDirectoryError> for Error {
    fn from(value: OpenDirectoryError) -> Self {
        Self::OpenDirectoryError(value)
    }
}
