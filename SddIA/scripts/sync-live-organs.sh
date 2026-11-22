#!/usr/bin/env bash
# sync-live-organs.sh — Ritual canónico pre-cierre (MAN-52 / AC-4 / D-R5):
# Recompila los 9 órganos vivos en --release, sella testigos .sha256 y reinicia
# las unidades systemd --user activas del workspace.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
TARGET_DIR="$REPO_ROOT/SddIA/target"

ORGANS=(
  execute-process
  kalma2-bridge
  event-watcher
  github-bridge-watcher
  email-watcher
  telegram-watcher
  iota-publish-relay
  sddia-workspace-server
  sddia-qa
)

SYSTEMD_ORGANS=(
  kalma2-bridge
  event-watcher
  github-bridge-watcher
  email-watcher
  telegram-watcher
  iota-publish-relay
  sddia-workspace-server
)

PKG_ARGS=()
for o in "${ORGANS[@]}"; do
  PKG_ARGS+=("-p" "$o")
done

echo "[sync-live-organs] Compilando 9 órganos en --release..."
(
  cd "$REPO_ROOT/SddIA"
  CARGO_TARGET_DIR="$TARGET_DIR" cargo build --release "${PKG_ARGS[@]}"
)

REL_EP="$TARGET_DIR/release/execute-process"
if [[ ! -x "$REL_EP" ]]; then
  echo "[ERROR] execute-process release ausente en $REL_EP" >&2
  exit 1
fi

NAMES_JSON=$(printf '"%s",' "${ORGANS[@]}")
NAMES_JSON="[${NAMES_JSON%,}]"

echo "[sync-live-organs] Sellando testigos .sha256..."
CARGO_TARGET_DIR="$TARGET_DIR" "$REL_EP" --seal-capsules --inputs "{\"profile\":\"release\",\"write_genome\":false,\"write_witness\":true,\"names\":${NAMES_JSON}}"

if command -v systemctl >/dev/null 2>&1; then
  ESC="$(systemd-escape -p "$REPO_ROOT" 2>/dev/null || echo "${REPO_ROOT#/}" | tr '/' '-')"
  for o in "${SYSTEMD_ORGANS[@]}"; do
    UNIT="sddia-${o}@${ESC}.service"
    if systemctl --user is-active --quiet "$UNIT" 2>/dev/null; then
      echo "[sync-live-organs] Reiniciando unidad activa: $UNIT"
      systemctl --user try-restart "$UNIT"
    fi
  done
fi

echo "[sync-live-organs] Verificando frescura con sddia-qa verify-compiled-capsules..."
"$TARGET_DIR/release/sddia-qa" verify-compiled-capsules --json

echo "[sync-live-organs] OK"
