#!/usr/bin/env bash
# Lanzador Unix: cápsula bus-operator (stdin JSON → stdout JSON)
# SSOT: SddIA/skills/ + SddIA/target (WASI o nativo)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
TARGET="$REPO_ROOT/SddIA/target"

_emit_error() {
  printf '%s\n' "{\"success\":false,\"exitCode\":1,\"error\":\"$1\"}"
}

WASM_RELEASE="$TARGET/wasm32-wasip1/release/bus-operator.wasm"
WASM_DEBUG="$TARGET/wasm32-wasip1/debug/bus-operator.wasm"
NATIVE_RELEASE="$TARGET/release/bus-operator"
NATIVE_DEBUG="$TARGET/debug/bus-operator"

cd "$REPO_ROOT"

if command -v wasmtime >/dev/null 2>&1; then
  if [[ -f "$WASM_RELEASE" ]]; then
    exec wasmtime run --dir=. "$WASM_RELEASE"
  fi
  if [[ -f "$WASM_DEBUG" ]]; then
    exec wasmtime run --dir=. "$WASM_DEBUG"
  fi
fi

if [[ -f "$NATIVE_RELEASE" ]]; then
  exec "$NATIVE_RELEASE"
fi
if [[ -f "$NATIVE_DEBUG" ]]; then
  exec "$NATIVE_DEBUG"
fi

_emit_error "cápsula bus-operator no encontrada (build: cd SddIA && cargo build && cargo build --target wasm32-wasip1)"
exit 1
