use crate::{server, GrpcInvoker};
use host_bindgen::InvokerGrpc;
use invoker::{InvokerContext, InvokerExecutable};
use micro_async_module::{run_async_block_on, AsyncRuntime, Module};
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

#[derive(Debug)]
pub struct GrpcInvokerRunnable {
    context: Arc<InvokerContext<GrpcInvoker>>,
}

impl Module for GrpcInvokerRunnable {
    fn name(&self) -> &str {
        "grpc-invoker"
    }

    fn start(&self, runtime: AsyncRuntime) {
        let grpc_invoker = GrpcInvoker::new(self.context.clone());
        run_async_block_on(grpc_invoker.run(), runtime);
    }
}
