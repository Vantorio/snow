use crate::error::Result;

pub mod error;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");
    Ok(())
}
