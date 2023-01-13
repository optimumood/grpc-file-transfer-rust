pub mod cli;
mod file_client;

use crate::cli::{Cli, Commands::*};
use anyhow::Result;
use file_client::FileClient;
use tonic::transport::channel::Channel;

pub async fn client_main(args: &Cli) -> Result<()> {
    let mut client = FileClient::new(args.address, args.port).await?;

    match &args.command {
        List => list_files(&mut client).await?,
        Download { .. } => unimplemented!(),
        Upload { .. } => unimplemented!(),
    }

    Ok(())
}

async fn list_files(client: &mut FileClient<Channel>) -> Result<()> {
    let files = client.list_files().await?;

    for file in files {
        println!("{:?}", file);
    }

    Ok(())
}
