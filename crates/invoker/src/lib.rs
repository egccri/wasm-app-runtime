use runtime::component::Component;
use runtime::Store;
pub use crate::execute::InvokerExecutable;

// Invoker common utils, invoker is a trigger that watch egccri to instantiate and call it's export funcs.
// There's tcp, mqtt, rpc and http(maybe) invokers. For upcoming, stream compute invoker.
mod execute;

pub struct InvokerContext<Executable> {

}

impl <Executable: InvokerExecutable> InvokerContext<Executable> {
    pub fn instantiate_pre() -> (Component, Store<Executable::RuntimeData>) {
        todo!()
    }

    pub fn fetch_component() -> Component {
        todo!()
    }
}

