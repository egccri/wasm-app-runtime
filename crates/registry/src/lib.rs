use std::collections::HashMap;
use std::path::PathBuf;
pub use warg_cli::commands;
pub use warg_cli::signing;

// name: grpc-service, the package name of the warg, is equal App name.
// component_id: generate component id when publish
// path: sha256 of the warg generated, equal the path of content.
pub struct App {
    name: String,
    component_id: String,
    // the path of component file
    path: PathBuf,
}

pub struct AppStore {
    base_dir: String,
    apps: HashMap<String, App>,
}

impl AppStore {
    pub fn fetch_component(component_id: &str) -> App {
        todo!()
    }
}
