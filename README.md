
ESPRIT
======

Local-first AI Workspace Intelligence Platform

Version: 0.1.0
License: MIT
Repository:
https://github.com/krtvysinghh/Esprit


1. PROJECT OVERVIEW
===================

Esprit is a Rust-based workspace intelligence platform designed to combine
project indexing, search, AI assistance, retrieval-augmented generation,
agents, workflows, and filesystem intelligence.

The project is built as a Cargo workspace containing independent Rust crates.

Main goals:

- Understand software projects locally.
- Search large codebases efficiently.
- Provide AI assistance using local models.
- Keep user data under user control.
- Provide reusable developer infrastructure.


2. CURRENT STATUS
=================

Current release:
0.1.0

Completed development areas:

✓ Rust workspace architecture
✓ Project indexing
✓ Tantivy-based search
✓ Database-backed file tracking
✓ Crash-safe index rebuilding
✓ Daemon watcher recovery
✓ Filesystem analysis
✓ Duplicate detection
✓ Storage hardening
✓ AI integration
✓ RAG pipeline
✓ Agent system
✓ Workflow layer
✓ API foundation
✓ Production startup improvements
✓ CI validation


Experimental / under development:

- Desktop application
- Cross-platform installers
- Distribution packages
- Advanced autonomous agents
- Enterprise deployment


3. FEATURES
===========

PROJECT INDEXING
----------------

Package:

crates/esprit-index

Capabilities:

- File discovery
- Metadata tracking
- Database storage
- Workspace isolation
- Index rebuilding
- Recovery handling


SEARCH
------

Packages:

crates/esprit-index
crates/esprit-search

Features:

- Full-text search
- Tantivy search backend
- Ranked results
- Workspace-based search
- Deterministic results


AI INTEGRATION
--------------

Package:

crates/esprit-ai

Backend:

Ollama local AI models

Features:

- Local model communication
- Prompt generation
- AI health checking
- Configurable models


RAG
---

Package:

crates/esprit-rag

Pipeline:

Question
   |
Search
   |
Relevant files
   |
Context creation
   |
AI response


AGENTS
------

Package:

crates/esprit-agents

Current agents:

- Chat
- Code
- Search


WORKFLOWS
---------

Package:

crates/esprit-workflows

Provides:

- Higher-level AI operations
- Agent execution flows
- Project search workflows


FILESYSTEM INTELLIGENCE
-----------------------

Package:

crates/esprit-filesystem

Features:

- SHA256 file hashing
- Duplicate detection
- Folder statistics
- File analysis


DAEMON
------

Package:

crates/esprit-daemon

Features:

- Background watching
- Root disappearance detection
- Automatic recovery
- Clean shutdown handling


API
---

Package:

crates/esprit-api

Current endpoints:

GET /health

POST /ask


DESKTOP
-------

Technology:

Tauri

Status:

Bootstrap infrastructure exists.
Full end-user desktop experience is still under development.


4. ARCHITECTURE
===============

Workspace:

Esprit
|
├── apps/
│   └── esprit-cli
|
├── crates/
│
├── esprit-ai
│   AI model integration
│
├── esprit-agents
│   Agent execution
│
├── esprit-rag
│   Retrieval augmented generation
│
├── esprit-index
│   Indexing and search database
│
├── esprit-filesystem
│   File intelligence
│
├── esprit-storage
│   Persistent storage
│
├── esprit-security
│   Security utilities
│
├── esprit-api
│   HTTP API
│
├── esprit-app
│   Application layer
│
└── esprit-production
    Production utilities


5. INSTALLATION
===============


MACOS
-----

Install Rust:

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

Reload:

source ~/.cargo/env


Clone:

git clone https://github.com/krtvysinghh/Esprit.git

cd Esprit


Build:

cargo build --workspace


Run tests:

cargo test --workspace


Release build:

cargo build --workspace --release



LINUX
-----

Install dependencies:

sudo apt update

sudo apt install \
git \
curl \
build-essential \
pkg-config \
libssl-dev


Install Rust:

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh


Clone:

git clone https://github.com/krtvysinghh/Esprit.git

cd Esprit


Build:

cargo build --workspace



WINDOWS
-------

Requirements:

- Windows 10/11
- Git
- Rust
- Visual Studio C++ Build Tools


Clone:

git clone https://github.com/krtvysinghh/Esprit.git

cd Esprit


Build:

cargo build --workspace


6. AI SETUP
===========

Install Ollama.

Start:

ollama serve


Download model:

ollama pull qwen3:1.7b


Environment:

export ESPRIT_MODEL=qwen3:1.7b

export OLLAMA_URL=http://127.0.0.1:11434


Check:

curl http://127.0.0.1:11434/api/tags


7. USING ESPRIT
===============


Build:

cargo build --workspace


Run CLI:

cargo run -p esprit-cli -- --help


Release binary:

./target/release/esprit


Search help:

cargo run -p esprit-cli -- search --help


8. DEVELOPMENT COMMANDS
=======================


Format:

cargo fmt --all


Check formatting:

cargo fmt --all -- --check


Test:

cargo test --workspace


Clippy:

cargo clippy --workspace -- -D warnings


Release validation:

cargo build --workspace --release


Check changes:

git diff --check


9. TESTING
==========

Current tests cover:

- Search reliability
- Index recovery
- Database safety
- Workspace isolation
- Rapid updates
- Large indexing
- Filesystem operations
- Daemon recovery


Run:

cargo test --workspace --all-targets


10. SECURITY
============

Principles:

- Local-first processing
- User-controlled data
- Minimal external dependency
- Safe storage handling
- Dependency auditing


Audit:

cargo deny check


11. CONTRIBUTING
================


Create branch:

git checkout -b feature-name


Before commit:

cargo fmt --all

cargo test --workspace

cargo clippy --workspace -- -D warnings


Commit:

git add .

git commit -m "describe change"


Push:

git push origin feature-name


12. ROADMAP
===========

Future improvements:

- Complete desktop application
- Plugin ecosystem
- Better code intelligence
- More AI agents
- Cross-platform installers
- Cloud/enterprise options
- Advanced automation


13. LICENSE
===========

Esprit is licensed under the MIT License.
