use runtime_core::component::Component;
use runtime_core::preview2::clocks::host;
use runtime_core::preview2::wasi;
use runtime_core::{component, Config, Engine, Store, Wasi, WasiCtx};
use runtime_wasi_messaging::handler::Event;
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

    let wasi = Wasi::build().wasi_ctx();
    let wasi_messaging = WasmtimeMessaging;

    let mut store = Store::new(&engine, (wasi, wasi_messaging));

    // link wasi standard and wasi-messaging host functions.
    // what different between linker and component linker?
    let mut linker = component::Linker::<WasiCtx>::new(&engine);
    // TODO wait preview2 impl WasiView
    wasi::command::add_to_linker(&mut linker).unwrap();

    let mut linker = component::Linker::<WasmtimeMessaging>::new(&engine);
    WasmtimeMessaging::add_to_linker(
        &mut linker,
        |mut wasmtime_messaging: &mut WasmtimeMessaging| &mut wasmtime_messaging,
    );

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
        .handler()
        .call_on_receive(&mut store, &new_event)
        .await?;

    println!(">>> called on_receive: {:#?}", res);

    Ok(())
}
