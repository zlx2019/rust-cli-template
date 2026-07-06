//! `greet` 示例子命令：演示位置参数、可选项与全局 `--verbose` 的配合使用。

use anyhow::Result;
use clap::Args;

#[derive(Debug, Args)]
pub struct GreetArgs {
    pub name: String,
    #[arg(short, long, default_value_t = 1)]
    pub count: u8,
}

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

    /// 校验 `greet` 在正常参数下可成功执行。
    #[test]
    fn greet_runs_without_error() {
        let args = GreetArgs {
            name: "Zero".to_string(),
            count: 2,
        };
        assert!(run(args, 0).is_ok());
    }
}
