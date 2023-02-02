pub mod cli;
mod file_client;
mod output_print;

use crate::cli::{Cli, Commands::*};
use anyhow::Result;
use file_client::FileClient;

pub async fn client_main(args: &Cli) -> Result<()> {
    let mut ca_cert_pem_str = None;
    if let Some(ca_cert_pem) = &args.ca_cert {
        ca_cert_pem_str = Some(std::fs::read_to_string(ca_cert_pem)?);
    }

    let mut cert_pem_str = None;
    if let Some(cert_pem) = &args.cert {
        cert_pem_str = Some(std::fs::read_to_string(cert_pem)?);
    }

    let mut key_pem_str = None;
    if let Some(key_pem) = &args.key {
        key_pem_str = Some(std::fs::read_to_string(key_pem)?);
    }

    let mut client = FileClient::new(
        &args.address,
        args.port,
        ca_cert_pem_str.as_deref(),
        cert_pem_str.as_deref(),
        key_pem_str.as_deref(),
    )
    .await?;

    match &args.command {
        List => &mut client.list_files().await?,
        Download { file, directory } => {
            &mut client
                .download_file(file.clone(), directory.clone())
                .await?
        }
        Upload { file, directory } => {
            &mut client.upload_file(file.clone(), directory.clone()).await?
        }
    };

    Ok(())
}
