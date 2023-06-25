use crate::wasi::messaging::consumer::{Channel, Client, Error, GuestConfiguration, Message};
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
    async fn subscribe_try_receive(
        &mut self,
        c: Client,
        ch: Channel,
        t_milliseconds: u32,
    ) -> wasmtime::Result<Result<Option<Vec<Message>>, Error>> {
        todo!()
    }

    async fn subscribe_receive(
        &mut self,
        c: Client,
        ch: Channel,
    ) -> wasmtime::Result<Result<Vec<Message>, Error>> {
        todo!()
    }

    async fn update_guest_configuration(
        &mut self,
        gc: GuestConfiguration,
    ) -> wasmtime::Result<Result<(), Error>> {
        todo!()
    }

    async fn complete_message(&mut self, m: Message) -> wasmtime::Result<Result<(), Error>> {
        todo!()
    }

    async fn abandon_message(&mut self, m: Message) -> wasmtime::Result<Result<(), Error>> {
        todo!()
    }
}

#[async_trait::async_trait]
impl producer::Host for WasmtimeMessaging {
    async fn send(
        &mut self,
        c: producer::Client,
        ch: producer::Channel,
        m: Vec<producer::Message>,
    ) -> wasmtime::Result<Result<(), producer::Error>> {
        todo!()
    }
}

#[async_trait::async_trait]
impl messaging_types::Host for WasmtimeMessaging {
    async fn disconnect(&mut self, c: messaging_types::Client) -> wasmtime::Result<()> {
        todo!()
    }

    async fn connect(
        &mut self,
        name: String,
    ) -> wasmtime::Result<Result<messaging_types::Client, messaging_types::Error>> {
        todo!()
    }

    async fn drop_error(&mut self, e: messaging_types::Error) -> wasmtime::Result<()> {
        todo!()
    }

    async fn trace(&mut self, e: messaging_types::Error) -> wasmtime::Result<String> {
        todo!()
    }
}
