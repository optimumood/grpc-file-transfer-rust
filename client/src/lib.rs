pub mod cli;
mod file_client;

use crate::cli::{Cli, Commands::*};
use anyhow::Result;
use file_client::FileClient;

pub async fn client_main(args: &Cli) -> Result<()> {
    let mut client = FileClient::new(args.address, args.port).await?;

    match &args.command {
        List => &mut client.list_files().await?,
        Download { file, directory } => {
            &mut client
                .download_file(file.clone(), directory.clone())
                .await?
        }
        Upload { .. } => unimplemented!(),
    };

    Ok(())
}
