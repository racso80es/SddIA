#!/usr/bin/env bash
# Lanzador Unix: kalma2-bridge (HTTP WUI) en foreground para systemd Type=simple.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=../common/sddia_shell_lib.sh
source "$SCRIPT_DIR/../common/sddia_shell_lib.sh"
_FALLBACK_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
REPO_ROOT="$(_sddia_resolve_instance_root "$_FALLBACK_ROOT")"
_sddia_load_vault "$REPO_ROOT"
cd "$REPO_ROOT"

resolve_bridge_bin() {
  if [[ -n "${SDDIA_KALMA2_BRIDGE_BIN:-}" ]] && _sddia_is_native_elf "${SDDIA_KALMA2_BRIDGE_BIN}"; then
    printf '%s\n' "${SDDIA_KALMA2_BRIDGE_BIN}"
    return 0
  fi
  _sddia_resolve_daemon_binary "$REPO_ROOT" kalma2-bridge
}

BRIDGE_BIN="$(resolve_bridge_bin || true)"
if [[ -z "$BRIDGE_BIN" ]]; then
  echo "[ERROR] kalma2-bridge nativo no encontrado. Compilar: cd SddIA && cargo build -p kalma2-bridge" >&2
  exit 1
fi

export SDDIA_REPO_ROOT="$REPO_ROOT"
exec "$BRIDGE_BIN"
