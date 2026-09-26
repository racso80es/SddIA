#!/usr/bin/env bash
# sddia-installer — orquestador físico deploy/teardown (Ceguera de Ejecución).
# Uso:
#   ./sddia-installer.sh deploy   [--root PATH] [--vault PATH] [--force] [--skip-build] [--dry-run]
#   ./sddia-installer.sh teardown [--root PATH] [--force] [--dry-run]
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FORGE_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
# shellcheck source=common/sddia_shell_lib.sh
source "$SCRIPT_DIR/common/sddia_shell_lib.sh"

DEFAULT_ROOT="/home/racso/Aplicaciones/Asistencia_Tormentosa_SddIA"
BUNDLE_PROFILE="full-node"
CMD=""
ROOT_FLAG=""
VAULT_FLAG=""
FORCE=0
SKIP_BUILD=0
DRY_RUN=0
ROOT=""
ESC=""
VAULT_DIR=""
WORK=""

_die() {
  local code="${2:-1}"
  echo "[installer] ERROR: $1" >&2
  exit "$code"
}

_abs() {
  realpath -m "$1"
}

_cleanup() {
  if [[ -n "${WORK:-}" && -d "${WORK:-}" ]]; then
    rm -rf "$WORK"
  fi
}

_parse() {
  [[ $# -ge 1 ]] || _die "comando requerido: deploy | teardown"
  CMD="$(echo "$1" | tr '[:upper:]' '[:lower:]')"
  shift
  case "$CMD" in
    deploy|teardown) ;;
    -h|--help)
      sed -n '2,5p' "$0"
      exit 0
      ;;
    *) _die "comando desconocido: $CMD (deploy|teardown)" ;;
  esac
  while [[ $# -gt 0 ]]; do
    case "$1" in
      --root) ROOT_FLAG="${2:-}"; [[ -n "$ROOT_FLAG" ]] || _die "--root exige PATH"; shift 2 ;;
      --vault) VAULT_FLAG="${2:-}"; [[ -n "$VAULT_FLAG" ]] || _die "--vault exige PATH"; shift 2 ;;
      --force) FORCE=1; shift ;;
      --skip-build) SKIP_BUILD=1; shift ;;
      --dry-run) DRY_RUN=1; shift ;;
      -h|--help)
        sed -n '2,5p' "$0"
        exit 0
        ;;
      *) _die "argumento desconocido: $1" ;;
    esac
  done
}

resolve_root() {
  local candidate forge_abs
  if [[ -n "$ROOT_FLAG" ]]; then
    candidate="$ROOT_FLAG"
  elif [[ -n "${SDDIA_INSTALL_ROOT:-}" ]]; then
    candidate="$SDDIA_INSTALL_ROOT"
  else
    candidate="$DEFAULT_ROOT"
  fi
  if [[ "$candidate" != /* ]]; then
    candidate="$FORGE_ROOT/$candidate"
  fi
  ROOT="$(_abs "$candidate")"
  forge_abs="$(_abs "$FORGE_ROOT")"
  home_abs="$(_abs "${HOME:-/home}")"
  if [[ -z "$ROOT" || "$ROOT" == "/" || "$ROOT" == "/home" || "$ROOT" == "$home_abs" ]]; then
    _die "ROOT inseguro ($ROOT). Abort." 1
  fi
  if [[ "$ROOT" == "$forge_abs" || "$ROOT" == "$forge_abs"/* ]]; then
    _die "ROOT coincide o está bajo la forja ($forge_abs). Abort." 1
  fi
}

escape_root() {
  command -v systemd-escape >/dev/null 2>&1 || _die "systemd-escape ausente"
  ESC="$(systemd-escape -p "$ROOT")"
}

is_live() {
  [[ -d "$ROOT/.SddIA" ]] && return 0
  [[ -f "$ROOT/MANIFEST.json" ]] && return 0
  if command -v systemctl >/dev/null 2>&1; then
    if systemctl --user --quiet is-active "sddia-event-watcher@${ESC}.service" 2>/dev/null; then
      return 0
    fi
    if systemctl --user list-units --state=active --plain --no-legend "sddia-*@${ESC}.service" 2>/dev/null | grep -q .; then
      return 0
    fi
  fi
  return 1
}

validate_host() {
  command -v systemctl >/dev/null 2>&1 || _die "systemctl ausente (se exige --user)"
  command -v systemd-escape >/dev/null 2>&1 || _die "systemd-escape ausente"
  if [[ "$SKIP_BUILD" -eq 0 ]]; then
    command -v cargo >/dev/null 2>&1 || _die "cargo ausente (usar --skip-build si el ELF ya existe)"
  fi
}

emit_plan() {
  local live_flag="${1:-0}"
  python3 -c '
import json, sys
print(json.dumps({
    "command": sys.argv[1],
    "root": sys.argv[2],
    "esc": sys.argv[3],
    "bundle_profile": sys.argv[4],
    "vault_set": sys.argv[5] == "1",
    "force": sys.argv[6] == "1",
    "skip_build": sys.argv[7] == "1",
    "dry_run": True,
    "live": sys.argv[8] == "1",
}, separators=(",", ":")))
' "$CMD" "$ROOT" "$ESC" "$BUNDLE_PROFILE" \
    "$([[ -n "$VAULT_DIR" || -n "$VAULT_FLAG" || -f "$FORGE_ROOT/.dev/.env" || -f "$FORGE_ROOT/.SddIA/.dev/.env" ]] && echo 1 || echo 0)" \
    "$FORCE" "$SKIP_BUILD" "$live_flag"
}

stage_vault() {
  local f
  VAULT_DIR=""
  if [[ -n "$VAULT_FLAG" ]]; then
    if [[ -d "$VAULT_FLAG" ]]; then
      VAULT_DIR="$(_abs "$VAULT_FLAG")"
      return 0
    fi
    if [[ -f "$VAULT_FLAG" ]]; then
      mkdir -p "$WORK/vault"
      cp -f "$VAULT_FLAG" "$WORK/vault/instance.SddIA.dev.env"
      VAULT_DIR="$WORK/vault"
      return 0
    fi
    _die "vault_source no existe: $VAULT_FLAG"
  fi
  for f in "$FORGE_ROOT/.dev/.env" "$FORGE_ROOT/.SddIA/.dev/.env"; do
    if [[ -f "$f" ]]; then
      mkdir -p "$WORK/vault"
      cp -f "$f" "$WORK/vault/instance.SddIA.dev.env"
      VAULT_DIR="$WORK/vault"
      return 0
    fi
  done
}

stop_lock_residuals() {
  local lock
  [[ -d "$ROOT/.SddIA/daemons/status" ]] || return 0
  shopt -s nullglob
  for lock in "$ROOT/.SddIA/daemons/status"/*.lock; do
    _sddia_stop_lock_pid "$lock"
  done
  shopt -u nullglob
}

_signal_matching_pids() {
  local sig="$1"
  local pid cwd exe cmd
  for pid in /proc/[0-9]*; do
    pid="${pid##*/}"
    [[ "$pid" =~ ^[0-9]+$ ]] || continue
    cwd="$(readlink -f "/proc/${pid}/cwd" 2>/dev/null || true)"
    exe="$(readlink -f "/proc/${pid}/exe" 2>/dev/null || true)"
    cmd="$(tr '\0' ' ' < "/proc/${pid}/cmdline" 2>/dev/null || true)"
    if [[ "$cmd" == *"${ROOT}/start-sddia.sh"* ]] \
      || [[ "$cwd" == "$ROOT" || "$cwd" == "${ROOT}/"* ]] \
      || [[ "$exe" == "${ROOT}/"* ]]; then
      kill "-${sig}" "$pid" 2>/dev/null || true
    fi
  done
}

signal_instance_procs() {
  _signal_matching_pids TERM
  _signal_matching_pids KILL
}

do_teardown() {
  local unit stem
  echo "[installer] teardown root=$ROOT esc=$ESC" >&2
  signal_instance_procs
  if command -v systemctl >/dev/null 2>&1; then
    for stem in sddia-email-watcher sddia-event-watcher sddia-event-sweeper \
      sddia-kalma2-bridge sddia-telegram-watcher sddia-github-bridge-watcher \
      sddia-iota-publish-relay; do
      unit="${stem}@${ESC}.service"
      systemctl --user stop "$unit" 2>/dev/null || true
      systemctl --user disable "$unit" 2>/dev/null || true
      systemctl --user reset-failed "$unit" 2>/dev/null || true
    done
    while IFS= read -r unit; do
      [[ -z "$unit" ]] && continue
      unit="${unit%% *}"
      [[ "$unit" == *"@${ESC}.service" ]] || continue
      systemctl --user stop "$unit" 2>/dev/null || true
      systemctl --user disable "$unit" 2>/dev/null || true
      systemctl --user reset-failed "$unit" 2>/dev/null || true
    done < <(systemctl --user list-units --all --plain --no-legend "sddia-*@${ESC}.service" 2>/dev/null || true)
    systemctl --user daemon-reload 2>/dev/null || true
  fi
  stop_lock_residuals
  signal_instance_procs
  if [[ -d "$ROOT" ]]; then
    rm -rf "$ROOT"
  fi
  echo "[installer] teardown ok (directorio ausente)" >&2
}

enable_units() {
  local user_sd src base name launcher
  user_sd="${XDG_CONFIG_HOME:-$HOME/.config}/systemd/user"
  mkdir -p "$user_sd"
  src="$ROOT/.SddIA/systemd"
  if [[ -d "$src" ]]; then
    shopt -s nullglob
    for unit in "$src"/sddia-*.service; do
      base="$(basename "$unit")"
      if [[ ! -f "$user_sd/$base" ]]; then
        cp -f "$unit" "$user_sd/$base"
      fi
    done
    shopt -u nullglob
  fi
  systemctl --user daemon-reload
  for name in event-watcher event-sweeper kalma2-bridge email-watcher telegram-watcher github-bridge-watcher iota-publish-relay; do
    launcher="$ROOT/SddIA/scripts/daemons/${name}.sh"
    [[ -f "$launcher" ]] || continue
    echo "[installer] enable --now sddia-${name}@${ESC}.service" >&2
    systemctl --user enable --now "sddia-${name}@${ESC}.service"
  done
}

do_deploy() {
  local inputs creator_bin
  echo "[installer] deploy root=$ROOT profile=$BUNDLE_PROFILE" >&2
  validate_host
  if is_live && [[ "$FORCE" -eq 1 ]]; then
    do_teardown
  fi
  bundle_args=(--out "$ROOT" --profile "$BUNDLE_PROFILE")
  if [[ "$SKIP_BUILD" -eq 1 ]]; then
    bundle_args+=(--skip-build)
  fi
  (
    cd "$FORGE_ROOT"
    ./SddIA/scripts/build-release-bundle.sh "${bundle_args[@]}"
  )
  stage_vault
  inputs="$(python3 -c '
import json, sys
payload = {
    "instance_root": sys.argv[1],
    "runtime_profile": "engineering",
    "skip_ignition": True,
}
if len(sys.argv) > 2 and sys.argv[2]:
    payload["vault_source"] = sys.argv[2]
print(json.dumps(payload, separators=(",", ":")))
' "$ROOT" "${VAULT_DIR:-}")"
  (
    cd "$FORGE_ROOT"
    ./sddia-run.sh --process instance-creator --inputs "$inputs"
  )
  enable_units
  echo "[installer] deploy ok" >&2
}

main() {
  _parse "$@"
  _sddia_augment_operator_path
  _sddia_load_vault "$FORGE_ROOT" || true
  resolve_root
  escape_root
  WORK="$(mktemp -d /tmp/sddia-installer.XXXXXX)"
  trap _cleanup EXIT

  local live=0
  if is_live; then
    live=1
  fi

  if [[ "$DRY_RUN" -eq 1 ]]; then
    if [[ "$CMD" == "deploy" && "$live" -eq 1 && "$FORCE" -eq 0 ]]; then
      emit_plan 1
      _die "destino vivo ($ROOT); exigir --force" 2
    fi
    emit_plan "$live"
    exit 0
  fi

  if [[ "$CMD" == "deploy" ]]; then
    if [[ "$live" -eq 1 && "$FORCE" -eq 0 ]]; then
      _die "destino vivo ($ROOT); exigir --force" 2
    fi
    do_deploy
    exit 0
  fi

  if [[ "$FORCE" -eq 0 ]]; then
    _die "teardown exige --force (consentimiento no interactivo)" 3
  fi
  do_teardown
}

main "$@"
