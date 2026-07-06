//! `greet` subcommand: prints a greeting a configurable number of times.

use anyhow::Result;
use clap::Args;

/// Arguments for the `greet` subcommand.
#[derive(Debug, Args)]
pub struct GreetArgs {
    /// Name to greet.
    pub name: String,

    /// Number of times to repeat the greeting.
    #[arg(short, long, default_value_t = 1)]
    pub count: u8,
}

/// Runs the `greet` subcommand: prints `Hello, <name>!` `count` times.
///
/// When `verbose` is non-zero, the resolved arguments are logged to stderr.
pub fn run(args: GreetArgs, verbose: u8) -> Result<()> {
    if verbose > 0 {
        eprintln!("[debug] greet name={} count={}", args.name, args.count);
    }
    for _ in 0..args.count {
        println!("Hello, {}!", args.name);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `greet` should run successfully with valid arguments.
    #[test]
    fn greet_runs_without_error() {
        let args = GreetArgs {
            name: "Zero".to_string(),
            count: 2,
        };
        assert!(run(args, 0).is_ok());
    }
}
