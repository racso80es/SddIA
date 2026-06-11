#!/bin/bash
source "$HOME/.cargo/env" 2>/dev/null || true
export PATH="$HOME/.wasmtime/bin:$PATH"
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
export CARGO_TARGET_DIR="${ROOT}/target"
TARGET="${WASI_TARGET:-wasm32-wasip1}"

if ! command -v cargo >/dev/null 2>&1; then
  echo '{"success":false,"exitCode":1,"message":"cargo not found"}' >&2
  exit 1
fi

rustup target add "${TARGET}" >/dev/null 2>&1 || true
cd "${ROOT}"
cargo build --release --target "${TARGET}"

ARTIFACT="${CARGO_TARGET_DIR}/${TARGET}/release/wasi-poc.wasm"
if [[ ! -f "${ARTIFACT}" ]]; then
  ALT="${CARGO_TARGET_DIR}/${TARGET}/release/wasi-poc"
  if [[ -f "${ALT}" ]]; then
    ARTIFACT="${ALT}"
  fi
fi
if [[ ! -f "${ARTIFACT}" ]]; then
  echo "{\"success\":false,\"exitCode\":1,\"message\":\"artifact missing: ${ARTIFACT}\"}" >&2
  exit 1
fi

printf '{"success":true,"exitCode":0,"message":"wasi artifact built","result":{"artifact":"%s","target":"%s"}}\n' \
  "${ARTIFACT}" "${TARGET}"
