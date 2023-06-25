use runtime::component::Component;
use runtime::preview2::{wasi, Table, WasiCtx, WasiView};
use runtime::{component, Config, Engine, Wasi, WasmtimeStore};
use std::error::Error;
use wasi_messaging::wasi::messaging::messaging_types;
use wasi_messaging::{Messaging, WasmtimeMessaging};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // first initial engine
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);

    let engine = Engine::new(&config)?;

    let wasi = Wasi::build();
    let wasi_messaging = WasmtimeMessaging;

    let ctx = Ctx {
        wasi,
        wasi_messaging,
    };

    let mut store = WasmtimeStore::new(&engine, ctx);

    // link wasi standard and wasi-messaging host functions.
    // what different between linker and component linker?
    let mut linker = component::Linker::<Ctx>::new(&engine);
    wasi::command::add_to_linker(&mut linker).unwrap();

    WasmtimeMessaging::add_to_linker(&mut linker, |ctx: &mut Ctx| &mut ctx.wasi_messaging);

    // initial guest component and pre instantiate.
    let component = Component::from_file(&engine, "guest.component.wasm")?;
    let instance_pre = linker.instantiate_pre(&component).unwrap();

    // instantiate
    let (messaging, _) = Messaging::instantiate_pre(&mut store, &instance_pre).await?;

    // pretend to have received a message
    let msg = messaging_types::Message {
        data: vec![1, 2, 3],
        format: messaging_types::FormatSpec::Http,
        metadata: None,
    };

    let res = messaging
        .wasi_messaging_messaging_guest()
        .call_handler(&mut store, &[&msg])
        .await?;

    println!(">>> called on_receive: {:#?}", res);

    Ok(())
}

pub struct Ctx {
    wasi: Wasi,
    wasi_messaging: WasmtimeMessaging,
}

impl WasiView for Ctx {
    fn table(&self) -> &Table {
        self.wasi.table()
    }

    fn table_mut(&mut self) -> &mut Table {
        self.wasi.table_mut()
    }

    fn ctx(&self) -> &WasiCtx {
        self.wasi.ctx()
    }

    fn ctx_mut(&mut self) -> &mut WasiCtx {
        self.wasi.ctx_mut()
    }
}
