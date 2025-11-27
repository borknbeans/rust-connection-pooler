use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConnectionPoolerError {
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}