use anyhow::anyhow;
use clap::Parser;
use cli::{Cli, Operation};
use commands::{down::down, erase::erase, push::push, up::up};
use log::{error, info};

pub mod cli;
pub mod commands;
pub mod models;

#[tokio::main]
async fn main() {
    env_logger::init();
    let result = exec().await;
    if let Some(e) = result.err() {
        error!("{e:#}")
    }
}

async fn exec() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let res = match cli.operation {
        Operation::Up { service, target } => up(service, target).await,
        Operation::Push { service, target } => push(service, target).await,
        Operation::Down { service, target } => down(service, target).await,
        Operation::Erase { service, target } => erase(service, target).await,
        #[allow(unreachable_patterns)]
        _ => Err(anyhow!("Unsupported Operation")),
    };
    info!("Success");
    res
}
