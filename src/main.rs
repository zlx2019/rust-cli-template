//! CLI 程序入口。
//!
//! 负责解析命令行参数、分发到对应子命令，并将错误统一冒泡到顶层。
//! `main` 返回 [`anyhow::Result`]，出错时由 anyhow 打印错误链并以非零码退出。

mod cli;
mod commands;

use anyhow::Result;
use clap::Parser;

use crate::cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.run()
}
