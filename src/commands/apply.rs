use std::path::PathBuf;

/// apply command
#[derive(clap::Parser, Debug)]
pub struct ApplyCommand {
    /// apply file path
    file: PathBuf,
}

impl ApplyCommand {
    pub fn execute(&self) {
        // check engine init complete with wasi and other host component features

        // collect runtime data, include store data, component data.

        // build component

        // instance with hooks
    }
}
