use crate::handler::{Error, Event};
wit_bindgen::generate!({path: "../../wit/wasi-messaging/wit"});

struct EgccriMessaging;

impl handler::Handler for EgccriMessaging {
    fn on_receive(e: Event) -> Result<(), Error> {
        todo!()
    }
}

export_messaging!(EgccriMessaging);