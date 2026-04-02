#!/usr/bin/env bash

ARCH="$(dpkg --print-architecture)"

# cargo-binstall
curl --proto '=https' --tlsv1.2 --silent --fail --show-error --location https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh \
    | bash

# Download dev tools binaries
cargo binstall -y --log-level debug \
    cargo-llvm-cov \
    cargo-nextest \
    cargo-udeps \
    cargo-watch \
    cargo-insta

pipx install maturin
