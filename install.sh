#!/usr/bin/env bash
# ==============================================================================
# Esprit — Universal Cross-Platform Installer & Model Bootstrapper
# ==============================================================================
# Installs the latest Esprit binary and automatically provisions default local
# AI models (Qwen LLM + Nomic Embed) so that Esprit works immediately out-of-the-box.
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/krtvysinghh/Esprit/main/install.sh | bash
# ==============================================================================

set -euo pipefail

REPO="krtvysinghh/Esprit"
BIN_NAME="esprit"

# ANSI Colors
BOLD="\033[1m"
GREEN="\033[0;32m"
BLUE="\033[0;34m"
YELLOW="\033[0;33m"
CYAN="\033[0;36m"
RED="\033[0;31m"
RESET="\033[0m"

echo -e "${CYAN}${BOLD}"
cat << "BANNER"
  ███████╗███████╗██████╗ ██████╗ ██╗████████╗
  ██╔════╝██╔════╝██╔══██╗██╔══██╗██║╚══██╔══╝
  █████╗  ███████╗██████╔╝██████╔╝██║   ██║   
  ██╔══╝  ╚════██║██╔═══╝ ██╔══██╗██║   ██║   
  ███████╗███████║██║     ██║  ██║██║   ██║   
  ╚══════╝╚══════╝╚═╝     ╚═╝  ╚═╝╚═╝   ╚═╝   
BANNER
echo -e "${RESET}"
echo -e "${BOLD}Esprit Universal Installer & Model Bootstrapper${RESET}\n"

# 1. Detect OS and Architecture
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

case "$OS" in
  darwin)
    TARGET_OS="apple-darwin"
    case "$ARCH" in
      arm64|aarch64) TARGET_ARCH="aarch64" ;;
      x86_64) TARGET_ARCH="x86_64" ;;
      *) echo -e "${RED}Unsupported macOS architecture: $ARCH${RESET}"; exit 1 ;;
    esac
    MODELS_DIR="$HOME/Library/Application Support/dev.esprit.esprit/models"
    ;;
  linux)
    TARGET_OS="unknown-linux-gnu"
    case "$ARCH" in
      x86_64) TARGET_ARCH="x86_64" ;;
      aarch64|arm64) TARGET_ARCH="aarch64" ;;
      *) echo -e "${RED}Unsupported Linux architecture: $ARCH${RESET}"; exit 1 ;;
    esac
    MODELS_DIR="$HOME/.local/share/esprit/models"
    ;;
  *)
    echo -e "${RED}Unsupported operating system: $OS. For Windows, please run install.ps1.${RESET}"
    exit 1
    ;;
esac

TARGET="${TARGET_ARCH}-${TARGET_OS}"
echo -e "▸ Detected Platform: ${GREEN}${TARGET}${RESET}"

# 2. Determine Install Destination Directory
if [[ -w "/usr/local/bin" ]]; then
  INSTALL_DIR="/usr/local/bin"
elif [[ -d "$HOME/.cargo/bin" ]]; then
  INSTALL_DIR="$HOME/.cargo/bin"
else
  INSTALL_DIR="$HOME/.local/bin"
  mkdir -p "$INSTALL_DIR"
fi

echo -e "▸ Install Destination: ${GREEN}${INSTALL_DIR}/${BIN_NAME}${RESET}"

# 3. Download or Verify Binary
echo -e "\n${BLUE}▸ [1/3] Installing Esprit CLI Binary...${RESET}"
RELEASE_URL="https://github.com/${REPO}/releases/latest/download/esprit-${TARGET}.tar.gz"

TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

if curl -fsSL "$RELEASE_URL" -o "$TMP_DIR/esprit.tar.gz" 2>/dev/null; then
  tar -xzf "$TMP_DIR/esprit.tar.gz" -C "$TMP_DIR"
  cp "$TMP_DIR/$BIN_NAME" "$INSTALL_DIR/$BIN_NAME"
  chmod +x "$INSTALL_DIR/$BIN_NAME"
  echo -e "  ${GREEN}✓ Installed ${BIN_NAME} to ${INSTALL_DIR}/${BIN_NAME}${RESET}"
elif command -v cargo >/dev/null 2>&1; then
  echo -e "  ${YELLOW}Release asset not yet available for ${TARGET}; building via cargo...${RESET}"
  cargo install --git "https://github.com/${REPO}" esprit-cli --bin esprit --root "$HOME/.cargo"
  echo -e "  ${GREEN}✓ Built and installed via Cargo${RESET}"
else
  echo -e "  ${YELLOW}Note: Pre-compiled binary release will be fetched upon official release.${RESET}"
fi

# 4. Provision Default AI Models
echo -e "\n${BLUE}▸ [2/3] Provisioning Default Local AI Models...${RESET}"
mkdir -p "$MODELS_DIR"

download_model() {
  local filename="$1"
  local url="$2"
  local target_path="$MODELS_DIR/$filename"

  if [[ -f "$target_path" && -s "$target_path" ]]; then
    echo -e "  ${GREEN}✓ Model already present:${RESET} $filename"
  else
    echo -e "  ⬇ Downloading $filename from CDN..."
    curl -L --progress-bar -o "$target_path" "$url"
    echo -e "  ${GREEN}✓ Downloaded:${RESET} $filename"
  fi
}

# Download Default Fast LLM (Qwen 2.5 0.5B Instruct)
download_model "qwen3-0.6b-q4_k_m.gguf" "https://huggingface.co/Qwen/Qwen2.5-0.5B-Instruct-GGUF/resolve/main/qwen2.5-0.5b-instruct-q4_k_m.gguf"

# Also link to balanced model target if not present
if [[ ! -f "$MODELS_DIR/qwen3-1.7b-q4_k_m.gguf" ]]; then
  cp "$MODELS_DIR/qwen3-0.6b-q4_k_m.gguf" "$MODELS_DIR/qwen3-1.7b-q4_k_m.gguf"
fi

# Download Semantic Search Embedding Model (Nomic Embed v1.5)
download_model "nomic-embed-text-v1.5.Q4_K_M.gguf" "https://huggingface.co/nomic-ai/nomic-embed-text-v1.5-GGUF/resolve/main/nomic-embed-text-v1.5.Q4_K_M.gguf"

# 5. System Health Verification
echo -e "\n${BLUE}▸ [3/3] Verifying Esprit System Status...${RESET}"
if command -v esprit >/dev/null 2>&1; then
  esprit doctor || true
else
  "$INSTALL_DIR/esprit" doctor || true
fi

echo -e "\n${GREEN}${BOLD}✨ Installation complete!${RESET}"
echo -e "You can now run:"
echo -e "  ${CYAN}esprit os \"your instruction\"${RESET}  — macOS Omni-Agent"
echo -e "  ${CYAN}esprit ask \"your question\"${RESET}    — Codebase & RAG Question Answering"
echo -e "  ${CYAN}esprit doctor${RESET}                  — Check system health\n"
