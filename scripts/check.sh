#!/bin/bash
# Ensure formatting and clippy is done on nightly.

set -ex

# Change to our project home.
script_dir=$(dirname "${BASH_SOURCE[0]}")
script_home=$(realpath "${script_dir}")
cd "${script_home}"/..

# Make sure we error on warnings, and don't format in-place.

# Do the formatting and clippy for all our project workspaces.
cargo +nightly fmt -- --check
cargo +nightly clippy --no-default-features -- --deny warnings
cargo +nightly clippy --features=compact -- --deny warnings
cargo +nightly clippy --features=format,radix -- --deny warnings
cargo +nightly clippy --features=f16 -- --deny warnings
cargo +nightly clippy --all-features -- --deny warnings

# ASM, size, and benchmarks use separate workspaces, do those separately.
cd lexical-asm
cargo +nightly fmt -- --check
cargo +nightly clippy --all-features -- --deny warnings

cd ../lexical-size
cargo +nightly fmt -- --check
cargo +nightly clippy --all-features -- --deny warnings

cd ../lexical-benchmark
cargo +nightly fmt -- --check
cargo +nightly clippy --all-features --benches -- --deny warnings

# Check our docs will be valid
cd ..
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --no-default-features
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --features=radix,format,f16
