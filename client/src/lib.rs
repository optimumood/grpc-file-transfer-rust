mod file_client;

use proto::api::file_service_client::FileServiceClient;
use proto::api::ListFilesRequest;
use tokio_stream::StreamExt;

pub async fn client_main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = FileServiceClient::connect("http://[::1]:50051")
        .await
        .unwrap();

    let response = client.list_files(ListFilesRequest {}).await.unwrap();

    let mut resp_stream = response.into_inner();

    while let Some(item) = resp_stream.next().await {
        println!("\treceived: {:?}", item.unwrap());
    }

    println!("Hello, world!");

    Ok(())
}
