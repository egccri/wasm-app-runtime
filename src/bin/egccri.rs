use clap::error::ErrorKind;
use clap::Parser;
use config::{Config, File};
use egccri_runtime::commands::run::RunCommand;
use egccri_runtime::settings;
use egccri_runtime::settings::ServerSetting;
/// Run server with config file.
use std::path::PathBuf;

/// Egccri runtime server
#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct EgccriServer {
    /// Sub command
    #[clap(subcommand)]
    command: ServerCommand,

    /// Server config file path, the default is "./config/config.toml"
    #[arg(
        short = 'c',
        long = "config-file",
        default_value = "config/config.toml"
    )]
    config_file: PathBuf,
}

#[derive(clap::Subcommand, Debug)]
enum ServerCommand {
    /// Run egccri server
    Run(RunCommand),
}

impl EgccriServer {
    pub fn execute(self, setting: ServerSetting) -> anyhow::Result<()> {
        tracing::info!("Egccri standalone staring...");
        match self.command {
            ServerCommand::Run(run_command) => run_command.execute(setting),
        }
    }

    pub fn config_file(&self) -> PathBuf {
        self.config_file.clone()
    }
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    // parse or else default run command.
    let egccri_server = EgccriServer::try_parse().unwrap_or_else(|e| match e.kind() {
        ErrorKind::InvalidSubcommand | ErrorKind::UnknownArgument => EgccriServer {
            command: ServerCommand::Run(RunCommand {
                server_addr: "0.0.0.0:9999".to_string(),
            }),
            config_file: "config/config.toml".parse().unwrap(),
        },
        _ => e.exit(),
    });
    let setting = Config::builder()
        .add_source(File::from(egccri_server.config_file()))
        .build()?;
    let setting = ServerSetting::new(setting)?;
    egccri_server.execute(setting)
}
