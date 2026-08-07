#!/usr/bin/env bash
set -e

cargo fmt
cargo clippy --workspace -- -D warnings
cargo test --workspace
cargo build --workspace --release

mkdir -p dist

cp target/release/esprit dist/ 2>/dev/null || true

echo "Release complete."
