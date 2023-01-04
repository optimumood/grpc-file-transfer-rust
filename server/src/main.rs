use proto::api::file_service_server::FileServiceServer;
use tonic::transport::Server;
mod file_service;
use file_service::FileServiceImpl;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:10000".parse().unwrap();

    let file_service_impl = FileServiceImpl {};

    let file_service_server = FileServiceServer::new(file_service_impl);

    Server::builder()
        .add_service(file_service_server)
        .serve(addr)
        .await?;

    Ok(())
}
