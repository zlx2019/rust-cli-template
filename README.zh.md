# rust-cli-template

[English](./README.md)

[![Template CI](https://github.com/zlx2019/rust-cli-template/actions/workflows/template-ci.yml/badge.svg)](https://github.com/zlx2019/rust-cli-template/actions/workflows/template-ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.96.0%2B-orange.svg)](https://www.rust-lang.org)

一个基于 [cargo-generate](https://github.com/cargo-generate/cargo-generate) 的 Rust **命令行应用**模板。生成的项目自带 [clap](https://docs.rs/clap) 命令行入口与子命令脚手架，以及完整的工程化配置：CI、发布自动化、格式化、Lint、测试、依赖审计、拼写检查和 pre-commit 钩子。

## 特性

- **开箱即用的 CLI 骨架**：参数解析、子命令分发、全局 `-v/--verbose` 选项，以及带单元测试的示例子命令 `greet`。
- **必备依赖内置**：`clap`（参数解析）与 `anyhow`（错误处理）已固定内置；Tokio 及 `serde_json`、`reqwest`、`tracing` 等常用库按需选择。
- **工具链锁定**：`rust-toolchain.toml` 保证本地与 CI 使用同一 Rust 版本。
- **严格而务实的 Lint 基线**：`unsafe_code`、`missing_docs`、`unwrap_used`、`expect_used`、`panic`、`dbg_macro` 在业务代码中告警，测试代码自动豁免。
- **CI 工作流**：每次 push / PR 自动运行格式化、Clippy、文档构建、nextest、cargo-deny 和拼写检查。
- **Release 工作流**：推送 `v*` 标签后自动用 git-cliff 生成 Changelog、构建 6 个平台的二进制（附 sha256 校验和），并可选发布到 crates.io。
- **开源协作文件**：贡献指南、安全策略、Issue 表单、PR 模板和 Dependabot 配置。

## 快速开始

### 1. 安装前置工具

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install --locked cargo-generate
```

### 2. 生成项目

```bash
cargo generate zlx2019/rust-cli-template --name my-cli
```

生成过程中会依次询问：

| 选项 | 说明 |
|------|------|
| `Github username` | 用于生成 README 徽章与 `Cargo.toml` 仓库链接 |
| `description` | 项目简介，写入 README 与 `Cargo.toml` |
| `license` | 开源许可证（`MIT` / `Apache-2.0` / `GPL-3.0`），自动生成对应 LICENSE 文件 |
| `ask_for_async` | 是否引入 Tokio 异步运行时（CLI 默认同步） |
| `multi_choice` | Tokio features，如 `full`、`rt-multi-thread`、`macros`、`time` |
| `ask_common_libs` | 常用基础库：`uuid`、`rand`、`serde_json`、`chrono`、`reqwest`、`tracing` |

### 3. 运行示例命令

```bash
cd my-cli
cargo run -- greet Ferris --count 2
```

```text
Hello, Ferris!
Hello, Ferris!
```

在 `src/commands/` 下新建模块并在 `src/cli.rs` 中注册，即可添加自己的子命令。

### 4. 安装开发工具

```bash
cargo install --locked cargo-deny
cargo install --locked cargo-nextest
cargo install --locked typos-cli
cargo install --locked git-cliff
pip install pre-commit
```

然后启用 pre-commit 钩子：

```bash
pre-commit install
```

## 生成后的目录结构

```text
.
├── .github/                 # CI、Release、Issue 和 PR 模板
├── examples/                # 可运行示例
├── fixtures/                # 测试数据
├── src/
│   ├── main.rs              # 入口：解析参数并分发
│   ├── cli.rs               # clap 定义与命令分发
│   └── commands/            # 子命令实现（示例：greet）
├── tests/                   # 集成测试
├── Cargo.toml               # 包元数据、依赖、lint 和 profile 配置
├── README.md                # 生成项目的 README
├── CONTRIBUTING.md          # 贡献指南
├── SECURITY.md              # 安全策略
├── deny.toml                # cargo-deny 配置
├── rustfmt.toml             # 格式化风格
├── clippy.toml              # Clippy lint 参数
├── rust-toolchain.toml      # Rust 工具链锁定
└── .pre-commit-config.yaml  # 本地提交前检查
```

## 本模板如何维护

本仓库自身也是一个模板：`Cargo.toml` 含 Liquid 占位符，无法作为普通 Rust crate 直接构建。Template CI 会先用 `cargo generate --path .` 展开模板，再对生成的项目运行完整检查。

本地验证模板：

```bash
cargo generate --path . --name smoke-test --destination /tmp
cd /tmp/smoke-test
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
cargo nextest run --all-features --no-tests pass
cargo deny check
typos
```

## License

本项目采用 [MIT](./LICENSE) 许可证。
