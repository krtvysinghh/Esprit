# Getting Started

## Build
cargo build --workspace

## Run
cargo run -p esprit-cli -- doctor

## Search
cargo run -p esprit-cli -- search TODO

## Index
cargo run -p esprit-cli -- index .

## Ask
cargo run -p esprit-cli -- ask "Explain the indexing engine"

## Agent
cargo run -p esprit-cli -- agent code "Explain rebuild_search_index"

## Workflow
cargo run -p esprit-cli -- workflow explain "How does indexing work?"
