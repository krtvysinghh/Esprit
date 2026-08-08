#!/usr/bin/env bash
set -Eeuo pipefail

echo "========== ESPRIT PRODUCTION AUDIT =========="

cargo fmt --all --check

cargo clippy --workspace --all-targets --all-features -- -D warnings

cargo test --workspace --all-features

cargo build --workspace --all-features --release

cargo doc --workspace --no-deps

cargo publish --workspace --dry-run --allow-dirty

cargo audit

cargo deny check all

cargo machete

cargo udeps --workspace

cargo llvm-cov clean --workspace

cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info

cargo llvm-cov report --workspace

cargo bench || true

cargo metadata --format-version=1 >/dev/null

cargo tree --duplicates

cargo tree --edges features

cargo check --workspace --target x86_64-unknown-linux-gnu

cargo check --workspace --target aarch64-apple-darwin

cargo check --workspace --target x86_64-pc-windows-msvc

cargo check --workspace --target x86_64-unknown-linux-musl

cargo test --release --workspace

cargo clean

cargo build --workspace --release

git diff --exit-code

git diff --cached --exit-code

git fsck --full

git gc

find . \
  -path "./target" -prune -o \
  -path "./.git" -prune -o \
  -type f \
  \( \
    -name "*.rs" -o \
    -name "Cargo.toml" -o \
    -name "*.md" -o \
    -name "*.yml" -o \
    -name "*.yaml" -o \
    -name "*.json" \
  \) -print0 |
xargs -0 grep -nE 'TODO|FIXME|unwrap\(|expect\(|panic!\(|dbg!\(|println!\(|eprintln!\(' || true

echo
echo "========== DONE =========="
echo "If every command above passes with zero errors and zero warnings,"
echo "the codebase is in excellent shape for production release."
