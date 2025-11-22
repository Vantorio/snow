#[derive(Debug, thiserror::Error)]
pub enum NetError {
    #[error("Failed to bind to port {0}")]
    BindError(String),
    #[error("I/O Error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, NetError>;
