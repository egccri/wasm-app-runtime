use crate::{Context, RuntimeError};
/// Util that help add host component.
use wasmtime::component::Linker;

pub struct HostData {}

pub trait HostComponent: Send + Sync + 'static {
    /// Host component runtime data.
    type Data: Send + Sized + 'static;

    fn add_to_linker<T: Send>(
        linker: &mut Linker<T>,
        get: impl Fn(&mut Context<T>) -> &mut Self::Data + Send + Sync + Copy + 'static,
    ) -> Result<(), RuntimeError>;

    /// Builds new host component runtime data for [`HostData`].
    fn build_data(&self) -> Self::Data;
}
