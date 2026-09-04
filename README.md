<div align="center">

```text
  ███████╗███████╗██████╗ ██████╗ ██╗████████╗
  ██╔════╝██╔════╝██╔══██╗██╔══██╗██║╚══██╔══╝
  █████╗  ███████╗██████╔╝██████╔╝██║   ██║   
  ██╔══╝  ╚════██║██╔═══╝ ██╔══██╗██║   ██║   
  ███████╗███████║██║     ██║  ██║██║   ██║   
  ╚══════╝╚══════╝╚═╝     ╚═╝  ╚═╝╚═╝   ╚═╝   
```

# Esprit

### Local-First AI Workspace Intelligence & Operating Layer

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](LICENSE)
[![Rust: MSRV 1.75+](https://img.shields.io/badge/Rust-1.75%2B-orange.svg?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![Apple Silicon Metal](https://img.shields.io/badge/Metal-GPU_Accelerated-black.svg?style=flat-square&logo=apple)](https://developer.apple.com/metal/)
[![Cross-Platform](https://img.shields.io/badge/Platform-macOS%20%7C%20Linux%20%7C%20Windows-blueviolet.svg?style=flat-square)](#installation--quick-start)
[![Zero Telemetry](https://img.shields.io/badge/Privacy-100%25_Air--Gapped-brightgreen.svg?style=flat-square)](#security--privacy-contract)

**Esprit** is an air-gapped, local-first AI workspace and developer operating layer built in Rust. It combines high-throughput project indexing, Tantivy full-text search, dense vector retrieval (RAG), and embedded GPU-accelerated LLMs into an intuitive command-line interface and daemon.

[Architecture](ARCHITECTURE.md) • [Getting Started](#installation--quick-start) • [Command Guide](#command-reference) • [Contributing](CONTRIBUTING.md) • [Security](SECURITY.md)

</div>

---

## ⚡ Highlights

* 🧠 **Embedded GPU-Accelerated Local AI**: Native embedded `llama.cpp` engine with Apple Silicon Metal acceleration (`MTL0 / Apple M2/M3/M4`). Zero mandatory external cloud dependencies.
* ⚡ **Zero-Config Universal Setup**: One-line install bootstraps the binary and automatically provisions default local models (`Qwen` LLMs & `Nomic Embed`).
* 🔎 **High-Throughput Project Indexing & Search**: Inverted full-text index powered by Tantivy with SQLite metadata persistence.
* 🤖 **macOS Omni-Agent (`esprit os`)**: Natural-language operating system assistant for automation, file discovery, and developer operations.
* 📚 **Offline Codebase RAG (`esprit ask`)**: Semantic search and contextual retrieval-augmented generation over indexed repositories.
* 🛡️ **100% Privacy & Zero Telemetry**: Operates completely offline. Your code and prompts never leave your hardware.

---

## 📦 Installation & Quick Start

### Option 1: Universal One-Line Installer (Recommended)

#### macOS & Linux:
```bash
curl -fsSL https://raw.githubusercontent.com/krtvysinghh/Esprit/main/install.sh | bash
```

#### Windows (PowerShell):
```powershell
irm https://raw.githubusercontent.com/krtvysinghh/Esprit/main/install.ps1 | iex
```

> **Note**: The universal installer automatically:
> 1. Detects your OS/architecture and installs the `esprit` binary to your PATH.
> 2. Provisions default local AI models (`qwen3:0.6b`, `qwen3:1.7b`, `nomic-embed`).
> 3. Runs `esprit doctor` to verify GPU acceleration and system health.

---

### Option 2: Build From Source (Cargo)

```bash
# Clone repository
git clone https://github.com/krtvysinghh/Esprit.git
cd Esprit

# Build & install binary to ~/.cargo/bin
cargo install --path apps/esprit-cli --bin esprit

# Initialize default models
esprit init
```

---

## 🔄 Updating Esprit

To update Esprit and its local model weights to the latest release:

```bash
# Re-run the universal installer (downloads latest binary & verifies models)
curl -fsSL https://raw.githubusercontent.com/krtvysinghh/Esprit/main/install.sh | bash

# Or update directly via Esprit CLI
esprit update
```

To update or pull specific local AI models:
```bash
# Pull balanced 1.7B model
esprit model pull qwen3:1.7b

# Check all installed models and system health
esprit doctor
```

---

## 🛠️ Command Reference

| Subcommand | Usage | Description |
| :--- | :--- | :--- |
| **`esprit doctor`** | `esprit doctor` | Check hardware, Metal GPU status, toolchains, and model availability. |
| **`esprit os`** | `esprit os "<intent>"` | Natural language macOS Omni-Agent assistant for developer and system tasks. |
| **`esprit ask`** | `esprit ask "<question>"` | Query your indexed codebase using local RAG and semantic search. |
| **`esprit index`** | `esprit index <folder>` | Index a directory for high-speed full-text and semantic search. |
| **`esprit search`** | `esprit search "<query>"` | Fast keyword or regex search across indexed projects. |
| **`esprit watch`** | `esprit watch <folder>` | Watch a directory in the background and keep the index up to date. |
| **`esprit duplicates`** | `esprit duplicates <dir>` | Find identical files by SHA-256 content hashing. |
| **`esprit stats`** | `esprit stats <dir>` | Display filesystem statistics and breakdown for a folder. |
| **`esprit model`** | `esprit model list` / `pull` | Manage local GGUF models on disk. |
| **`esprit init`** | `esprit init` | Download default model bundle for offline first-time use. |

---

## 🏗️ System Architecture

```
Esprit Workspace
│
├── apps/
│   └── esprit-cli          # Main CLI entrypoint & terminal UI
│
├── crates/
│   ├── esprit-ai           # Local inference engine (Metal llama.cpp + Ollama)
│   ├── esprit-agents       # Autonomous developer agents (Chat, Review, Search)
│   ├── esprit-rag          # Semantic retrieval-augmented generation pipeline
│   ├── esprit-search       # Tantivy full-text search engine
│   ├── esprit-index        # Codebase parser & SQLite metadata store
│   ├── esprit-vectors      # HNSW vector similarity engine
│   ├── esprit-daemon       # Filesystem watcher daemon with crash recovery
│   ├── esprit-filesystem   # Multi-threaded file intelligence & SHA-256 hashing
│   ├── esprit-platform     # Hardware diagnostics, GPU detection, toolchain health
│   └── esprit-config       # TOML configuration engine
```

For complete technical specifications, see [ARCHITECTURE.md](ARCHITECTURE.md).

---

## 🔒 Security & Privacy Contract

1. **Air-Gapped by Design**: All inference, indexing, and calculations run on your local hardware.
2. **Zero Telemetry**: No tracking, metrics, or telemetry are transmitted to third-party endpoints.
3. **Safe Execution**: Omni-Agent mutating operations require explicit confirmation.

For details or vulnerability reporting, see [SECURITY.md](SECURITY.md).

---

## 🤝 Contributing & Community

Contributions are welcome! Please review our:
* [Contributing Guide](CONTRIBUTING.md) for build setup and PR standards.
* [Code of Conduct](CODE_OF_CONDUCT.md) for community guidelines.

---

## 📄 License

Esprit is open-source software licensed under the **MIT License**. See [LICENSE](LICENSE) for details.

**Author**: Kartavya Singh ([@krtvysinghh](https://github.com/krtvysinghh)) — `kartxvyaa@gmail.com`
