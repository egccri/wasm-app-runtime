use crate::RegistryError;
use serde::Deserialize;
use std::path::PathBuf;
use warg_client::{ClientError, Config, FileSystemClient, StorageLockResult};
use warg_protocol::VersionReq;

/// Create when application started, hold by `OnceCell`
pub struct WargWrapper {
    config: WargConfig,
}

/// Config from config file or command line.
#[derive(Deserialize, Debug, Clone)]
pub struct WargConfig {
    /// registry url, default from egccri app store.
    pub registry: Option<String>,

    /// The path to the registries directory to use.
    pub registries_dir: Option<PathBuf>,

    /// The path to the content directory to use.
    pub content_dir: Option<PathBuf>,
}

impl WargWrapper {
    pub fn new(config: WargConfig) -> Self {
        WargWrapper { config }
    }

    pub fn create_client(&self) -> Result<FileSystemClient, ClientError> {
        let config = self.map_client_config().unwrap();
        match FileSystemClient::try_new_with_config(self.config.registry.as_deref(), &config)? {
            StorageLockResult::Acquired(client) => Ok(client),
            StorageLockResult::NotAcquired(path) => {
                println!(
                    "blocking on lock for directory `{path}`...",
                    path = path.display()
                );

                FileSystemClient::new_with_config(self.config.registry.as_deref(), &config)
            }
        }
    }

    // FIXME: Remove this config from file, add from `WargClientConfig`
    pub fn map_client_config(&self) -> Result<Config, RegistryError> {
        Config::from_file("../../../config/config.json")
            .map_err(|err| RegistryError::WargWrapperError(err.to_string()))
    }

    pub async fn download(
        &self,
        app_name: &str,
        version: Option<VersionReq>,
    ) -> Result<PathBuf, RegistryError> {
        let client = self.create_client().unwrap();

        let res = client
            .download(app_name, version.as_ref().unwrap_or(&VersionReq::STAR))
            .await?
            .ok_or_else(|| RegistryError::WargWrapperError("download failed".to_string()))?;
        Ok(res.path)
    }
}
