#!/usr/bin/env bash
# Prepara target/release para AC-4 lab (CI installer-smoke). No despliega instancia.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$ROOT/SddIA"
# AC-4 lab: deploy --skip-build invoca build-release-bundle (perfil engineering).
# No hace falta --workspace; sí los binarios obligatorios en skip-build (build-release-bundle.sh).
INSTALLER_LAB_RELEASE_PKGS=(
  execute-process
  kalma2-bridge
  event-watcher
  send-telegram-notification
  llm-router
  thought-graph-access
)
build_args=(build --release)
for pkg in "${INSTALLER_LAB_RELEASE_PKGS[@]}"; do
  build_args+=(-p "$pkg")
done
echo "[prep-installer-lab] cargo ${build_args[*]}"
cargo "${build_args[@]}" -q
for pkg in "${INSTALLER_LAB_RELEASE_PKGS[@]}"; do
  [[ -x "$ROOT/SddIA/target/release/$pkg" ]] || {
    echo "[prep-installer-lab] ERROR: $pkg ausente tras build" >&2
    exit 1
  }
done
echo "[prep-installer-lab] OK"
