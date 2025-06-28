use snafu::prelude::*;
use std::array::TryFromSliceError;
use std::path::PathBuf;

/// Represents errors that can occur while working with JImage files.
#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum JImageError {
    #[snafu(display("File is not a valid jimage. Found magic: {magic:02x?}"))]
    Magic {
        magic: [u8; 4],
        context: &'static str,
    },
    #[snafu(display("Unsupported jimage version: {major_version}.{minor_version}"))]
    Version {
        major_version: u16,
        minor_version: u16,
    },
    #[snafu(display("Failed read from slice: [{from}..{to}]"))]
    RawRead { from: usize, to: usize },
    #[snafu(display(
        "Utf8 Error: {source}. Invalid data (lossy): '{}'",
        String::from_utf8_lossy(invalid_data)
    ))]
    Utf8 {
        #[snafu(source)]
        source: std::str::Utf8Error,
        invalid_data: Vec<u8>,
    },
    #[snafu(display("I/O error for path '{path}': {source}", path = path.display()))]
    Io {
        #[snafu(source)]
        source: std::io::Error,
        path: PathBuf,
    },
    #[snafu(display("Failed during zlib decompression: {source}"))]
    Decompression {
        #[snafu(source)]
        source: std::io::Error,
    },
    #[snafu(display("Unsupported decompressor: {decompressor_name}"))]
    UnsupportedDecompressor { decompressor_name: String },
    #[snafu(display("Internal error: {value}"))]
    Internal { value: String },
}

/// Type alias for a Result type that uses JImageError for error handling.
pub type Result<T> = std::result::Result<T, JImageError>;

impl From<TryFromSliceError> for JImageError {
    fn from(value: TryFromSliceError) -> Self {
        JImageError::Internal {
            value: value.to_string(),
        }
    }
}
