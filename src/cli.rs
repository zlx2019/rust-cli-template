//! CLI definition and command dispatch.

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::commands;

/// Top-level command-line interface: global options plus the subcommand to run.
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Subcommand to execute.
    #[command(subcommand)]
    pub command: Commands,

    /// Increase log verbosity; repeat to raise the level (e.g. `-vv`).
    #[arg(short, long, global = true, action = clap::ArgAction::Count)]
    pub verbose: u8,
}

/// Supported subcommands.
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Example command
    Greet(commands::greet::GreetArgs),
}

impl Cli {
    /// Dispatches to and runs the selected subcommand.
    pub fn run(self) -> Result<()> {
        match self.command {
            Commands::Greet(args) => commands::greet::run(args, self.verbose),
        }
    }
}
