#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error("I/O Error: {0}")]
    Io(#[from] std::io::Error),
}
