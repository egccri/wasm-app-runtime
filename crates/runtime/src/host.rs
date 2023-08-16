/// Util that help add host component.
use crate::{Context, RuntimeError};
use std::any::Any;
use std::sync::Arc;
use wasmtime::component::Linker;

type AnyData = Box<dyn Any + Send>;

#[derive(Clone)]
pub struct HostComponents<T: HostComponent> {
    components: Arc<Vec<T>>,
}

pub struct HostData {
    data: Vec<Option<AnyData>>,
}

pub trait HostComponent: Send + Sync + 'static {
    /// Host component runtime data.
    type Data: Send + Sized + 'static;

    fn add_to_linker<T: Send>(
        &self,
        linker: &mut Linker<T>,
        get: impl Fn(&mut Context<T>) -> &mut Self::Data + Send + Sync + Copy + 'static,
    ) -> Result<(), RuntimeError>;

    /// Builds new host component runtime data for [`HostData`].
    fn build_data(&self) -> Self::Data;
}
