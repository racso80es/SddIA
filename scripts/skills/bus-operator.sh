#!/usr/bin/env bash
# Lanzador Unix: cápsula bus-operator (stdin JSON → stdout JSON)
# Excepción D8: bus-operator.py es la ruta funcional en laboratorio.
# Ref: docs/features/migracion-rust-wasi/clarify.md §D8
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
PY_SCRIPT="$SCRIPT_DIR/bus-operator.py"

_emit_error() {
  "$PYTHON" -c 'import json,sys; json.dump({"success":False,"exitCode":1,"error":sys.argv[1]}, sys.stdout, ensure_ascii=False); sys.stdout.write("\n")' "$1"
}

if [[ ! -f "$PY_SCRIPT" ]]; then
  if command -v python3 >/dev/null 2>&1; then
    PYTHON=python3
  elif command -v python >/dev/null 2>&1; then
    PYTHON=python
  else
    printf '%s\n' '{"success":false,"exitCode":1,"error":"cápsula inexistente: scripts/skills/bus-operator.py"}'
    exit 1
  fi
  _emit_error "cápsula inexistente: scripts/skills/bus-operator.py"
  exit 1
fi

if [[ -x "$REPO_ROOT/.venv/bin/python3" ]]; then
  PYTHON="$REPO_ROOT/.venv/bin/python3"
elif command -v python3 >/dev/null 2>&1; then
  PYTHON=python3
elif command -v python >/dev/null 2>&1; then
  PYTHON=python
else
  printf '%s\n' '{"success":false,"exitCode":1,"error":"Python 3 no encontrado"}'
  exit 1
fi

export PYTHONUTF8=1
cd "$REPO_ROOT"
exec "$PYTHON" "$PY_SCRIPT"
