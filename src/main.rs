//! Command-line entry point.
//!
//! Parses the CLI arguments and dispatches to the selected subcommand.

mod cli;
mod commands;

use anyhow::Result;
use clap::Parser;

use crate::cli::Cli;

/// Program entry point: parses arguments and runs the selected subcommand.
fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.run()
}
