#[derive(clap::Parser, Debug)]
pub struct RegistryCommand {
    #[clap(subcommand)]
    commands: Commands,
}

#[derive(clap::Subcommand, Debug)]
pub enum Commands {
    /// Push a wasm app to a registry.
    Push(Push),
    /// Pull a wasm app from a registry.
    Pull(Pull),
    /// Login to a registry.
    Login(Login),
}

#[derive(clap::Parser, Debug)]
pub struct Push {}

#[derive(clap::Parser, Debug)]
pub struct Pull {}

#[derive(clap::Parser, Debug)]
pub struct Login {}
