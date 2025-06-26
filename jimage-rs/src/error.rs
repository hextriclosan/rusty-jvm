use std::array::TryFromSliceError;
use thiserror::Error;

/// Represents errors that can occur while working with JImage files.
#[derive(Debug, Error)]
pub enum JImageError {
    #[error("File is not a valid jimage. Found magic: {0:#010x}")]
    Magic(u32),
    #[error("Unsupported jimage version: {major_version}.{minor_version}")]
    Version {
        major_version: u16,
        minor_version: u16,
    },
    #[error("Failed read from slice: [{from}..{to}]")]
    RawRead { from: usize, to: usize },
    #[error("Failed to create Utf8 string: {0}")]
    Utf8(#[from] std::str::Utf8Error),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Type alias for a Result type that uses JImageError for error handling.
pub type Result<T> = std::result::Result<T, JImageError>;

impl From<TryFromSliceError> for JImageError {
    fn from(value: TryFromSliceError) -> Self {
        JImageError::Internal(value.to_string())
    }
}
