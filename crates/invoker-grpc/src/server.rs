pub mod caller;

use crate::server::caller::caller_server::{Caller, CallerServer};
use crate::server::caller::CallRequest;
use crate::{GrpcInvoker, InvokerGrpcError};
use std::net::SocketAddr;
use std::sync::Arc;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

pub async fn start(invoker: Arc<GrpcInvoker>) -> Result<(), InvokerGrpcError> {
    let addr = invoker.addr.parse::<SocketAddr>().unwrap();
    let caller_service = CallerService::new(invoker);
    Server::builder()
        .add_service(CallerServer::new(caller_service))
        .serve(addr)
        .await
        .map_err(|err| InvokerGrpcError::GrpcServerStartError(err))?;
    Ok(())
}

pub struct CallerService {
    invoker: Arc<GrpcInvoker>,
}

impl CallerService {
    pub fn new(invoker: Arc<GrpcInvoker>) -> Self {
        CallerService { invoker }
    }
}

#[tonic::async_trait]
impl Caller for CallerService {
    async fn call(&self, request: Request<CallRequest>) -> Result<Response<()>, Status> {
        let request = request.into_inner();
        let service_id = request.service_id;
        let payload = request.payload;
        let result = self
            .invoker
            .execute(service_id.to_string().as_str(), payload)
            .await;
        match result {
            Ok(result) => {
                println!("the result: {result}");
                Ok(Response::new(()))
            }
            Err(err) => Err(Status::internal(err.to_string())),
        }
    }
}
