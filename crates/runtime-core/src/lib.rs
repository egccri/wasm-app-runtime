use std::borrow::BorrowMut;
pub use wasmtime::*;
pub use wasmtime_wasi::preview2::{Table, WasiCtx};
use wasmtime_wasi::preview2::{WasiCtxBuilder, WasiView};
pub use wasmtime_wasi::*;

pub struct Wasi {
    wasi_ctx: WasiCtx,
    table: Table,
}

impl Wasi {
    pub fn build() -> Self {
        let mut table = Table::new();
        Wasi {
            wasi_ctx: WasiCtxBuilder::new().build(table.borrow_mut()).unwrap(),
            table,
        }
    }
}

impl WasiView for Wasi {
    fn table(&self) -> &Table {
        &self.table
    }

    fn table_mut(&mut self) -> &mut Table {
        &mut self.table
    }

    fn ctx(&self) -> &WasiCtx {
        &self.wasi_ctx
    }

    fn ctx_mut(&mut self) -> &mut WasiCtx {
        &mut self.wasi_ctx
    }
}
