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
        // check engine init complete with wasi and other host component features

        // collect runtime data, include store data, component data.

        // build component

        // instance with hooks
        Ok(())
    }
}
