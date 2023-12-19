mod app;
mod warg;

pub use app::{App, AppStore, AppStoreConfig};
use std::sync::OnceLock;
use warg_client::ClientError;

static APP_STORE: OnceLock<AppStore> = OnceLock::new();

#[derive(thiserror::Error, Debug)]
pub enum RegistryError {
    #[error("Warg client error cause by {0}")]
    WargClientError(#[from] ClientError),

    #[error("Warg wrapper error cause by {0}")]
    WargWrapperError(String),
}
