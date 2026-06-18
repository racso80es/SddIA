#!/usr/bin/env bash
# Ejecuta un Centinela con bóveda cargada (usado en foreground y en terminal nueva).
set -euo pipefail

if [[ $# -lt 1 ]]; then
  echo "[ERROR] Uso: $(basename "$0") <daemon-name> [args...]" >&2
  exit 1
fi

DAEMON="$1"
shift

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
ENTRY="$REPO_ROOT/SddIA/daemons/${DAEMON}.sh"

_resolve_python() {
  if command -v python3 >/dev/null 2>&1; then
    echo python3
  elif command -v python >/dev/null 2>&1; then
    echo python
  else
    echo "[ERROR] Python 3 requerido para cargar bóveda (.dev/.env)." >&2
    return 1
  fi
}

_load_vault() {
  local python
  python="$(_resolve_python)" || return 1
  # shellcheck disable=SC2046
  eval "$("$python" - "$REPO_ROOT" <<'PY'
import shlex
import sys
from pathlib import Path

repo = Path(sys.argv[1])
sys.path.insert(0, str(repo / "SddIA" / "scripts" / "qa"))
from env_loader import load_hierarchical_env

for key, value in load_hierarchical_env(repo).items():
    print(f"export {key}={shlex.quote(value)}")
PY
)"
}

_setup_node_path() {
  local node_bin
  for node_bin in "$REPO_ROOT"/.tools/node-v*-linux-x64/bin; do
    if [[ -x "$node_bin/node" ]]; then
      export PATH="$node_bin:$PATH"
      break
    fi
  done
}

if [[ ! -f "$ENTRY" ]]; then
  echo "[ERROR] Entrypoint no encontrado: $ENTRY" >&2
  exit 1
fi

export PYTHONUTF8=1
_load_vault
_setup_node_path
cd "$REPO_ROOT"
exec "$ENTRY" "$@"
