use proto::api::file_service_server::FileService;
use proto::api::{
    DownloadFileRequest, DownloadFileResponse, ListFilesRequest, ListFilesResponse,
    UploadFilesRequest, UploadFilesResponse,
};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status, Streaming};

#[derive(Default)]
pub struct FileServiceImpl;

#[tonic::async_trait]
impl FileService for FileServiceImpl {
    type DownloadFileStream = ReceiverStream<Result<DownloadFileResponse, Status>>;
    type UploadFileStream = ReceiverStream<Result<UploadFilesResponse, Status>>;
    type ListFilesStream = ReceiverStream<Result<ListFilesResponse, Status>>;

    async fn download_file(
        &self,
        request: Request<DownloadFileRequest>,
    ) -> Result<Response<Self::DownloadFileStream>, Status> {
        unimplemented!()
    }
    async fn upload_file(
        &self,
        request: Request<Streaming<UploadFilesRequest>>,
    ) -> Result<Response<Self::UploadFileStream>, Status> {
        unimplemented!()
    }
    async fn list_files(
        &self,
        request: Request<ListFilesRequest>,
    ) -> Result<Response<Self::ListFilesStream>, Status> {
        unimplemented!()
    }
}
