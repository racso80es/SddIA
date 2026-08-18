#!/usr/bin/env bash
# Lanzador Unix: centinela email-watcher (nativo Rust)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
TARGET="$REPO_ROOT/SddIA/target"

NATIVE_RELEASE="$TARGET/release/email-watcher"
NATIVE_DEBUG="$TARGET/debug/email-watcher"

cd "$REPO_ROOT"

if [[ -x "$NATIVE_DEBUG" ]]; then
  exec "$NATIVE_DEBUG" "$@"
fi
if [[ -x "$NATIVE_RELEASE" ]]; then
  exec "$NATIVE_RELEASE" "$@"
fi

echo "[email-watcher] binario no encontrado (build: cd SddIA && cargo build -p email-watcher)" >&2
exit 1
