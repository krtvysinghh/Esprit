fmt:
	cargo fmt --all

lint:
	cargo clippy --workspace -- -D warnings

test:
	cargo test --workspace

build:
	cargo build --workspace --release

check:
	cargo fmt --all -- --check
	cargo clippy --workspace -- -D warnings
	cargo test --workspace
	cargo build --workspace --release

run:
	cargo run -p esprit-cli

clean:
	cargo clean
