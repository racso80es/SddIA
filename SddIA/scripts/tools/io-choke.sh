#!/usr/bin/env bash
# Lanzador Unix: tool io-choke (stdin JSON → stdout envelope JSON)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

if command -v python3 >/dev/null 2>&1; then
  PYTHON=python3
elif command -v python >/dev/null 2>&1; then
  PYTHON=python
else
  echo "[ERROR] Python 3 requerido." >&2
  exit 1
fi

export PYTHONUTF8=1
exec "$PYTHON" "$SCRIPT_DIR/invoke.py" io-choke "$@"
