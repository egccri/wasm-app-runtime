use std::borrow::BorrowMut;
pub use wasmtime::*;
pub use wasmtime_wasi::preview2::{Table, WasiCtx};
pub use wasmtime_wasi::*;

pub struct Wasi {
    wasi_ctx: WasiCtx,
}

impl Wasi {
    pub fn build() -> Self {
        Wasi {
            wasi_ctx: wasmtime_wasi::preview2::WasiCtxBuilder::new()
                .build(Table::new().borrow_mut())
                .unwrap(),
        }
    }

    pub fn wasi_ctx(self) -> WasiCtx {
        self.wasi_ctx
    }
}
