pub mod cli;
mod file_service;

use crate::cli::Cli;
use crate::file_service::FileServiceImpl;
use proto::api::file_service_server::FileServiceServer;
use tonic::transport::Server;
use anyhow::Result;
use std::net::SocketAddr;

pub async fn server_main(args: &Cli) -> Result<()> {
    let socket_addr = SocketAddr::new(args.address, args.port);
    let file_service_impl = FileServiceImpl {};
    let file_service_server = FileServiceServer::new(file_service_impl);

    Server::builder()
        .add_service(file_service_server)
        .serve(socket_addr)
        .await?;

    Ok(())
}
