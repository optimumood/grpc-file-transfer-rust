pub mod cli;
mod file_service;

use crate::cli::Cli;
use crate::file_service::FileServiceImpl;
use anyhow::Result;
use proto::api::file_service_server::FileServiceServer;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tonic::transport::Server;

pub async fn server_main(args: &Cli) -> Result<()> {
    let socket_addr = SocketAddr::new(args.address, args.port.unwrap_or(0));
    let listener = TcpListener::bind(socket_addr).await?;

    let file_service_impl = FileServiceImpl::new(args.directory.clone());
    let file_service_server = FileServiceServer::new(file_service_impl);

    println!("Server address {}", listener.local_addr()?);

    Server::builder()
        .add_service(file_service_server)
        .serve_with_incoming(tokio_stream::wrappers::TcpListenerStream::new(listener))
        .await?;

    Ok(())
}
