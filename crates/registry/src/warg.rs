use crate::RegistryError;
use serde::Deserialize;
use std::path::PathBuf;
use warg_client::{ClientError, Config, FileSystemClient, StorageLockResult};
use warg_protocol::registry::PackageId;
use warg_protocol::VersionReq;

/// Create when application started, hold by `APPSTORE`
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
        let config = Config {
            default_url: self.config.registry.clone(),
            registries_dir: self.config.registries_dir.clone(),
            content_dir: self.config.content_dir.clone(),
        };
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

    pub async fn download(
        &self,
        app_name: &str,
        version: Option<VersionReq>,
    ) -> Result<PathBuf, RegistryError> {
        let client = self.create_client().unwrap();

        let package_id = PackageId::new(app_name).map_err(
            |err| RegistryError::WargWrapperError(err.to_string())
        )?;

        let res = client
            .download(&package_id, version.as_ref().unwrap_or(&VersionReq::STAR))
            .await?
            .ok_or_else(|| RegistryError::WargWrapperError("download failed".to_string()))?;
        Ok(res.path)
    }
}
