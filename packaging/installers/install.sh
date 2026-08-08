#!/usr/bin/env bash
set -euo pipefail

REPO="krtvysinghh/Esprit"
INSTALL_DIR="${HOME}/.local/bin"

mkdir -p "$INSTALL_DIR"

OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS-$ARCH" in
  Darwin-arm64)
    ASSET="macos-arm64"
    ;;
  Darwin-x86_64)
    ASSET="macos-x86_64"
    ;;
  Linux-x86_64)
    ASSET="linux-x86_64"
    ;;
  *)
    echo "Unsupported platform: $OS-$ARCH"
    exit 1
    ;;
esac

VERSION="$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" | \
  sed -n 's/.*"tag_name": "\(.*\)",/\1/p' | head -1)"

VERSION="${VERSION#v}"

ARCHIVE="esprit-${VERSION}-${ASSET}.tar.gz"
URL="https://github.com/${REPO}/releases/download/v${VERSION}/${ARCHIVE}"

TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

curl -fL "$URL" -o "$TMP/$ARCHIVE"
tar -xzf "$TMP/$ARCHIVE" -C "$TMP"

install -m 755 "$TMP/esprit" "$INSTALL_DIR/esprit"

echo "Esprit ${VERSION} installed."
echo "Binary: $INSTALL_DIR/esprit"
