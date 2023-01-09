use clap::Parser;
use server::{cli::Cli, server_main};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::try_parse()?;
    server_main(&args).await?;
    Ok(())
}
