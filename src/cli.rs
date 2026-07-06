//! 命令行接口定义。

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::commands;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// 待执行的子命令。
    #[command(subcommand)]
    pub command: Commands,

    /// 输出更详细的日志，可叠加（如 `-vv` 提高级别）。
    #[arg(short, long, global = true, action = clap::ArgAction::Count)]
    pub verbose: u8,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Example command
    Greet(commands::greet::GreetArgs),
}

impl Cli {
    /// 根据解析结果分发并执行对应子命令。
    pub fn run(self) -> Result<()> {
        match self.command {
            Commands::Greet(args) => commands::greet::run(args, self.verbose),
        }
    }
}
