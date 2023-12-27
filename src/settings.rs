use anyhow::anyhow;
use config::Config;
use registry::AppStoreConfig;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct ServerSetting {
    pub app_store: AppStoreConfig,
}

impl ServerSetting {
    pub fn new(config: Config) -> anyhow::Result<ServerSetting> {
        config.try_deserialize().map_err(|err| anyhow!("Server setting deserialize error: {}", err))
    }
}
