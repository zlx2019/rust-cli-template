# rust-cli-template

[简体中文](./README.zh.md)

[![Template CI](https://github.com/zlx2019/rust-cli-template/actions/workflows/template-ci.yml/badge.svg)](https://github.com/zlx2019/rust-cli-template/actions/workflows/template-ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.96.0%2B-orange.svg)](https://www.rust-lang.org)

A Rust **command-line application** template powered by [cargo-generate](https://github.com/cargo-generate/cargo-generate). It creates a production-ready CLI project with a [clap](https://docs.rs/clap)-based entry point, a subcommand scaffold, and a complete engineering setup: CI, release automation, formatting, linting, tests, dependency auditing, spell checking, and pre-commit hooks.

## Features

- **Ready-to-run CLI skeleton**: argument parsing, subcommand dispatch, a global `-v/--verbose` flag, and an example `greet` subcommand with unit tests.
- **Batteries included**: `clap` (argument parsing) and `anyhow` (error handling) are built in; Tokio and common crates such as `serde_json`, `reqwest`, and `tracing` are optional presets.
- **Pinned toolchain**: `rust-toolchain.toml` keeps local machines and CI on the same Rust version.
- **Strict but practical lints**: `unsafe_code`, `missing_docs`, `unwrap_used`, `expect_used`, `panic`, and `dbg_macro` warn in production code while tests stay exempt.
- **CI workflow**: formatting, Clippy, documentation builds, nextest, cargo-deny, and typos on every push and pull request.
- **Release workflow**: pushing a `v*` tag generates a changelog with git-cliff, builds binaries for six platforms with sha256 checksums, and optionally publishes to crates.io.
- **Collaboration files**: contribution guide, security policy, issue forms, a pull request template, and Dependabot configuration.

## Quick Start

### 1. Install Prerequisites

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install --locked cargo-generate
```

### 2. Generate A Project

```bash
cargo generate zlx2019/rust-cli-template --name my-cli
```

The generator will ask for the following values:

| Option | Description |
|--------|-------------|
| `Github username` | Used for README badges and `Cargo.toml` homepage/repository links |
| `description` | Project description written to README and `Cargo.toml` |
| `license` | Open source license, one of `MIT`, `Apache-2.0`, or `GPL-3.0`; the matching `LICENSE` file is generated automatically |
| `ask_for_async` | Whether to enable the Tokio async runtime (the CLI itself is synchronous by default) |
| `multi_choice` | Tokio features to enable, such as `full`, `rt-multi-thread`, `macros`, or `time` |
| `ask_common_libs` | Common crate choices: `uuid`, `rand`, `serde_json`, `chrono`, `reqwest`, and `tracing` |

### 3. Run The Example Command

```bash
cd my-cli
cargo run -- greet Ferris --count 2
```

```text
Hello, Ferris!
Hello, Ferris!
```

Add your own subcommand by creating a module under `src/commands/` and registering it in `src/cli.rs`.

### 4. Install Development Tools

```bash
cargo install --locked cargo-deny
cargo install --locked cargo-nextest
cargo install --locked typos-cli
cargo install --locked git-cliff
pip install pre-commit
```

Then enable pre-commit checks:

```bash
pre-commit install
```

## Generated Project Layout

```text
.
├── .github/                 # CI, release, issue, and pull request templates
├── examples/                # Runnable examples
├── fixtures/                # Test data
├── src/
│   ├── main.rs              # Entry point: parse arguments and dispatch
│   ├── cli.rs               # clap definitions and command dispatch
│   └── commands/            # Subcommand implementations (example: greet)
├── tests/                   # Integration tests
├── Cargo.toml               # Package metadata, dependencies, lints, and profile config
├── README.md                # README for the generated project
├── CONTRIBUTING.md          # Contribution guide
├── SECURITY.md              # Security policy
├── deny.toml                # cargo-deny configuration
├── rustfmt.toml             # Formatting style
├── clippy.toml              # Clippy lint parameters
├── rust-toolchain.toml      # Pinned Rust toolchain
└── .pre-commit-config.yaml  # Local pre-commit checks
```

## Maintaining This Template

This repository is itself a template. `Cargo.toml` contains Liquid placeholders, so the repository cannot be built directly as a normal Rust crate. Template CI first expands the template with `cargo generate --path .`, then runs the full check suite against the generated project.

To validate the template locally:

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

This project is licensed under [MIT](./LICENSE).
