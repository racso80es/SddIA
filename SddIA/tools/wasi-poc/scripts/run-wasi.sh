#!/bin/bash
source "$HOME/.cargo/env" 2>/dev/null || true
export PATH="$HOME/.wasmtime/bin:$PATH"
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
export CARGO_TARGET_DIR="${ROOT}/target"
TARGET="${WASI_TARGET:-wasm32-wasip1}"
ARTIFACT="${CARGO_TARGET_DIR}/${TARGET}/release/wasi-poc.wasm"
if [[ ! -f "${ARTIFACT}" && -f "${CARGO_TARGET_DIR}/${TARGET}/release/wasi-poc" ]]; then
  ARTIFACT="${CARGO_TARGET_DIR}/${TARGET}/release/wasi-poc"
fi
RUNTIME="${WASI_RUNTIME:-wasmtime}"

if [[ ! -f "${ARTIFACT}" ]]; then
  "${ROOT}/scripts/build-wasi.sh" >/dev/null
fi

if ! command -v "${RUNTIME}" >/dev/null 2>&1; then
  echo '{"success":false,"exitCode":1,"message":"wasmtime not found; install from https://wasmtime.dev"}' >&2
  exit 1
fi

if [[ $# -ge 1 ]]; then
  PAYLOAD="$1"
else
  PAYLOAD='{"meta":{"schemaVersion":"2.0","entityKind":"tool","entityId":"wasi-poc"},"request":{"ping":true}}'
fi

# Sin --dir: ceguera espacial; solo stdin/stdout.
printf '%s' "${PAYLOAD}" | "${RUNTIME}" run "${ARTIFACT}"
