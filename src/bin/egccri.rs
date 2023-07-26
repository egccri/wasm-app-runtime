// run server with config file.

use clap::error::ErrorKind;
use clap::Parser;
use egccri_runtime::commands::run::RunCommand;

/// Egccri runtime server
#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
enum EgccriServer {
    /// run egccri server
    Run(RunCommand),
}

impl EgccriServer {
    pub fn execute(self) -> anyhow::Result<()> {
        tracing::info!("Egccri standalone staring...");
        match self {
            EgccriServer::Run(run_command) => run_command.execute(),
        }
    }
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    // parse or else default run command.
    EgccriServer::try_parse()
        .unwrap_or_else(|e| match e.kind() {
            ErrorKind::InvalidSubcommand | ErrorKind::UnknownArgument => {
                EgccriServer::Run(RunCommand {})
            }
            _ => e.exit(),
        })
        .execute()
}
