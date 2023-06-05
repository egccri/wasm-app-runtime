// build Store

// There should put wasi standard and common used component runtime data here.
// Then build app runtime data here.

// common used component like device, storage, and messaging.

use crate::host::HostData;
use wasmtime::StoreLimits;
use wasmtime_wasi::preview2::{Table, WasiCtx, WasiCtxBuilder, WasiView};

pub struct Store<T> {
    inner: wasmtime::Store<Context<T>>,
}

pub struct Context<T> {
    inner: T,
    wasi: Wasi,
    host_data: HostData,
    limit: StoreLimits,
}

pub struct Wasi {
    wasi_ctx: WasiCtx,
    table: Table,
}

impl Wasi {
    pub fn build() -> Self {
        let mut table = Table::new();
        Wasi {
            wasi_ctx: WasiCtxBuilder::new().build(&mut table).unwrap(),
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
