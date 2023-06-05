/// Execute when invoker invoked.
#[async_trait::async_trait]
pub trait InvokerExecutable {
    type RuntimeData: Default + Send + Sync + 'static;

    async fn run(&self);

    async fn instantiate_pre();

    async fn execute(&self);
}
