#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Net Error: {0}")]
    Net(#[from] net::error::NetError),
    #[error("Server Error: {0}")]
    Server(#[from] server::error::ServerError),
    #[error("I/O Error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
