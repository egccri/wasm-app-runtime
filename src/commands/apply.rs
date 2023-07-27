use std::path::PathBuf;
use tonic::transport::Channel;

/// apply command
#[derive(clap::Parser, Debug)]
pub struct ApplyCommand {
    /// apply file path
    file: PathBuf,
}

impl ApplyCommand {
    pub fn execute(&self, channel: Channel) -> anyhow::Result<()> {
        // check engine init complete with wasi and other host component features

        // collect runtime data, include store data, component data.

        // build component

        // instance with hooks
        Ok(())
    }
}
