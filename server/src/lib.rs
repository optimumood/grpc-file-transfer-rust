mod file_service;

use crate::file_service::FileServiceImpl;
use proto::api::file_service_server::FileServiceServer;
use tonic::transport::Server;

pub async fn server_main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();

    let file_service_impl = FileServiceImpl {};

    let file_service_server = FileServiceServer::new(file_service_impl);

    Server::builder()
        .add_service(file_service_server)
        .serve(addr)
        .await?;

    Ok(())
}
