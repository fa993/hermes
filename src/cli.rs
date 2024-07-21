use std::path::PathBuf;

use clap::{command, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// sub operation to run
    #[command(subcommand)]
    pub(crate) operation: Operation,
}

#[derive(Subcommand)]
pub enum Operation {
    /// To do a cold start
    Up {
        /// Path to service.toml file
        service: PathBuf,
        /// Path to target.toml file
        target: PathBuf,
    },
    /// To update a deployment
    Push {
        /// Path to service.toml file
        service: PathBuf,
        /// Path to target.toml file
        target: PathBuf,
    },
}
