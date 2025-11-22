#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error("I/O Error: {0}")]
    IoError(#[from] std::io::Error),
}
