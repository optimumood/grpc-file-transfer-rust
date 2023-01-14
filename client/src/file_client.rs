use anyhow::Result;
use proto::api::file_service_client::FileServiceClient;
use proto::api::{DownloadFileRequest, ListFilesRequest};
use std::net::IpAddr;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio_stream::StreamExt;
use tonic::transport::channel::Channel;
use tracing::{info, instrument};

#[derive(Clone)]
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
        info!("Connected");
        Ok(Self { client })
    }

    #[instrument(skip(self))]
    pub async fn list_files(&mut self) -> Result<()> {
        let mut files = Vec::new();

        let response = self.client.list_files(ListFilesRequest {}).await?;

        let mut files_stream = response.into_inner();

        while let Some(item) = files_stream.next().await {
            files.push(item?);
        }

        for file in files {
            info!(?file);
        }

        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn download_file(&mut self, file: String, directory: PathBuf) -> Result<()> {
        let response = self
            .client
            .download_file(DownloadFileRequest {
                name: file.to_string(),
            })
            .await?;

        let mut file_stream = response.into_inner();
        let mut file_path = directory;
        file_path.push(file);

        let mut file = File::create(&file_path).await?;

        while let Some(item) = file_stream.next().await {
            file.write_all(&item?.chunk).await?
        }

        file.flush().await?;

        Ok(())
    }
}
