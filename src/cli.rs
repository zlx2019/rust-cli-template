//! 命令行接口定义。
//!
//! 使用 clap 的 derive API 声明全局参数与子命令。程序名、版本、描述等元信息
//! 由 clap 自动从 Cargo.toml 的 `CARGO_PKG_*` 环境变量读取，无需手动维护。

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::commands;

/// 顶层命令行参数。
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

/// 支持的子命令集合。
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// 向指定对象打招呼（示例子命令）。
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
