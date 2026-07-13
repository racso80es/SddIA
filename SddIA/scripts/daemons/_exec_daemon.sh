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

# shellcheck source=../common/sddia_shell_lib.sh
source "$SCRIPT_DIR/../common/sddia_shell_lib.sh"

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

_sddia_load_vault "$REPO_ROOT"
_setup_node_path
cd "$REPO_ROOT"
exec "$ENTRY" "$@"
