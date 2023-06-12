mod app;
mod warg;

use crate::warg::WargWrapper;
pub use app::{App, AppStore};
use std::sync::OnceLock;
use warg_client::ClientError;

static CELL: OnceLock<WargWrapper> = OnceLock::new();

#[derive(thiserror::Error, Debug)]
pub enum RegistryError {
    #[error("Warg client error cause by {0}")]
    WargClientError(#[from] ClientError),

    #[error("Warg wrapper error cause by {0}")]
    WargWrapperError(String),
}
