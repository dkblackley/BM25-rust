use thiserror::Error;

pub(crate) type Result<T> = std::result::Result<T, BM25Error>;

#[derive(Error, Debug)]
pub enum BM25Error {
    #[error("Unable to read file: {0}")]
    FSError(#[from] std::io::Error),
    #[error("Unexpected Serde JSON Error: {0}")]
    SerdeJSON(#[from] serde_json::Error),
    #[error("unknown data store error")]
    Unknown,
}
