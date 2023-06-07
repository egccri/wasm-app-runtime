use invoker::{InvokerContext, InvokerExecutable};
use runtime::component::{Instance, InstancePre};
use runtime::{AsContextMut, Context, Store, WasmtimeStore};
use std::collections::HashMap;

pub mod server;

#[derive(thiserror::Error, Debug)]
pub enum InvokerGrpcError {
    #[error("Grpc server start error, cause by: {0}")]
    GrpcServerStartError(#[from] tonic::transport::Error),

    #[error("Can not found component for service_id: {0}")]
    ComponentNotFoundError(String),

    #[error("Invoker fetch instance error")]
    InvokerFetchInstanceError,
}

pub type RuntimeData = ();

/// Support a common rpc call with function service id.
pub struct GrpcInvoker {
    context: InvokerContext<Self>,
    /// Rpc bind address.
    addr: String,
    /// Function service id map to component id.
    components_router: HashMap<String, String>,
}

#[async_trait::async_trait]
impl InvokerExecutable for GrpcInvoker {
    type RuntimeData = RuntimeData;

    type Error = InvokerGrpcError;

    async fn run(self) {
        let _ = server::start(self).await;
    }

    // call context, first, fetch component from registry, secondly, instantiate_pre
    async fn instantiate_pre(
        &self,
        service_id: &str,
    ) -> Result<(InstancePre<Context<RuntimeData>>, Store<RuntimeData>), InvokerGrpcError> {
        // Fetch component_id, if not, throw `ComponentNotFoundError`
        let component_id = self
            .components_router
            .get(service_id)
            .ok_or_else(|| InvokerGrpcError::ComponentNotFoundError(service_id.to_string()))?;
        Ok(self.context.instantiate_pre(component_id.as_str()))
    }
}

impl GrpcInvoker {
    async fn execute(&self, service_id: &str, payload: String) -> Result<String, InvokerGrpcError> {
        // todo instance_pre should use multi times, so this method should out this scope
        let (instance_pre, store) = self.instantiate_pre(service_id).await.unwrap();
        let mut wasmtime_store = store.inner();
        // todo take out instance method to the invoker context
        let (_, instance) =
            host_bindgen::InvokerGrpc::instantiate_pre(&mut wasmtime_store, &instance_pre)
                .await
                .unwrap();
        execute_impl(&mut wasmtime_store, instance, payload.as_str()).await
    }
}

async fn execute_impl(
    store: &mut WasmtimeStore<Context<RuntimeData>>,
    instance: Instance,
    payload: &str,
) -> Result<String, InvokerGrpcError> {
    let func = instance
        .exports(store.as_context_mut())
        .instance("grpc")
        .ok_or_else(|| InvokerGrpcError::InvokerFetchInstanceError)?
        .typed_func::<(&str,), (Result<String, host_bindgen::exports::gprc::Error>,)>("call")
        .unwrap();
    match func
        .call_async(store.as_context_mut(), (payload,)) // move param to the method
        .await
        .unwrap()
    {
        (Ok(result),) => Ok(result),
        _ => Err(InvokerGrpcError::InvokerFetchInstanceError),
    }
}
