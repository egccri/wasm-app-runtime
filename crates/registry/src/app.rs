use crate::warg::WargWrapper;
use crate::RegistryError::WargWrapperError;
use crate::{RegistryError, CELL};
use std::collections::HashMap;
use std::path::PathBuf;

// name: grpc-service, the package name of the warg, is equal App name.
// component_id: generate component id when publish
// path: sha256 of the warg generated, equal the path of content.
pub struct App {
    name: String,
    component_id: String,
    // the path of component file
    path: Option<PathBuf>,
}

pub struct AppStore {
    base_dir: String,
    apps: HashMap<String, App>,
}

impl AppStore {
    pub async fn install_app(app_name: &str) -> Result<App, RegistryError> {
        let warg_wrapper = CELL
            .get()
            .ok_or_else(|| WargWrapperError("Fetch warg wrapper not found".to_string()))?;
        let app_path = warg_wrapper.download(app_name, None).await?;
        Ok(App {
            name: app_name.to_string(),
            component_id: "".to_string(),
            path: Some(app_path),
        })
    }

    pub async fn update_app() -> Result<App, RegistryError> {
        todo!()
    }
}
