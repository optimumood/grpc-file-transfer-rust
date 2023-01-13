use anyhow::Result;
use proto::api::file_service_client::FileServiceClient;
use proto::api::{ListFilesRequest, ListFilesResponse};
use std::net::IpAddr;
use tokio_stream::StreamExt;
use tonic::transport::channel::Channel;
use tracing::{info, instrument};

pub struct FileClient<T> {
    client: FileServiceClient<T>,
}

impl FileClient<Channel> {
    #[instrument]
    pub async fn new(address: IpAddr, port: u16) -> Result<Self> {
        let dst = match address {
            IpAddr::V4(ipv4) => format!("http://{}:{}", ipv4, port),
            IpAddr::V6(ipv6) => format!("http://[{}]:{}", ipv6, port),
        };

        info!("Connecting to {}", dst);
        let client = FileServiceClient::connect(dst).await?;
        Ok(Self { client })
    }

    #[instrument(skip(self))]
    pub async fn list_files(&mut self) -> Result<Vec<ListFilesResponse>> {
        let mut files = Vec::new();

        let response = self.client.list_files(ListFilesRequest {}).await?;

        let mut files_stream = response.into_inner();

        while let Some(item) = files_stream.next().await {
            files.push(item?);
        }

        Ok(files)
    }
}
