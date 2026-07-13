#!/usr/bin/env bash
# Lanzador genérico de tool catalogada (stdin JSON → stdout envelope JSON).
set -euo pipefail

if [[ $# -lt 1 ]]; then
  echo "[ERROR] Uso: $(basename "$0") <tool-name> [--prefer-native] [args...]" >&2
  exit 1
fi

TOOL="$1"
shift
PREFER_NATIVE=()
if [[ "${1:-}" == "--prefer-native" ]]; then
  PREFER_NATIVE=(--prefer-native)
  shift
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
# shellcheck source=../common/sddia_shell_lib.sh
source "$SCRIPT_DIR/../common/sddia_shell_lib.sh"
_sddia_resolve_orchestrator "$REPO_ROOT"
cd "$REPO_ROOT"
exec "$SDDIA_EXECUTE_PROCESS_BIN" --tool "$TOOL" "${PREFER_NATIVE[@]}" "$@"
