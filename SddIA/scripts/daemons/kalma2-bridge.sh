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

warn_if_mayeuta_llm_missing() {
  local candidate
  if [[ -n "${SDDIA_MAYEUTA_LLM_BIN:-}" ]]; then
    if _sddia_is_native_elf "${SDDIA_MAYEUTA_LLM_BIN}"; then
      return 0
    fi
    echo "[WARN] SDDIA_MAYEUTA_LLM_BIN no es ELF nativo (${SDDIA_MAYEUTA_LLM_BIN}). POST /api/chat colapsará. Compilar: cd SddIA && cargo build --release -p mayeuta-llm" >&2
    return 0
  fi
  for candidate in \
    "$REPO_ROOT/SddIA/target/release/mayeuta-llm" \
    "$REPO_ROOT/SddIA/target/debug/mayeuta-llm"; do
    if _sddia_is_native_elf "$candidate"; then
      return 0
    fi
  done
  echo "[WARN] mayeuta-llm no encontrado en SddIA/target/{release,debug}. POST /api/chat emitirá prosthetic_collapse. Compilar: cd SddIA && cargo build --release -p mayeuta-llm" >&2
}

warn_if_mayeuta_llm_missing

export SDDIA_REPO_ROOT="$REPO_ROOT"
exec "$BRIDGE_BIN"
