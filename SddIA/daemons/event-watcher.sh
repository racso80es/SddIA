#!/usr/bin/env bash
# Lanzador Unix: centinela event-watcher (nativo Rust)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
TARGET="$REPO_ROOT/SddIA/target"

NATIVE_RELEASE="$TARGET/release/event-watcher"
NATIVE_DEBUG="$TARGET/debug/event-watcher"

cd "$REPO_ROOT"

if [[ -x "$NATIVE_DEBUG" ]]; then
  exec "$NATIVE_DEBUG" "$@"
fi
if [[ -x "$NATIVE_RELEASE" ]]; then
  exec "$NATIVE_RELEASE" "$@"
fi

echo "[event-watcher] binario no encontrado (build: cd SddIA && CARGO_TARGET_DIR=\$PWD/target cargo build -p event-watcher)" >&2
exit 1
