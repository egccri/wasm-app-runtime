// Install app, actually wasm module pre instance.

/// apply command
#[derive(clap::Parser, Debug)]
pub struct InstallCommand {
    /// apply file path
    app_name: String,
}
