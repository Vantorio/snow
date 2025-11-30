#[derive(thiserror::Error, Debug)]
pub enum NetError {
    #[error("Bind Error: {0}")]
    BindError(String),
}

#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error("Net Layer Error: {0}")]
    Net(#[from] NetError),
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Server Error: {0}")]
    Server(#[from] ServerError),
    #[error("Net Layer Error: {0}")]
    Net(#[from] NetError),
    #[error("Configuration Error: {0}")]
    Config(#[from] config::ConfigError),
    #[error("I/O Error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
