_default:
    just --list

# Install deps and tools
install:
    rustup install
    rustup component add --toolchain nightly miri
    cargo fetch

# Update deps and tools
update:
    rustup update
    cargo update
    pre-commit autoupdate

alias up := update

# =============================================================================
# Development
# =============================================================================

# Run all checks
ci: (format "--check") lint ub-check test

# Autoformat code
[arg("check", long, value="--check")]
format check="":
    rustup run nightly cargo fmt {{check}}

alias fmt := format

# Run all linters
lint:
    cargo clippy --all-targets --all-features -- --deny warnings

# Run Undefined Behavior Check (Miri)
ub-check:
    MIRIFLAGS='-Zmiri-disable-isolation' \
        rustup run nightly cargo miri nextest run --workspace --all-targets --all-features

# Run all tests
test:
    cargo llvm-cov nextest --workspace --all-targets --all-features --lcov --output-path lcov.info
    cargo llvm-cov report --summary-only

# Apply autofixes
fix:
    cargo clippy --all-targets --all-features --fix --allow-dirty --allow-no-vcs -- --deny warnings
    rustup run nightly cargo fmt

# Build the application
build:
    cargo build --release

# Run application in dev mode
run *args="--help":
    cargo watch --exec 'run -- {{args}}'

# =============================================================================
# Utility
# =============================================================================

# Remove temporary files
clean:
    rm --recursive --force target/ lcov.info
    find . -path '*.log*' -delete
