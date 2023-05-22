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
    async fn subscribe(&mut self, b: Broker, c: Channel) -> wasmtime::Result<Result<SubscriptionToken, Error>> {
        todo!()
    }

    async fn unsubscribe(&mut self, b: Broker, st: SubscriptionToken) -> wasmtime::Result<Result<(), Error>> {
        todo!()
    }
}

#[async_trait::async_trait]
impl producer::Host for WasmtimeMessaging {
    async fn publish(&mut self, b: producer::Broker, c: producer::Channel, e: Event) -> wasmtime::Result<Result<(), producer::Error>> {
        todo!()
    }
}



