mod service;

use crate::service::InstallSvc;
use micro_async_module::{run_async_block_on, AsyncRuntime, Module};
use runtime_api::runtime::runtime_app::app_service_server::AppServiceServer;
use tonic::transport::Server;

const MODULE_NAME: &str = "runtime-server";

#[derive(Debug)]
pub struct RuntimeServer {
    server_addr: String,
}

impl Module for RuntimeServer {
    fn name(&self) -> &str {
        MODULE_NAME
    }

    fn start(&self, runtime: AsyncRuntime) {
        run_async_block_on(start(self.server_addr.clone()), runtime);
    }
}

impl RuntimeServer {
    pub fn new(server_addr: String) -> RuntimeServer {
        RuntimeServer { server_addr }
    }
}

async fn start(server_addr: String) {
    tracing::info!("Start runtime server with address: {}.", &server_addr);
    let addr = server_addr.parse().unwrap();
    let install_svc = InstallSvc::new();
    Server::builder()
        .add_service(AppServiceServer::new(install_svc))
        .serve(addr)
        .await
        .expect("Runtime server start failure!");
}

#[derive(thiserror::Error, Debug)]
pub enum RuntimeServerError {}
