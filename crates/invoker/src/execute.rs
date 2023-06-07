use runtime::component::InstancePre;
use runtime::{Context, Store};

/// Execute when invoker invoked.
#[async_trait::async_trait]
pub trait InvokerExecutable {
    type RuntimeData: Default + Send + Sync + 'static;

    type Error;

    async fn run(self);

    async fn instantiate_pre(
        &self,
        service_id: &str,
    ) -> Result<
        (
            InstancePre<Context<Self::RuntimeData>>,
            Store<Self::RuntimeData>,
        ),
        Self::Error,
    >;
}
