use egccri_rust::consumer::Error;
use egccri_rust::handler;
use egccri_rust::handler::Event;
use egccri_rust::messaging_types::{open_broker, Channel};
use egccri_rust::producer::publish;

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
