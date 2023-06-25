use egccri_rust::messaging::exports;
use egccri_rust::messaging::exports::wasi::messaging::messaging_guest::{
    Error, GuestConfiguration, Message, MessagingGuest,
};

struct EgccriMessaging;

impl MessagingGuest for EgccriMessaging {
    fn configure() -> Result<GuestConfiguration, Error> {
        todo!()
    }

    fn handler(ms: Vec<Message>) -> Result<(), Error> {
        todo!()
    }
}

egccri_rust::messaging::export_messaging!(EgccriMessaging);
