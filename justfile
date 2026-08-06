build:
    cargo build --workspace

check:
    cargo check --workspace

test:
    cargo test --workspace

fmt:
    cargo fmt --all

lint:
    cargo clippy --workspace --all-targets -- -D warnings

run:
    cargo run -p esprit-cli --

doctor:
    cargo run -p esprit-cli -- doctor
