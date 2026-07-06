//! `greet` 示例子命令：演示位置参数、可选项与全局 `--verbose` 的配合使用。

use anyhow::Result;
use clap::Args;

/// `greet` 子命令的参数。
#[derive(Debug, Args)]
pub struct GreetArgs {
    /// 要问候的对象名称。
    pub name: String,

    /// 重复问候的次数。
    #[arg(short, long, default_value_t = 1)]
    pub count: u8,
}

/// 执行 `greet` 子命令：按 `count` 次数向 `name` 打招呼。
///
/// `verbose` 为全局详细级别，大于 0 时向 stderr 输出调试信息。
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
