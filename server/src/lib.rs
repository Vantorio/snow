use net::listener::Listener;
use tokio::signal;

use tracing::info;

use error::Result;

const CURRENT_MINECRAFT_VERSION: &str = "1.21.124";

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

        info!("Starting server for MCBE v{}", CURRENT_MINECRAFT_VERSION,);

        tokio::select! {
            result = listener.listen() => {
                result?;
                Ok(())
            }
            _ = signal::ctrl_c() => {
                info!("Shutdown signal received. Shutting down server...");
                Ok(())
            }
        }
    }
}
