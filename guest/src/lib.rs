use egccri_rust::exports::wasi::messaging::handler;
use egccri_rust::exports::wasi::messaging::handler::{Error, Event};
use egccri_rust::wasi::messaging::messaging_types::open_broker;
use egccri_rust::wasi::messaging::producer::{publish, Channel};

struct EgccriMessaging;

impl handler::Handler for EgccriMessaging {
    fn on_receive(e: Event) -> Result<(), Error> {
        println!(">>> Received: {:#?}", e);

        let data = e.data.unwrap();
        let data_s = String::from_utf8(data).unwrap();

        let broker = open_broker("test-broker")?;

        let new_event = EventParam {
            data: Some(msg.as_bytes()),
            id: "123",
            source: "rust",
            specversion: "1.0",
            ty: "com.my-messaing.rust.fizzbuzz",
            datacontenttype: None,
            dataschema: None,
            subject: None,
            time: None,
            extensions: None,
        };

        println!(">>> Publishing: {:#?}", new_event);
        publish(b, Channel::Topic("rust"), new_event)?;

        Ok(())
    }
}

export_messaging!(EgccriMessaging);
