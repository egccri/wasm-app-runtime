use egccri_runtime::commands::apply::ApplyCommand;
use egccri_runtime::commands::install::InstallCommand;

/// Runtime is a sub command of Egccri, it's used to deploy wasm components to the egccri
#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
enum Egccri {
    /// apply config
    Apply(ApplyCommand),

    /// install app
    Install(InstallCommand),
}

fn main() {
    // parse
}
