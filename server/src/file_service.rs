use anyhow::anyhow;
use proto::api::file_service_server::FileService;
use proto::api::{
    DownloadFileRequest, DownloadFileResponse, ListFilesRequest, ListFilesResponse,
    UploadFileRequest, UploadFileResponse,
};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status, Streaming};
use tracing::{error, instrument, Instrument};

#[derive(Default)]
pub struct FileServiceImpl {
    directory: Arc<PathBuf>,
}

impl FileServiceImpl {
    const CHANNEL_SIZE: usize = 10;

    pub fn new(directory: PathBuf) -> Self {
        Self {
            directory: Arc::new(directory),
        }
    }
}

#[tonic::async_trait]
impl FileService for FileServiceImpl {
    type DownloadFileStream = ReceiverStream<Result<DownloadFileResponse, Status>>;
    type ListFilesStream = ReceiverStream<Result<ListFilesResponse, Status>>;

    #[instrument(skip(self))]
    async fn download_file(
        &self,
        request: Request<DownloadFileRequest>,
    ) -> Result<Response<Self::DownloadFileStream>, Status> {
        unimplemented!()
    }

    #[instrument(skip(self))]
    async fn upload_file(
        &self,
        request: Request<Streaming<UploadFileRequest>>,
    ) -> Result<Response<UploadFileResponse>, Status> {
        unimplemented!()
    }

    #[instrument(skip(self))]
    async fn list_files(
        &self,
        _request: Request<ListFilesRequest>,
    ) -> Result<Response<Self::ListFilesStream>, Status> {
        let (tx, rx) = mpsc::channel(Self::CHANNEL_SIZE);
        let directory = Arc::clone(&self.directory);
        let tx_error = tx.clone();

        tokio::spawn(
            async move {
                let result = async move {
                    let mut dir_stream = fs::read_dir(directory.as_ref()).await?;

                    while let Some(dir_entry) = dir_stream.next_entry().await? {
                        let file_metadata = dir_entry.metadata().await?;
                        if !file_metadata.is_file() {
                            continue;
                        }
                        let file_name = dir_entry.file_name().into_string().map_err(|e| {
                            anyhow!("OsString convertion failed: {:?}", e.to_string_lossy())
                        })?;
                        let file_size = file_metadata.len();
                        tx.send(Ok(ListFilesResponse {
                            name: file_name,
                            size: file_size,
                        }))
                        .await?;
                    }

                    Ok::<(), anyhow::Error>(())
                }
                .await;

                if let Err(err) = result {
                    error!(%err);
                    let send_result = tx_error
                        .send(Err(Status::internal("Failed to list files")))
                        .await;

                    if let Err(err) = send_result {
                        error!(%err);
                    }
                }
            }
            .in_current_span(),
        );

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}
