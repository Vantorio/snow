#[derive(Debug, thiserror::Error)]
pub enum NetError {
    #[error("Bind Error: {0}")]
    BindError(String),
    #[error("I/O Error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, NetError>;
