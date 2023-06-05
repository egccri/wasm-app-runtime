use invoker::InvokerExecutable;
use std::path::Path;

mod server;

pub type RuntimeData = ();

pub struct GrpcInvoker {
    server_addr: String,
}

impl InvokerExecutable for GrpcInvoker {
    type RuntimeData = RuntimeData;

    async fn run(&self) {
        server::start(self.server_addr.as_str()).await;
    }

    async fn instantiate_pre() {
        todo!()
    }
}
