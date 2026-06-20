# ❤️‍🔥 Contributing to this project

Thank you for your interest in contributing to **devobs**.

## 🐛 Reporting issues

Please report issues in our [GitHub repository](https://github.com/lasuillard-s/devobs/issues). Before submitting an issue, please search for existing issues to avoid duplicates.

## 🏗️ Project overview

This project is a CLI tool written in Rust for development workflow automation. It provides various commands to handle development tasks such as checking for matching file pairs or detecting changes in the target directory by comparing file hashes before and after running a command.

### 🛠️ Tech stack

This project uses the following tech stack:

- [Rust](https://www.rust-lang.org)
- [clap](https://docs.rs/clap/latest/clap/) for command-line parsing
- [Clippy](https://doc.rust-lang.org/clippy/) for linting, and [rustfmt](https://github.com/rust-lang/rustfmt) for formatting
- [Miri](https://github.com/rust-lang/miri) for undefined behavior checks
- [cargo-nextest](https://nexte.st/) for test coverage and execution

### 📂 Key directory structure

- `src/`: Rust source code
- `tests/`: Project integration tests
- `Cargo.toml`: Rust package configuration
- `flake.nix`: Nix Flakes development environment
- `Justfile`: Local development commands
- `pyproject.toml`: Python package configuration for [Maturin](https://www.maturin.rs/)
- `rust-toolchain.toml`: Rust toolchain configuration

## 🔧 Set up the development environment

For development, the following tools are required:

### ❄️ Tools managed via Nix Flakes

This repository uses [Nix Flakes](https://nix.dev/concepts/flakes.html) to manage development tools. The following tools are installed automatically when `nix` is available:

- `pre-commit`
- `just`
- `pipx`
- `rustup`
- `cargo`
- `cargo-llvm-cov`
- `cargo-nextest`
- `cargo-watch`
- `cargo-insta`
- `maturin`

Simply run `nix develop` to enter the development environment, then run `just install` to set up dependencies.

If you prefer using a [Dev Container](https://containers.dev), an example configuration file ([`devcontainer.json`](./.devcontainer.example/devcontainer.json)) is provided.

## ✅ Verifying changes

Before pushing your code, run `just ci` to verify that your changes adhere to the project's coding standards and pass all linters, formatters, and tests.

Alternatively, use the `pre-commit` hooks to handle formatting, linting, and quick test feedback automatically.

## ✨ Submitting changes

Please feel free to submit pull requests on GitHub. Before opening a PR, ensure your changes pass all checks by running `just ci`.

## 🚀 Release process

`devobs` is published to PyPI as a Python package via [Maturin](https://github.com/PyO3/maturin), to make it easy to install and use. To release a new version, follow these steps:

1. Dispatch [Prepare Release](https://github.com/lasuillard-s/devobs/actions/workflows/prepare-release.yaml) workflow with the new version (e.g. `v0.1.0`)
1. Review and merge the PR created by the workflow
1. Create and publish a new release in GitHub Releases
1. Application will be built with Maturin and published to PyPI automatically
