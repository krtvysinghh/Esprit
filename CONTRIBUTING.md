# Contributing to Esprit

Thank you for your interest in contributing to **Esprit**! We welcome contributions from developers across all backgrounds and skill levels.

---

## 1. Code of Conduct

All contributors and maintainers are expected to adhere to our [Code of Conduct](CODE_OF_CONDUCT.md). Please report unacceptable behavior to `kartxvyaa@gmail.com`.

---

## 2. Development Prerequisites

To build and test Esprit locally, ensure you have:
* **Rust & Cargo**: Stable toolchain (MSRV: `1.75.0+`).
* **C/C++ Compiler**: `clang` / `gcc` / MSVC (required for embedded `llama.cpp` Metal/C++ compilation).
* **Git**: `2.30+`.
* **Platform Dependencies**:
  * **macOS**: Xcode Command Line Tools (`xcode-select --install`).
  * **Linux (Ubuntu/Debian)**: `sudo apt install build-essential pkg-config libssl-dev`.
  * **Windows**: Visual Studio 2022 C++ Build Tools.

---

## 3. Workspace Setup & Build

```bash
# Clone repository
git clone https://github.com/krtvysinghh/Esprit.git
cd Esprit

# Build entire Cargo workspace
cargo build --workspace

# Run CLI locally
cargo run -p esprit-cli -- doctor
```

---

## 4. Development Workflow & Quality Standards

Before submitting a Pull Request, ensure your code satisfies all quality gates:

### A. Formatting & Lints
```bash
# Check formatting
cargo fmt --all -- --check

# Run Clippy with strict warnings
cargo clippy --workspace --all-targets -- -D warnings
```

### B. Automated Testing
```bash
# Run all unit and integration tests
cargo test --workspace --all-targets
```

### C. Dependency & Security Auditing
```bash
# Check licensing and security advisories
cargo deny check
```

---

## 5. Branching & Commit Conventions

* **Branch Naming**:
  * `feat/feature-name` for new capabilities.
  * `fix/bug-description` for bug fixes.
  * `docs/documentation-topic` for documentation updates.
  * `perf/optimization-area` for performance enhancements.
* **Commit Messages**: Follow Conventional Commits:
  * `feat(rag): add hierarchical chunking support`
  * `fix(cli): resolve relative path resolution on Windows`
  * `docs(readme): add one-line installer guide`

---

## 6. Pull Request Process

1. Fork the repository and create your branch from `main`.
2. Add comprehensive tests for any new features or bug fixes.
3. Ensure all CI checks pass locally (`cargo fmt`, `cargo clippy`, `cargo test`).
4. Submit a Pull Request targeting `main` with a clear description of changes, motivation, and verification steps.

Thank you for helping build the future of local-first workspace intelligence!
