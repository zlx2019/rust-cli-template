# rust-cli-template

[![Template CI](https://github.com/zlx2019/rust-cli-template/actions/workflows/template-ci.yml/badge.svg)](https://github.com/zlx2019/rust-cli-template/actions/workflows/template-ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.96.0%2B-orange.svg)](https://www.rust-lang.org)

一个 [cargo-generate](https://github.com/cargo-generate/cargo-generate) 项目模板，用于快速初始化带有完整工程化配置的 Rust **命令行（CLI）** 项目。内置基于 [clap](https://docs.rs/clap) 的命令行入口与子命令示例。

## 使用

```bash
cargo install cargo-generate
cargo generate zlx2019/rust-cli-template
```

生成过程中会依次询问：

| 选项 | 说明 |
|------|------|
| Github username | 用于生成 README 徽章与 Cargo.toml 仓库链接 |
| description | 项目简介 |
| license | 开源许可证（MIT / Apache-2.0 / GPL-3.0），自动生成对应 LICENSE 文件 |
| 异步运行时 | 可选 Tokio 及其 features（CLI 默认同步，仅额外引入依赖供按需使用） |
| 常用基础库 | uuid / rand / serde_json / chrono / reqwest / tracing |

> `clap`（参数解析）与 `anyhow`（错误处理）为 CLI 必备，已固定内置，无需选择。

## 模板内容

- `src/` 预置 CLI 脚手架：命令行入口、`clap` 定义、子命令模块与示例子命令 `greet`
- `rust-toolchain.toml` 锁定 Rust 工具链版本
- Lint 规则预设（`unsafe_code` / `missing_docs` / `unwrap_used` 告警）
- pre-commit 钩子：fmt / cargo-deny / typos / check / clippy / nextest
- GitHub Actions：CI（lint / test / deny / typos）与 Release（git-cliff 生成 Changelog）工作流

## 生成后

进入新项目目录，按项目内 README 指引安装开发工具并启用 pre-commit，即可开始开发：

```bash
cargo run -- greet Zero --count 2
```

## License

本项目采用 [MIT](./LICENSE) 许可证。
