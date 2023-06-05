use egccri_runtime::commands::apply::ApplyCommand;

/// Runtime is a sub command of Egccri, it's used to deploy wasm components to the egccri
#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
enum Runtime {
    /// apply command
    Apply(ApplyCommand),
}

fn main() {
    // parse
}
