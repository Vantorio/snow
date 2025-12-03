use core::init_tracing;

use server::Server;

use error::Result;
use tracing::info;

mod config;

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();

    let settings = config::Config::new()?;

    info!(
        "Configuration loaded. Host: {}, Port: {}",
        settings.network.host, settings.network.port
    );

    let server = Server::new(settings.network.host, settings.network.port).await?;

    server.start().await?;

    Ok(())
}
