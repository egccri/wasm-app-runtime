pub use wasmtime_wasi::WasiCtx as WasiCtx;

pub struct Wasi {
    wasi_ctx: WasiCtx,
}

impl Wasi {
    pub fn wasi_ctx() -> Self {
        Wasi {
            wasi_ctx: wasmtime_wasi::WasiCtxBuilder::new().build()
        }
    }
}