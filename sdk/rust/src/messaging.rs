// wit_bindgen::generate!("messaging" in "../../wit/wasi-messaging/wit");

wit_bindgen::generate!({
    macro_export,
    path: "../../wit/wasi-messaging/wit",
    world: "messaging",
});

pub use export_messaging;
