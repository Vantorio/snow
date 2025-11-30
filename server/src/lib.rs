use net::listener::Listener;
use tokio::signal;

use error::Result;

pub struct Server {
    host: String,
    port: u16,
}

impl Server {
    pub async fn new(host: String, port: u16) -> Result<Self> {
        Ok(Server { host, port })
    }
}

impl Server {
    pub async fn start(&self) -> Result<()> {
        let addr = format!("{}:{}", self.host, self.port);
        let listener = Listener::bind(addr).await?;

        tokio::select! {
            result = listener.listen() => {
                result?;
                Ok(())
            }
            _ = signal::ctrl_c() => {
                tracing::info!("Shutdown signal received. Shutting down server...");
                Ok(())
            }
        }
    }
}
