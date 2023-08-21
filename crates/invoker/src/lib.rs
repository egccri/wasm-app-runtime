pub use crate::execute::InvokerExecutable;
use runtime::component::{Component, InstancePre};
use runtime::{Context, RuntimeEngine, Store};
use std::sync::Arc;

// Invoker common utils, invoker is a trigger that watch egccri to instantiate and call it's export funcs.
// There's tcp, mqtt, rpc and http(maybe) invokers. For upcoming, stream compute invoker.
mod execute;

/// Support common method for InvokerExecutable
#[derive(Debug, Clone)]
pub struct InvokerContext<Executable: InvokerExecutable> {
    engine: Arc<RuntimeEngine<Executable::RuntimeData>>,
    // todo linker here
}

impl<Executable: InvokerExecutable> InvokerContext<Executable> {
    pub fn instantiate_pre(
        &self,
        component_id: &str,
    ) -> (
        InstancePre<Context<Executable::RuntimeData>>,
        Store<Executable::RuntimeData>,
    ) {
        let component = self.fetch_component(component_id);
        todo!()
    }

    pub fn fetch_component(&self, component_id: &str) -> Component {
        todo!()
    }
}
