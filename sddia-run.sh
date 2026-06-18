#!/bin/bash

# Abortar en caso de error
set -e

REPO_ROOT="$(cd "$(dirname "$0")" && pwd)"
ORCH_BIN="${SDDIA_EXECUTE_PROCESS_BIN:-}"

# Binario nativo Rust (preferente si existe)
if [ -z "$ORCH_BIN" ] && [ -x "$REPO_ROOT/SddIA/target/debug/execute-process" ]; then
  ORCH_BIN="$REPO_ROOT/SddIA/target/debug/execute-process"
fi
if [ -z "$ORCH_BIN" ] && [ -x "$REPO_ROOT/SddIA/target/release/execute-process" ]; then
  ORCH_BIN="$REPO_ROOT/SddIA/target/release/execute-process"
fi

if [ -n "$ORCH_BIN" ]; then
  exec "$ORCH_BIN" "$@"
fi

# Fallback Python (legacy) — requiere PyYAML en el intérprete activo
VENV_DIR=".venv"
if [ ! -d "$VENV_DIR" ]; then
    python3 -m venv "$VENV_DIR"
fi
source "$VENV_DIR/bin/activate"
pip install -r requirements.txt -q
python SddIA/scripts/qa/execute-process.py "$@"
