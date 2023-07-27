use runtime_api::runtime::runtime_app::{AppInstallReply, AppInstallRequest};
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct InstallSvc {}

impl InstallSvc {
    pub fn new() -> InstallSvc {
        InstallSvc {}
    }
}

#[tonic::async_trait]
impl runtime_api::runtime::runtime_app::app_service_server::AppService for InstallSvc {
    async fn install(
        &self,
        request: Request<AppInstallRequest>,
    ) -> Result<Response<AppInstallReply>, Status> {
        tracing::debug!("Receive install call: {:?}", request.into_inner());
        Err(Status::ok("Not implement yet!"))
    }
}
