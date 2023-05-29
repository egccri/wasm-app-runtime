use runtime_core::component::Component;
use runtime_core::preview2::{wasi, WasiView};
use runtime_core::{component, Config, Engine, Store, Table, Wasi, WasiCtx};
use runtime_wasi_messaging::exports::wasi::messaging::handler::Event;
use runtime_wasi_messaging::{Messaging, WasmtimeMessaging};
use std::error::Error;

mod cli;

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

    let mut store = Store::new(&engine, ctx);

    // link wasi standard and wasi-messaging host functions.
    // what different between linker and component linker?
    let mut linker = component::Linker::<Ctx>::new(&engine);
    wasi::command::add_to_linker(&mut linker).unwrap();

    let mut linker = component::Linker::<Ctx>::new(&engine);
    WasmtimeMessaging::add_to_linker(&mut linker, |ctx: &mut Ctx| &mut ctx.wasi_messaging);

    // initial guest component.
    let component = Component::from_file(&engine, "guest.component.wasm")?;
    let (messaging, _) = Messaging::instantiate_async(&mut store, &component, &linker).await?;

    // call guest component from host
    let new_event = Event {
        data: Some(Vec::from("fizz".as_bytes())),
        id: "123".to_string(),
        source: "rust".to_string(),
        specversion: "1.0".to_string(),
        ty: "com.my-messaing.rust.fizzbuzz".to_string(),
        datacontenttype: None,
        dataschema: None,
        subject: None,
        time: None,
        extensions: None,
    };

    let res = messaging
        .wasi_messaging_handler()
        .call_on_receive(&mut store, &new_event)
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
