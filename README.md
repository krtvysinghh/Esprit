ESPRIT — COMPLETE PROJECT DOCUMENTATION
===========================================

PROJECT STATUS
--------------
Version: 0.1.0
License: MIT
Repository: https://github.com/krtvysinghh/Esprit

Esprit is a Rust-based AI workspace and operating layer for project indexing,
search, retrieval-augmented generation, AI agents, workflows, filesystem
analysis, and related developer tooling.

Esprit is currently an early-development Rust workspace. The CLI is the
primary usable interface. The desktop package, HTTP API, packaging targets,
and cross-platform distribution infrastructure exist at different stages
of development and should not be treated as finished end-user products.

TABLE OF CONTENTS
-----------------
1. What Esprit Does
2. Current Features
3. Current Limitations
4. Requirements
5. Installation
6. Build From Source
7. Running Esprit
8. CLI
9. Search and Indexing
10. AI and Ollama
11. RAG
12. Agents
13. Workflows
14. Filesystem Features
15. HTTP API
16. Desktop
17. Workspace Structure
18. Architecture
19. Environment Variables
20. Development
21. Formatting and Linting
22. Testing
23. Security and Dependency Auditing
24. Continuous Integration
25. Release Process
26. Contributing
27. Security Policy
28. Versioning
29. Documentation
30. Project Status
31. License
32. Changelog
33. Quick Start
34. Development Quick Reference


1. WHAT ESPRIT DOES
-------------------
Esprit is organized as a Cargo workspace containing reusable Rust crates
and applications.

The current implementation combines:

- Project and filesystem discovery
- File indexing
- Full-text search
- Tantivy-backed search indexing
- AI model integration through Ollama
- Retrieval-augmented generation
- Search-oriented AI agents
- Higher-level workflows
- Filesystem hashing and duplicate detection
- Platform diagnostics and filesystem watching
- HTTP API primitives
- Tauri desktop bootstrap
- Rust workspace tooling
- GitHub Actions CI

The architecture is split into multiple crates so functionality can evolve
without placing the entire implementation inside one binary.


2. CURRENT FEATURES
-------------------

PROJECT INDEXING
Relevant crate:
    crates/esprit-index/

The indexing layer includes:
- Content handling
- Database/index storage
- File indexing
- Query handling
- Search
- Schema management

FULL-TEXT SEARCH
Relevant crates:
    crates/esprit-index/
    crates/esprit-search/

The search layer supports:
- Building/rebuilding the search index
- Searching indexed project content
- Returning matching file paths
- Higher-level search interfaces

Inspect the current CLI interface with:
    cargo run -p esprit-cli -- --help

Search help:
    cargo run -p esprit-cli -- search --help


AI
Relevant crate:
    crates/esprit-ai/

Esprit integrates with Ollama for local AI model execution.

Default CLI model:
    qwen3:1.7b

The model can be changed with:
    export ESPRIT_MODEL=qwen3:1.7b

The Ollama endpoint can be changed with:
    export OLLAMA_URL=http://127.0.0.1:11434


RETRIEVAL-AUGMENTED GENERATION
Relevant crate:
    crates/esprit-rag/

The RAG layer combines project search results with AI processing.

Conceptually:

    Question
       |
       v
    Search
       |
       v
    Relevant project files
       |
       v
    RAG
       |
       v
    Ollama
       |
       v
    Answer


AGENTS
Relevant crate:
    crates/esprit-agents/

The current implementation includes a search-oriented agent:

    Agent::Search

The search agent uses the RAG layer to find information related to a
supplied prompt.


WORKFLOWS
Relevant crate:
    crates/esprit-workflows/

The workflow layer provides higher-level operations over search, RAG,
and agents.

Current workflow functionality includes project-search-oriented operations.


FILESYSTEM FEATURES
Relevant crate:
    crates/esprit-filesystem/

Current filesystem components include:
- File hashing
- Duplicate detection
- File organization
- Filesystem statistics

Relevant source files:
    crates/esprit-filesystem/src/duplicates.rs
    crates/esprit-filesystem/src/hash.rs
    crates/esprit-filesystem/src/organize.rs
    crates/esprit-filesystem/src/stats.rs


PLATFORM FEATURES
Relevant crate:
    crates/esprit-platform/

Current platform utilities include:
- Environment/installation diagnostics
- Ollama detection
- Platform checks
- Filesystem watching

The platform doctor checks whether tools such as Ollama are available.


3. CURRENT LIMITATIONS
----------------------
Esprit 0.1.0 is an early development release.

Important limitations:

- Esprit is not yet a finished desktop application.
- The current desktop entrypoint only performs a bootstrap action.
- The HTTP API router exists, but the desktop entrypoint does not currently
  start an HTTP server.
- GitHub Actions currently build and test the project but do not publish
  release binaries.
- No finished cross-platform installer/distribution system is generated
  by the release workflow.
- Windows is not currently covered by the GitHub Actions workflows.
- The existing workspace test suite contains many crates with zero unit/doc
  tests.
- Passing compilation does not mean every planned feature is production-ready.
- Package-manager installation through Homebrew, Scoop, AUR, or another
  package manager is not currently provided by the release workflow.

A crate or directory existing in the repository must not automatically be
interpreted as proof that the corresponding feature is production-complete.


4. REQUIREMENTS
---------------
All platforms require:
- Git
- Rust
- Cargo

Verify:
    rustc --version
    cargo --version

The repository contains:
    rust-toolchain.toml
    rustfmt.toml

These provide project toolchain and formatting configuration.


5. INSTALLATION
---------------

MACOS
-----
Install Rust:

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source "$HOME/.cargo/env"

Verify:

    rustc --version
    cargo --version

Clone and build:

    git clone https://github.com/krtvysinghh/Esprit.git
    cd Esprit
    cargo build --workspace

Release build:

    cargo build --workspace --release

For AI functionality, install Ollama separately and ensure it is running.


LINUX
-----
Install Rust:

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source "$HOME/.cargo/env"

Install the Linux development dependencies used by the current CI:

    sudo apt-get update
    sudo apt-get install -y \
      libwebkit2gtk-4.1-dev \
      build-essential \
      curl \
      wget \
      file \
      libxdo-dev \
      libssl-dev \
      libgtk-3-dev \
      libayatana-appindicator3-dev \
      librsvg2-dev \
      patchelf

Clone and build:

    git clone https://github.com/krtvysinghh/Esprit.git
    cd Esprit
    cargo build --workspace

Release build:

    cargo build --workspace --release


WINDOWS
-------
Install:
- Rust
- Cargo
- Git
- Microsoft C/C++ build tooling required by Rust dependencies

Verify:

    rustc --version
    cargo --version

Clone:

    git clone https://github.com/krtvysinghh/Esprit.git
    cd Esprit

Build:

    cargo build --workspace

Release build:

    cargo build --workspace --release

Important:
Windows is currently not covered by the repository's GitHub Actions CI.
Local Windows builds therefore provide the relevant validation until
dedicated Windows CI is added.


6. BUILD FROM SOURCE
--------------------
Debug build:

    cargo build --workspace

Release build:

    cargo build --workspace --release

Compilation check:

    cargo check --workspace

Build only the CLI:

    cargo build -p esprit-cli

Optimized CLI:

    cargo build -p esprit-cli --release


7. RUNNING ESPRIT
-----------------
The primary user-facing entry point is the CLI.

Show commands:

    cargo run -p esprit-cli -- --help

Show version:

    cargo run -p esprit-cli -- --version

After a release build:

    ./target/release/esprit --help

The exact command set should be obtained from the compiled CLI because
Esprit is under active development.


8. CLI
------
Binary:
    esprit

Source:
    apps/esprit-cli/src/main.rs

Package:
    esprit-cli

The CLI connects:
- esprit-core
- esprit-config
- esprit-platform
- esprit-search
- esprit-filesystem
- esprit-index
- esprit-ai
- esprit-rag
- esprit-agents
- esprit-workflows

Inspect the interface:

    cargo run -p esprit-cli -- --help

Inspect a command:

    cargo run -p esprit-cli -- <command> --help


9. SEARCH AND INDEXING
----------------------
The search system is backed by the Esprit index layer and Tantivy.

Relevant crates:

    crates/esprit-index/
    crates/esprit-search/

The internal index layer exposes:

    rebuild_search_index()
    search(query)

Inspect the CLI:

    cargo run -p esprit-cli -- --help

Search help:

    cargo run -p esprit-cli -- search --help

The search system operates on indexed project content and returns matching
file paths.


10. AI AND OLLAMA
-----------------
Esprit's AI implementation expects an Ollama service.

Start Ollama:

    ollama serve

In another terminal, pull the default model:

    ollama pull qwen3:1.7b

Set the model:

    export ESPRIT_MODEL=qwen3:1.7b

Set the Ollama endpoint if required:

    export OLLAMA_URL=http://127.0.0.1:11434

Verify Ollama:

    curl http://127.0.0.1:11434/api/tags

Then run:

    cargo run -p esprit-cli -- --help

If Ollama is unavailable, the AI layer reports that Ollama is not running.


11. RAG
-------
Relevant crate:
    crates/esprit-rag/

Current flow:

    Prompt
      |
      v
    Project Search
      |
      v
    Matching Files
      |
      v
    Context
      |
      v
    AI Model
      |
      v
    Response

RAG is currently an internal subsystem used by higher-level functionality.


12. AGENTS
---------
Relevant crate:
    crates/esprit-agents/

Current agent:
    Agent::Search

The search agent delegates project-related questions into the RAG layer.

The current implementation should not be described as a large autonomous
agent platform.


13. WORKFLOWS
------------
Relevant crate:
    crates/esprit-workflows/

Current workflow functionality includes project-search-oriented operations.

Relationship:

    CLI
      |
      v
    Workflow
      |
      v
    Agent
      |
      v
    RAG
      |
      v
    Search / Index


14. FILESYSTEM FEATURES
-----------------------
Relevant crate:
    crates/esprit-filesystem/

Implemented areas include:
- Duplicate detection
- Hashing
- File organization
- Filesystem statistics

Source:
    crates/esprit-filesystem/src/duplicates.rs
    crates/esprit-filesystem/src/hash.rs
    crates/esprit-filesystem/src/organize.rs
    crates/esprit-filesystem/src/stats.rs


15. HTTP API
------------
Relevant crate:
    crates/esprit-api/

Current router endpoints:

    GET  /health
    POST /ask

Health:

    /health
    -> "ok"

The /ask endpoint accepts an AI request and delegates to the RAG layer.

Important limitation:
The current desktop entrypoint does not start this router as a long-running
HTTP server.

Therefore, running the current desktop executable does NOT automatically
mean that an HTTP server is available.


16. DESKTOP
-----------
Directory:
    desktop/

Technology:
    Tauri 2

Current dependencies include:
- tauri
- tokio
- esprit-rag
- esprit-api

Current entrypoint behavior:

    Esprit Desktop bootstrap

The desktop application is therefore experimental/incomplete rather than
a finished cross-platform desktop product.


17. WORKSPACE STRUCTURE
-----------------------
The repository is a Cargo workspace.

    Esprit/
    ├── apps/
    │   └── esprit-cli/
    │
    ├── crates/
    │   ├── esprit-agents/
    │   ├── esprit-ai/
    │   ├── esprit-analysis/
    │   ├── esprit-api/
    │   ├── esprit-app/
    │   ├── esprit-bench/
    │   ├── esprit-cache/
    │   ├── esprit-codeintel/
    │   ├── esprit-config/
    │   ├── esprit-config2/
    │   ├── esprit-core/
    │   ├── esprit-core-index/
    │   ├── esprit-daemon/
    │   ├── esprit-diagnostics/
    │   ├── esprit-embeddings/
    │   ├── esprit-filesystem/
    │   ├── esprit-index/
    │   ├── esprit-jobs/
    │   ├── esprit-memory/
    │   ├── esprit-metrics/
    │   ├── esprit-package/
    │   ├── esprit-pipeline/
    │   ├── esprit-platform/
    │   ├── esprit-plugins/
    │   ├── esprit-production/
    │   ├── esprit-rag/
    │   ├── esprit-ranking/
    │   ├── esprit-search/
    │   ├── esprit-security/
    │   ├── esprit-semantic/
    │   ├── esprit-storage/
    │   ├── esprit-telemetry/
    │   ├── esprit-testing/
    │   ├── esprit-utils/
    │   ├── esprit-vectors/
    │   └── esprit-workflows/
    │
    ├── desktop/
    ├── docs/
    ├── packaging/
    ├── scripts/
    ├── tests/
    ├── benches/
    ├── Cargo.toml
    ├── Cargo.lock
    ├── rust-toolchain.toml
    ├── rustfmt.toml
    ├── deny.toml
    ├── LICENSE
    ├── SECURITY.md
    ├── CONTRIBUTING.md
    └── CHANGELOG.md


18. ARCHITECTURE
----------------
High-level relationship:

                         Esprit CLI
                             |
            +----------------+----------------+
            |                |                |
        Filesystem        Search             AI
            |                |                |
            |              Index            Ollama
            |                |
            |             Tantivy
            |                |
            |               RAG
            |                |
            |              Agents
            |                |
            |            Workflows
            |
        Filesystem
        utilities

Main relationship:

    CLI
     ├── Configuration
     ├── Platform
     ├── Filesystem
     ├── Search
     │    └── Index
     │         └── Tantivy
     ├── AI
     │    └── Ollama
     ├── RAG
     │    └── Search
     ├── Agents
     │    └── RAG
     └── Workflows
          └── Agents

Supporting crates also exist for storage, vectors, telemetry, security,
plugins, package management, jobs, pipelines, code intelligence, and
related infrastructure.

Repository architecture documentation:

    docs/ARCHITECTURE.md
    docs/architecture/


19. ENVIRONMENT VARIABLES
-------------------------

ESPRIT_MODEL
------------
Controls the model selected by Esprit.

Default:
    qwen3:1.7b

Example:

    export ESPRIT_MODEL=qwen3:1.7b


OLLAMA_URL
----------
Controls the Ollama service endpoint.

Example:

    export OLLAMA_URL=http://127.0.0.1:11434


WORKSPACE
---------
The configuration layer derives the current workspace from the process
working directory.


20. DEVELOPMENT
---------------
Clone:

    git clone https://github.com/krtvysinghh/Esprit.git
    cd Esprit

Check:

    cargo check --workspace

Build:

    cargo build --workspace

Run CLI:

    cargo run -p esprit-cli -- --help


21. FORMATTING AND LINTING
--------------------------
Check formatting:

    cargo fmt --all -- --check

Apply formatting:

    cargo fmt --all

Run Clippy:

    cargo clippy --workspace -- -D warnings

Both formatting and Clippy are required by CI.


22. TESTING
-----------
Run all workspace tests:

    cargo test --workspace

Run tests with output:

    cargo test --workspace -- --nocapture

Run one package:

    cargo test -p <package-name>

Important:
The current repository contains many crates whose unit/doc test suites
report zero tests. A successful test command therefore means the current
test suites completed successfully; it does not imply comprehensive
behavioral coverage.


23. SECURITY AND DEPENDENCY AUDITING
------------------------------------
Dependency policy is configured in:

    deny.toml

The dependency audit covers:
- Rust security advisories
- Dependency bans
- License policy
- Dependency sources

When cargo-deny is installed:

    cargo deny check

The dependency audit should pass before release.


24. CONTINUOUS INTEGRATION
--------------------------
Current workflows:

    .github/workflows/ci.yml
    .github/workflows/full-ci.yml
    .github/workflows/release.yml


CI WORKFLOW
-----------
Triggers:
    push
    pull_request

Runner:
    ubuntu-latest

Linux dependencies:

    libwebkit2gtk-4.1-dev
    build-essential
    curl
    wget
    file
    libxdo-dev
    libssl-dev
    libgtk-3-dev
    libayatana-appindicator3-dev
    librsvg2-dev
    patchelf

Checks:

    cargo fmt --all -- --check
    cargo clippy --workspace -- -D warnings
    cargo test --workspace
    cargo build --workspace --release


FULL CI WORKFLOW
----------------
Triggers:
    push
    pull_request

Runner:
    ubuntu-latest

Installs the same Linux dependencies.

Checks:

    cargo fmt --all -- --check
    cargo clippy --workspace -- -D warnings
    cargo test --workspace
    cargo build --workspace --release


RELEASE WORKFLOW
----------------
Triggers:
    workflow_dispatch
    push of tags matching v*

Runner:
    ubuntu-latest

Installs the same Linux dependencies.

Runs:

    cargo build --workspace --release
    cargo test --workspace

Important:
The current release workflow validates and builds the project. It does not
publish compiled binaries or installers.


25. RELEASE PROCESS
-------------------
Before releasing:

    cargo fmt --all -- --check
    cargo clippy --workspace -- -D warnings
    cargo test --workspace
    cargo build --workspace --release
    git diff --check

Check the repository:

    git status --short

Review:

    CHANGELOG.md

Create a version tag:

    git tag v0.1.0

Push:

    git push origin v0.1.0

The GitHub Actions release workflow is then triggered.

Replace v0.1.0 with the intended future version.


26. RELEASE CHECKLIST
---------------------
Before tagging:

- Code compiles
- Formatting passes
- Clippy passes with warnings denied
- Workspace tests pass
- Release build passes
- Dependency/security audit passes
- Documentation reflects actual behavior
- Changelog is updated
- Git working tree is clean
- Version is correct
- CI is expected to pass
- Release limitations are understood


27. CONTRIBUTING
----------------
Contributions are welcome.

Create a branch:

    git checkout -b feature/my-change

Run the quality checks:

    cargo fmt --all -- --check
    cargo clippy --workspace -- -D warnings
    cargo test --workspace
    cargo build --workspace --release
    git diff --check

Review:

    git status
    git diff

Commit:

    git add .
    git commit -m "feat: describe the change"

Push:

    git push origin feature/my-change

Then open a Pull Request.

Pull Requests should:
- Explain what changed
- Explain why it changed
- Avoid unrelated modifications
- Include tests when practical
- Keep documentation synchronized with behavior
- Pass formatting
- Pass Clippy
- Pass workspace tests
- Pass the release build
- Avoid unnecessary dependencies

Related files:

    CONTRIBUTING.md
    docs/CONTRIBUTING.md


28. SECURITY POLICY
-------------------
Security issues should not be disclosed through public issue reports.

See:

    SECURITY.md

The current supported development line is:

    0.1.x

Security reports should be sent privately using the contact information
specified in SECURITY.md.

The repository's security policy aims to acknowledge reports within the
stated response period.


29. VERSIONING
--------------
Esprit currently uses semantic-style versions such as:

    0.1.0

The 0.x version indicates that the project is still under active development
and APIs may change.

Release tags follow:

    v<version>

Example:

    v0.1.0


30. DOCUMENTATION
-----------------
Main documentation directory:

    docs/

Important documentation:

    docs/ARCHITECTURE.md
    docs/GETTING_STARTED.md
    docs/PROJECT_STRUCTURE.md
    docs/ROADMAP.md
    docs/CONTRIBUTING.md

Additional architecture documentation:

    docs/architecture/

API documentation:

    docs/api/


31. PROJECT STATUS
------------------

AVAILABLE
---------
- Rust Cargo workspace
- Esprit CLI
- Project indexing infrastructure
- Tantivy-backed search
- Filesystem utilities
- AI integration through Ollama
- RAG implementation
- Search-oriented agent
- Workflow layer
- Platform diagnostics
- HTTP API router
- Tauri desktop package
- GitHub Actions CI
- Dependency/security configuration


IN DEVELOPMENT
--------------
- Desktop application
- HTTP server integration
- Cross-platform distribution
- Installer/package publishing
- Expanded automated tests
- Broader production functionality


NOT CURRENTLY PROVIDED BY THE RELEASE WORKFLOW
----------------------------------------------
- Published macOS application
- Published Windows executable/installer
- Published Linux binary/installer
- Automatic Homebrew release
- Automatic Scoop release
- Automatic AUR release
- Automatic package-manager publishing


32. LICENSE
-----------
Esprit is licensed under the MIT License.

Copyright (c) 2026 Kartavya Singh.

See:

    LICENSE

for the complete license text.


33. CHANGELOG
-------------
Release history is maintained in:

    CHANGELOG.md

Current release:

    0.1.0

Current documented 0.1.0 history includes:
- Initial development release of the Esprit workspace
- Base infrastructure for HTTP API and background daemon
- Early implementations of semantic search, RAG, and AI agents


34. QUICK START
---------------
For a developer who already has Rust installed:

    git clone https://github.com/krtvysinghh/Esprit.git
    cd Esprit
    cargo build --workspace
    cargo run -p esprit-cli -- --help

For AI functionality:

    ollama serve

In another terminal:

    ollama pull qwen3:1.7b

Then:

    cd Esprit
    export ESPRIT_MODEL=qwen3:1.7b
    cargo run -p esprit-cli -- --help


35. DEVELOPMENT QUICK REFERENCE
-------------------------------
Build:
    cargo build --workspace

Release build:
    cargo build --workspace --release

Check:
    cargo check --workspace

Format:
    cargo fmt --all

Format verification:
    cargo fmt --all -- --check

Clippy:
    cargo clippy --workspace -- -D warnings

Tests:
    cargo test --workspace

CLI help:
    cargo run -p esprit-cli -- --help

Search help:
    cargo run -p esprit-cli -- search --help

Dependency audit:
    cargo deny check

Patch validation:
    git diff --check


FINAL NOTE
----------
Esprit is an actively developed 0.1.x project. This documentation
intentionally distinguishes between functionality that exists in the
source tree and functionality that is already a finished end-user product.

If a feature, command, installer, API server, platform target, or
distribution mechanism is not explicitly implemented by the current
source and workflow configuration, it should not be assumed to be
available.
