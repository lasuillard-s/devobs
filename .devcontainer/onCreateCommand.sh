#!/usr/bin/env bash

ARCH="$(dpkg --print-architecture)"

CARGO_BINSTALL_VERSION="1.12.5"

curl -fsSL "https://github.com/cargo-bins/cargo-binstall/releases/download/v${CARGO_BINSTALL_VERSION}/cargo-binstall-$(rustc -vV | sed -n 's|host: ||p').tgz" \
    | tar --extract --gzip --directory "${CARGO_HOME}/bin"

# Download dev tools binaries
cargo binstall -y --log-level debug \
    cargo-llvm-cov \
    cargo-nextest \
    cargo-udeps \
    cargo-watch
