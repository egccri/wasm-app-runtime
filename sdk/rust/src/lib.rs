use crate::handler::{Error, Event};
use crate::producer::publish;
wit_bindgen::generate!({path: "../../wit/wasi-messaging/wit"});

struct EgccriMessaging;

impl handler::Handler for EgccriMessaging {
    fn on_receive(e: Event) -> Result<(), Error> {
        todo!()
    }
}

export_messaging!(EgccriMessaging);
