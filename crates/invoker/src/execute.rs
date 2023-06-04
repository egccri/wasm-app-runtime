/// Execute when invoker invoked.
pub trait InvokerExecutable {
    type RuntimeData: Default + Send + Sync + 'static;

    async fn run(&self);

    async fn instantiate_pre();
}