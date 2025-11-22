use net::listener::Listener;

use crate::error::Result;

pub mod error;

pub struct Server;

impl Server {
    pub async fn new() -> Result<Self> {
        Ok(Server)
    }
}

impl Server {
    pub async fn start(&self) -> Result<()> {
        let listener = Listener::bind(19132).await?;
        listener.accept().await?;

        Ok(())
    }
}
