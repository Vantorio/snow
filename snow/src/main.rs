use server::Server;

use crate::error::Result;

pub mod error;

#[tokio::main]
async fn main() -> Result<()> {
    let server = Server::new().await?;
    server.start().await?;
    Ok(())
}
