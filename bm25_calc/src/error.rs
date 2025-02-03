#![allow(clippy::missing_docs_in_private_items)]

use thiserror::Error;

/// Type alias for Result. Import this and use the ? to autoconvert to these errors
pub type Result<T> = std::result::Result<T, BM25Error>;

#[derive(Error, Debug)]
pub enum BM25Error {
    #[error("Unable to read file: {0}")]
    FSError(#[from] std::io::Error),
    #[error("Unexpected Serde JSON Error: {0}")]
    SerdeJSON(#[from] serde_json::Error),
    #[error("Unable to convert from an integer: {0}")]
    TryFromIntError(#[from] std::num::TryFromIntError),
}
