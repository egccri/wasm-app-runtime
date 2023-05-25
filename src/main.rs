use std::error::Error;
use wasmtime::component::Component;
use wasmtime::{component, Config, Linker, Store};
use runtime_core::Wasi;
use runtime_wasi_messaging::{Messaging, WasmtimeMessaging};

mod cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    // first initial engine
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);

    let engine = wasmtime::Engine::new(&config)?;

    let mut wasi = Wasi::wasi_ctx();
    let wasi_messaging = WasmtimeMessaging;

    let store = Store::new(&engine, wasi);

    // link wasi standard
    let mut linker = Linker::new(&engine);

    wasmtime_wasi::add_to_linker(&mut linker, |wasi: &mut Wasi| {wasi}).unwrap();

    // add custom component like mqtt to the wasmtime use wasm component-model linker.



    let component = Component::from_file(&engine, "guest.component.wasm")?;
    let (messaging, _) = Messaging::instantiate_async(&mut store, &component, &linker).await?;



    Ok(())
}
