use std::collections::HashMap;
use invoker::{InvokerContext, InvokerExecutable};
use std::path::Path;

mod server;

pub type RuntimeData = ();

/// Support a common rpc call with function service id.
pub struct GrpcInvoker {
    context: InvokerContext<Self>,
    /// Rpc bind address.
    addr: String,
    /// Function service id map to component id.
    components_router: HashMap<String, String>
}

impl InvokerExecutable for GrpcInvoker {
    type RuntimeData = RuntimeData;

    async fn run(&self) {
        server::start(self.server_addr.as_str()).await;
    }

    // call context, first, fetch component from registry, secondly, instantiate_pre
    async fn instantiate_pre() {
        todo!()
    }

    async fn execute(&self) {
        todo!()
    }
}


