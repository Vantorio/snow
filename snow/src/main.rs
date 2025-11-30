use server::Server;

use error::Result;
use tracing::info;
use tracing_subscriber::{self, fmt};

mod config;

#[tokio::main]
async fn main() -> Result<()> {
    fmt::Subscriber::builder()
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_timer(fmt::time::ChronoUtc::new("[%H:%M:%S]".into()))
        .init();

    let settings = config::Config::new()?;

    info!(
        "Configuration loaded. Host: {}, Port: {}",
        settings.network.host, settings.network.port
    );

    let server = Server::new(settings.network.host, settings.network.port).await?;

    server.start().await?;

    Ok(())
}
