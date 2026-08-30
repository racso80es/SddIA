#!/usr/bin/env bash
# Lanzador Unix: recolector event-sweeper (nativo Rust)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=../scripts/common/sddia_shell_lib.sh
source "$SCRIPT_DIR/../scripts/common/sddia_shell_lib.sh"
_FALLBACK_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
REPO_ROOT="$(_sddia_resolve_instance_root "$_FALLBACK_ROOT")"
cd "$REPO_ROOT"

if BIN="$(_sddia_resolve_daemon_binary "$REPO_ROOT" event-sweeper)"; then
  exec "$BIN" "$@"
fi

echo "[event-sweeper] binario no encontrado (build: cd SddIA && cargo build -p event-sweeper)" >&2
exit 1
