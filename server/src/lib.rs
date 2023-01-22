pub mod cli;
mod file_service;

use crate::cli::Cli;
use crate::file_service::FileServiceImpl;
use anyhow::{anyhow, Result};
use proto::api::file_service_server::FileServiceServer;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tonic::transport::server::TcpIncoming;
use tonic::transport::Server;

pub async fn server_main(args: &Cli) -> Result<()> {
    let socket_addr = SocketAddr::new(args.address, args.port.unwrap_or(0));
    let listener = TcpListener::bind(socket_addr).await?;
    let local_addr = listener.local_addr()?;
    let listener = TcpIncoming::from_listener(listener, true, None).map_err(|e| anyhow!(e))?;

    let file_service_impl = FileServiceImpl::new(args.directory.clone());
    let file_service_server = FileServiceServer::new(file_service_impl);

    println!("Server address {}", local_addr);

    Server::builder()
        .add_service(file_service_server)
        .serve_with_incoming(listener)
        .await?;

    Ok(())
}
