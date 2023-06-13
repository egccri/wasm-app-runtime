//! This is `Embedder` core.
//!
//! A WebAssembly implementation will typically be embedded into a host environment. This environment
//! defines how loading of modules is initiated, how imports are provided (including host-side definitions),
//! and how exports can be accessed. However, the details of any particular embedding are beyond the scope of this specification,
//! and will instead be provided by complementary, environment-specific API definitions.
mod componentize;
mod host;
mod registry;
mod store;

// Re-export for the main demo, remove this later.
pub use wasmtime::Store as WasmtimeStore;
pub use wasmtime::*;
pub use wasmtime_wasi::*;

// Export here.
pub use crate::host::{HostComponent, HostData};
pub use crate::store::{Context, Store, Wasi};

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum RuntimeError {
    #[error("Host linker error")]
    HostLinkerError,
}

pub struct RuntimeEngine<T> {
    inner: Engine,
    // hold linker here.
    linker: component::Linker<T>,
}

// hold Config here
pub struct RuntimeConfig {
    inner: Config,
}
