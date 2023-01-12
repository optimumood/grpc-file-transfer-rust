use anyhow::Result;
use clap::Parser;
use server::{cli::Cli, server_main};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::try_parse()?;
    server_main(&args).await?;
    Ok(())
}
