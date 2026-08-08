#!/bin/sh
set -eu

if command -v brew >/dev/null 2>&1; then
    brew install krtvysinghh/tap/esprit
    exit 0
fi

if command -v nix >/dev/null 2>&1; then
    nix profile install github:krtvysinghh/Esprit
    exit 0
fi

echo "Install Homebrew or Nix first."
exit 1
