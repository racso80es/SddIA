#!/usr/bin/env bash
# Lanzador Unix: kalma2-bridge (HTTP WUI) en foreground para systemd Type=simple.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"

# shellcheck source=../common/sddia_shell_lib.sh
source "$SCRIPT_DIR/../common/sddia_shell_lib.sh"
_sddia_load_vault "$REPO_ROOT"
cd "$REPO_ROOT"

_is_native_elf() {
  local candidate="$1"
  local mime
  [[ -x "$candidate" ]] || return 1
  mime="$(file -Lb --mime-type "$candidate" 2>/dev/null || true)"
  [[ "$mime" == "application/x-executable" || "$mime" == "application/x-pie-executable" ]]
}

resolve_bridge_bin() {
  if [[ -n "${SDDIA_KALMA2_BRIDGE_BIN:-}" ]] && _is_native_elf "${SDDIA_KALMA2_BRIDGE_BIN}"; then
    printf '%s\n' "${SDDIA_KALMA2_BRIDGE_BIN}"
    return 0
  fi
  local rel
  for rel in SddIA/target/debug/kalma2-bridge SddIA/target/release/kalma2-bridge; do
    if _is_native_elf "$REPO_ROOT/$rel"; then
      printf '%s\n' "$REPO_ROOT/$rel"
      return 0
    fi
  done
  return 1
}

BRIDGE_BIN="$(resolve_bridge_bin || true)"
if [[ -z "$BRIDGE_BIN" ]]; then
  echo "[ERROR] kalma2-bridge nativo no encontrado. Compilar: cd SddIA && cargo build -p kalma2-bridge" >&2
  exit 1
fi

export SDDIA_REPO_ROOT="$REPO_ROOT"
exec "$BRIDGE_BIN"
