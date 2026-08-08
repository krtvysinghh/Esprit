# Esprit

Esprit is an open-source Rust workspace for building a local, extensible system around filesystem access, indexing, search, semantic retrieval, AI capabilities, background processing, and a desktop application.

> **Status:** `0.1.0` — initial development release.

Esprit is currently under active development. Some components are foundational or experimental, so APIs, crate boundaries, configuration, and behavior may evolve between releases.

## Contents

- [Overview](#overview)
- [Features](#features)
- [Workspace](#workspace)
- [Architecture](#architecture)
- [Requirements](#requirements)
- [Installation](#installation)
  - [macOS](#macos)
  - [Linux](#linux)
  - [Windows](#windows)
- [Building](#building)
- [Running](#running)
- [Configuration](#configuration)
- [Development](#development)
- [Testing](#testing)
- [Code Quality](#code-quality)
- [CI](#ci)
- [Release Process](#release-process)
- [Project Documentation](#project-documentation)
- [Contributing](#contributing)
- [Security](#security)
- [License](#license)

---

## Overview

Esprit is organized as a Rust workspace rather than a single monolithic binary.

The workspace separates functionality into focused crates covering areas such as:

- core application functionality
- filesystem operations
- indexing
- search
- semantic processing
- embeddings and vectors
- retrieval-augmented generation
- AI integration
- agents
- memory
- storage
- caching
- configuration
- plugins
- workflows
- jobs
- platform integration
- security
- telemetry
- packaging
- code intelligence
- application and daemon infrastructure

The repository also contains:

- a command-line application
- a desktop application
- shared Rust libraries
- integration and end-to-end tests
- benchmarks
- GitHub Actions workflows
- packaging configuration
- development and release tooling

The workspace is intended to keep these concerns independently testable while allowing them to be composed into a larger system.

---

## Features

The `0.1.0` workspace contains infrastructure and implementations for several major areas.

### Core system

- Shared core application infrastructure
- Workspace-level Rust architecture
- Configuration management
- Platform abstractions
- Utility functionality

### Filesystem

- Filesystem-related functionality
- File indexing infrastructure
- Filesystem statistics
- Content processing

### Search and retrieval

- Traditional indexing/search infrastructure
- Semantic search components
- Vector-related infrastructure
- Embedding support
- Retrieval-augmented generation infrastructure

### AI

- Configurable AI integration
- AI agents
- Model and endpoint configuration
- AI-related error/context handling

### Background processing

- Daemon infrastructure
- Jobs
- Pipelines
- Workflows
- Caching
- Memory-related infrastructure

### Applications

- Esprit CLI
- Desktop application

### Engineering infrastructure

- Rust workspace organization
- Formatting checks
- Clippy checks
- Workspace tests
- Release builds
- GitHub Actions CI
- Security analysis through CodeQL
- Linux native dependency installation for GUI-related builds

> Esprit `0.1.0` should be considered an early development release. The existence of a crate does not necessarily mean that every planned capability is production-ready.

---

# Workspace

The repository is a Cargo workspace.

```text
Esprit/
├── apps/
│   └── esprit-cli/
│
├── crates/
│   ├── esprit-agents/
│   ├── esprit-ai/
│   ├── esprit-api/
│   ├── esprit-app/
│   ├── esprit-cache/
│   ├── esprit-codeintel/
│   ├── esprit-config/
│   ├── esprit-core/
│   ├── esprit-core-index/
│   ├── esprit-daemon/
│   ├── esprit-embeddings/
│   ├── esprit-filesystem/
│   ├── esprit-index/
│   ├── esprit-jobs/
│   ├── esprit-memory/
│   ├── esprit-package/
│   ├── esprit-pipeline/
│   ├── esprit-platform/
│   ├── esprit-plugins/
│   ├── esprit-production/
│   ├── esprit-rag/
│   ├── esprit-search/
│   ├── esprit-security/
│   ├── esprit-storage/
│   ├── esprit-telemetry/
│   ├── esprit-utils/
│   ├── esprit-vectors/
│   └── esprit-workflows/
│
├── desktop/
│   └── src/
│
├── benches/
├── tests/
├── packaging/
├── scripts/
├── .github/
├── Cargo.toml
├── Cargo.lock
├── rust-toolchain.toml
├── rustfmt.toml
├── clippy.toml
├── deny.toml
├── LICENSE
└── README.md
