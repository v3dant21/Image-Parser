use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Inspect {
        path: PathBuf,
    },
    Metadata {
        path: PathBuf,
    },
}
