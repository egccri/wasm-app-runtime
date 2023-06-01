use std::path::PathBuf;

#[derive(clap::Subcommand, Debug)]
pub struct ApplyCommand {
    file: PathBuf
}

impl ApplyCommand {
    pub fn execute(&self) {
        // check engine init complete with wasi and other host component features

        // collect runtime data, include store data, component data.

        // build component

        // instance with hooks
    }
}