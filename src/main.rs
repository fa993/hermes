use clap::Parser;
use cli::{Cli, Operation};
use commands::up::up;
use log::error;

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
    match cli.operation {
        Operation::Up { service, target } => up(service, target),
    }
    .await
}
