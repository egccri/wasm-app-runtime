use anyhow::{anyhow, bail};
use clap::error::ErrorKind;
use clap::Parser;
use egccri_runtime::commands::apply::ApplyCommand;
use egccri_runtime::commands::install::InstallCommand;
use std::fmt::format;
use std::path::PathBuf;
use tonic::codegen::Body;
use tonic::transport::Channel;

/// Runtime is a sub command of Egccri, it's used to deploy wasm components to the egccri
#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Egccri {
    /// Egccri server addr
    #[arg(short = 's', long = "server_addr", default_value = "127.0.0.1:9999")]
    server_addr: String,

    /// Sub command
    #[clap(subcommand)]
    command: Command,

    /// Server config file path, the default is "./config.toml"
    #[arg(
        short = 'c',
        long = "config-file",
        default_value = "config/config.toml"
    )]
    config_file: Option<PathBuf>,
}

#[derive(clap::Subcommand, Debug)]
enum Command {
    /// apply config
    Apply(ApplyCommand),

    /// install app
    Install(InstallCommand),
}

impl Egccri {
    pub async fn execute(self) -> anyhow::Result<()> {
        let server_addr = self.server_addr.clone();
        let channel = EgccriClient::connect(server_addr).await?;
        match self.command {
            Command::Apply(apply_command) => apply_command.execute(channel.clone()),
            Command::Install(install_command) => install_command.execute(channel.clone()),
        }
    }
}

pub struct EgccriClient {
    channel: Channel,
}

impl EgccriClient {
    pub async fn connect(server_addr: String) -> anyhow::Result<Channel> {
        tracing::info!("Connect to runtime server: {}", &server_addr);
        let server_addr = format!("http://{}", server_addr);
        Channel::from_shared(server_addr)
            .map_err(|err| anyhow!("Connect server error, cause by {err}"))?
            .connect()
            .await
            .map_err(|err| anyhow!("Connect server error, cause by {:?}", err))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    // parse or else default run command.
    Egccri::try_parse()
        .unwrap_or_else(|e| match e.kind() {
            ErrorKind::InvalidSubcommand | ErrorKind::UnknownArgument => {
                eprintln!("Error command, please check your command.");
                e.exit()
            }
            _ => e.exit(),
        })
        .execute()
        .await
}
