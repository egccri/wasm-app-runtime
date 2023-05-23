use crate::consumer::{Broker, Channel, Error, SubscriptionToken};
use crate::producer::Event;

wasmtime::component::bindgen!({
    path: "../../wit/wasi-messaging/wit",
    world: "messaging",
    async: true
});

struct WasmtimeMessaging;

#[async_trait::async_trait]
impl consumer::Host for WasmtimeMessaging {
    async fn subscribe(
        &mut self,
        b: Broker,
        c: Channel,
    ) -> wasmtime::Result<Result<SubscriptionToken, Error>> {
        todo!()
    }

    async fn unsubscribe(
        &mut self,
        b: Broker,
        st: SubscriptionToken,
    ) -> wasmtime::Result<Result<(), Error>> {
        todo!()
    }
}

#[async_trait::async_trait]
impl producer::Host for WasmtimeMessaging {
    async fn publish(
        &mut self,
        b: producer::Broker,
        c: producer::Channel,
        e: Event,
    ) -> wasmtime::Result<Result<(), producer::Error>> {
        todo!()
    }
}

impl messaging_types::Host for WasmtimeMessaging {
    async fn drop_broker(&mut self, b: messaging_types::Broker) -> wasmtime::Result<()> {
        todo!()
    }

    async fn open_broker(
        &mut self,
        name: String,
    ) -> wasmtime::Result<Result<messaging_types::Broker, messaging_types::Error>> {
        todo!()
    }

    async fn drop_error(&mut self, e: messaging_types::Error) -> wasmtime::Result<()> {
        todo!()
    }

    async fn trace_error(&mut self, e: messaging_types::Error) -> wasmtime::Result<String> {
        todo!()
    }
}
