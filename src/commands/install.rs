// Install app, actually wasm module pre instance.

use tonic::transport::Channel;

/// apply command
#[derive(clap::Parser, Debug)]
pub struct InstallCommand {
    /// App name
    app_name: String,

    /// App version
    version: String,
}

impl InstallCommand {
    pub fn execute(&self, channel: Channel) -> anyhow::Result<()> {
        tracing::info!("Install app: {}-{}", &self.app_name, &self.version);
        InstallClient::install_app(self.app_name.clone(), self.version.clone(), channel);
        Ok(())
    }
}

// Add some auth
pub struct InstallClient;

impl InstallClient {
    pub async fn install_app(
        app_name: String,
        version: String,
        channel: Channel,
    ) -> anyhow::Result<()> {
        let mut client =
            runtime_api::runtime::runtime_app::app_service_client::AppServiceClient::new(channel);
        let request = runtime_api::runtime::runtime_app::AppInstallRequest { app_name, version };
        client.install(tonic::Request::new(request));
        Ok(())
    }
}
