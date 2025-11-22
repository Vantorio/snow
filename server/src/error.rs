#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error("Net Error: {0}")]
    Net(#[from] net::error::NetError),
    #[error("I/O Error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, ServerError>;
