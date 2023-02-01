use crate::output_print::FilesOutputPrint;
use anyhow::Result;
use proto::api::{
    file_service_client::FileServiceClient, upload_file_request, DownloadFileRequest,
    ListFilesRequest, UploadFileRequest,
};
use std::{net::IpAddr, path::PathBuf};
use tokio::{
    fs,
    io::{AsyncReadExt, AsyncWriteExt},
    sync::mpsc,
};
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::transport::{channel::Channel, Certificate, ClientTlsConfig};
use tracing::{debug, error, instrument, Instrument};

#[derive(Clone)]
pub struct FileClient<T> {
    client: FileServiceClient<T>,
}

impl<T> FileClient<T> {
    const CHANNEL_SIZE: usize = 10;
    const CHUNK_SIZE_BYTES: u64 = 1024 * 1024; // 1 MB
}

fn create_uri(host: &str, port: u16, tls: bool) -> String {
    let scheme = if tls { "https" } else { "http" };

    if let Ok(ip_addr) = host.parse::<IpAddr>() {
        return match ip_addr {
            IpAddr::V4(ipv4) => format!("{scheme}://{ipv4}:{port}"),
            IpAddr::V6(ipv6) => format!("{scheme}://[{ipv6}]:{port}"),
        };
    }

    format!("{scheme}://{host}:{port}")
}

fn create_tls_config(ca_cert_pem: &str, domain_name: &str) -> Result<ClientTlsConfig> {
    let ca = Certificate::from_pem(ca_cert_pem);

    let tls_config = ClientTlsConfig::new()
        .ca_certificate(ca)
        .domain_name(domain_name);

    Ok(tls_config)
}

impl FileClient<Channel> {
    #[instrument]
    pub async fn new(address: &str, port: u16, ca_cert_pem: Option<&str>) -> Result<Self> {
        let dst = create_uri(address, port, ca_cert_pem.is_some());

        debug!("Connecting to {}", dst);

        let mut endpoint = Channel::from_shared(dst)?;

        if let Some(ca_cert_pem) = ca_cert_pem {
            let tls_config = create_tls_config(ca_cert_pem, address)?;
            endpoint = endpoint.tls_config(tls_config)?;
        };

        let channel = endpoint.connect().await?;

        let client = FileServiceClient::new(channel);

        debug!("Connected");
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

        println!("{}", FilesOutputPrint::from(files));

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

        let mut file = fs::File::create(&file_path).await?;

        while let Some(item) = file_stream.next().await {
            file.write_all(&item?.chunk).await?
        }

        file.sync_all().await?;

        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn upload_file(&mut self, file: String, directory: PathBuf) -> Result<()> {
        let (tx, rx) = mpsc::channel(Self::CHANNEL_SIZE);

        let receiver_stream = ReceiverStream::new(rx);

        let mut file_path = PathBuf::new();
        file_path.push(&directory);
        file_path.push(&file);

        let task_handle = tokio::spawn(
            async move {
                if let Err(err) = tx
                    .send(UploadFileRequest {
                        r#type: Some(upload_file_request::Type::Name(file)),
                    })
                    .await
                {
                    error!(%err);
                    Err(err)?;
                }

                let file = fs::File::open(file_path).await?;
                let mut handle = file.take(Self::CHUNK_SIZE_BYTES);

                loop {
                    let mut chunk = Vec::with_capacity(Self::CHUNK_SIZE_BYTES as usize);

                    let n = handle.read_to_end(&mut chunk).await?;

                    if 0 == n {
                        break;
                    } else {
                        handle.set_limit(Self::CHUNK_SIZE_BYTES);
                    }

                    let request = UploadFileRequest {
                        r#type: Some(upload_file_request::Type::Chunk(chunk)),
                    };

                    if let Err(err) = tx.send(request).await {
                        error!(%err);
                        Err(err)?;
                    }

                    if n < Self::CHUNK_SIZE_BYTES as usize {
                        break;
                    }
                }

                Ok::<(), anyhow::Error>(())
            }
            .in_current_span(),
        );

        self.client.upload_file(receiver_stream).await?;

        if let Err(err) = task_handle.await? {
            error!(%err);
            Err(err)?;
        }

        Ok(())
    }
}
