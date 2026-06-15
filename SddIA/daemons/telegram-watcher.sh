#!/usr/bin/env bash
# Lanzador Unix: centinela telegram-watcher (nativo Rust)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
TARGET="$REPO_ROOT/SddIA/target"

NATIVE_RELEASE="$TARGET/release/telegram-watcher"
NATIVE_DEBUG="$TARGET/debug/telegram-watcher"

cd "$REPO_ROOT"

if [[ -f "$NATIVE_RELEASE" ]]; then
  exec "$NATIVE_RELEASE" "$@"
fi
if [[ -f "$NATIVE_DEBUG" ]]; then
  exec "$NATIVE_DEBUG" "$@"
fi

echo "[telegram-watcher] binario no encontrado (build: cd SddIA && CARGO_TARGET_DIR=\$PWD/target cargo build -p telegram-watcher)" >&2
exit 1
