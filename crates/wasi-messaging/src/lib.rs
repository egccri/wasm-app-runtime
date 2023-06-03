use crate::wasi::messaging::{consumer, messaging_types, producer};
use wasmtime::component::Linker;

wasmtime::component::bindgen!({path: "../../wit/wasi-messaging/wit", async: true});

#[derive(Debug, Clone)]
pub struct WasmtimeMessaging;

impl WasmtimeMessaging {
    pub fn add_to_linker<T, U>(
        linker: &mut Linker<T>,
        get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
    ) where
        T: Send,
        U: messaging_types::Host + consumer::Host + producer::Host + Send,
    {
        messaging_types::add_to_linker(linker, get).unwrap();
        consumer::add_to_linker(linker, get).unwrap();
        producer::add_to_linker(linker, get).unwrap();
    }
}

#[async_trait::async_trait]
impl consumer::Host for WasmtimeMessaging {
    async fn subscribe(
        &mut self,
        b: consumer::Broker,
        c: consumer::Channel,
    ) -> wasmtime::Result<Result<consumer::SubscriptionToken, consumer::Error>> {
        println!(">>> called subscribe");
        Ok(Ok("".to_string()))
    }

    async fn unsubscribe(
        &mut self,
        b: consumer::Broker,
        st: consumer::SubscriptionToken,
    ) -> wasmtime::Result<Result<(), consumer::Error>> {
        println!(">>> called unsubscribe");
        Ok(Ok(()))
    }
}

#[async_trait::async_trait]
impl producer::Host for WasmtimeMessaging {
    async fn publish(
        &mut self,
        b: producer::Broker,
        c: producer::Channel,
        e: producer::Event,
    ) -> wasmtime::Result<Result<(), producer::Error>> {
        println!(">>> called publish");
        Ok(Ok(()))
    }
}

#[async_trait::async_trait]
impl messaging_types::Host for WasmtimeMessaging {
    async fn drop_broker(&mut self, b: messaging_types::Broker) -> wasmtime::Result<()> {
        println!(">>> called drop_broker");
        Ok(())
    }

    async fn open_broker(
        &mut self,
        name: String,
    ) -> wasmtime::Result<Result<messaging_types::Broker, messaging_types::Error>> {
        println!(">>> called open_broker");
        Ok(Ok(0))
    }

    async fn drop_error(&mut self, e: messaging_types::Error) -> wasmtime::Result<()> {
        println!(">>> called drop_error");
        Ok(())
    }

    async fn trace_error(&mut self, e: messaging_types::Error) -> wasmtime::Result<String> {
        println!(">>> called trace");
        Ok("".to_string())
    }
}
