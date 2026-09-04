# Security Policy

The **Esprit** project takes the security, data sovereignty, and privacy of local AI and filesystem intelligence very seriously.

---

## 1. Supported Versions

We provide security updates and patches for the following versions:

| Version | Supported          |
| :------ | :----------------- |
| `0.1.x` | :white_check_mark: |
| `< 0.1` | :x:                |

---

## 2. Core Security & Privacy Guarantees

* **100% Local-First & Air-Gapped**: By default, Esprit runs completely offline using local embedded inference engines (`llama.cpp` Metal backend) or local Ollama instances. No code, filesystem data, or telemetry is transmitted to external servers without explicit user invocation.
* **Non-Destructive Operations**: Mutating file operations require explicit confirmation or `--force` parameters.
* **Memory & Storage Encryption**: Sensitive configuration and local index vectors are stored strictly in user application data directories with restricted file permissions (`0700` / `0600`).
* **Dependency Auditing**: All dependencies in the Cargo workspace are scanned with `cargo deny` and `cargo audit` in CI pipelines to prevent supply chain vulnerabilities.

---

## 3. Reporting a Vulnerability

If you discover a security vulnerability within Esprit:

1. **Do not disclose it publicly** via GitHub Issues, Discussions, or social media.
2. Send an email with detailed reproduction steps to:
   **`kartxvyaa@gmail.com`**
3. Include:
   * A description of the vulnerability and its potential impact.
   * Steps to reproduce the issue (CLI commands, sample code, or configuration).
   * Affected operating systems and architectures (e.g., macOS Apple Silicon, Linux x86_64, Windows).
   * Any potential remediations or mitigation suggestions if available.

### Response Timeline
* **Initial Acknowledgement**: Within 48 hours.
* **Triage & Reproduction**: Within 5 business days.
* **Fix & Coordinated Release**: Distributed via GitHub releases with appropriate CVE credit and security advisory notes.

Thank you for helping keep the open-source community secure!
