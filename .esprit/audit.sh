#!/usr/bin/env bash
set -e

echo "===== GIT ====="
git branch --show-current
git status --short
git log --oneline --decorate -5

echo
echo "===== WORKSPACE ====="
find . -name Cargo.toml | sort

echo
echo "===== TREE ====="
tree -L 3

echo
echo "===== BUILD ====="
cargo build --workspace

echo
echo "===== CLIPPY ====="
cargo clippy --workspace --all-targets -- -D warnings || true

echo
echo "===== TEST ====="
cargo test --workspace || true
