use anyhow::Result;
use clap::Parser;
use server::{cli::Cli, server_main};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .init();

    let args = Cli::try_parse()?;

    server_main(&args).await?;

    Ok(())
}
