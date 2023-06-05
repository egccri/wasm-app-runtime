mod caller;

use crate::server::caller::caller_server::{Caller, CallerServer};
use crate::server::caller::CallRequest;
use crate::InvokerGrpcError;
use std::net::SocketAddr;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

pub async fn start(addr: &str) -> Result<(), InvokerGrpcError> {
    let addr = addr.parse::<SocketAddr>().unwrap();
    let caller_service = CallerService;
    Server::builder()
        .add_service(CallerServer::new(caller_service))
        .serve(addr)
        .await
        .map_err(|err| InvokerGrpcError::GrpcServerStartError(err))?;
    Ok(())
}

pub struct CallerService;

#[tonic::async_trait]
impl Caller for CallerService {

    async fn call(&self, request: Request<CallRequest>) -> Result<Response<()>, Status> {
        todo!()
    }
}
