#!/usr/bin/env bash
# sddia-installer — orquestador físico deploy/teardown (Ceguera de Ejecución).
# Uso:
#   ./sddia-installer.sh deploy   [--root PATH] [--vault PATH] [--codex SLUG] [--force] [--skip-build] [--dry-run] [--allow-shared-mailbox]
#   ./sddia-installer.sh teardown [--root PATH] [--force] [--dry-run]
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FORGE_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
# shellcheck source=common/sddia_shell_lib.sh
source "$SCRIPT_DIR/common/sddia_shell_lib.sh"

DEFAULT_ROOT="/home/racso/Aplicaciones/Asistencia_Tormentosa_SddIA"
BUNDLE_PROFILE="full-node"
HOST_REGISTRY="$FORGE_ROOT/.SddIA/instances.json"
STARTER_INSTANCE_ENV="$FORGE_ROOT/SddIA/scripts/starter-kit/.SddIA/.dev/.env.example"
FORGE_ROOT_ENV="$FORGE_ROOT/.dev/.env"
FORGE_WUI_PORT=8765

CMD=""
ROOT_FLAG=""
VAULT_FLAG=""
CODEX_FLAG=""
FORCE=0
SKIP_BUILD=0
DRY_RUN=0
ALLOW_SHARED_MAILBOX=0
ROOT=""
ESC=""
VAULT_DIR=""
WORK=""
PLAN_WUI_PORT=""
PLAN_PORT_SOURCE=""
PLAN_VAULT_ROOT_SOURCE=""
PLAN_VAULT_INSTANCE_SOURCE=""

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
      sed -n '2,6p' "$0"
      exit 0
      ;;
    *) _die "comando desconocido: $CMD (deploy|teardown)" ;;
  esac
  while [[ $# -gt 0 ]]; do
    case "$1" in
      --root) ROOT_FLAG="${2:-}"; [[ -n "$ROOT_FLAG" ]] || _die "--root exige PATH"; shift 2 ;;
      --vault) VAULT_FLAG="${2:-}"; [[ -n "$VAULT_FLAG" ]] || _die "--vault exige PATH"; shift 2 ;;
      --codex) CODEX_FLAG="${2:-}"; [[ -n "$CODEX_FLAG" ]] || _die "--codex exige SLUG"; shift 2 ;;
      --force) FORCE=1; shift ;;
      --skip-build) SKIP_BUILD=1; shift ;;
      --dry-run) DRY_RUN=1; shift ;;
      --allow-shared-mailbox) ALLOW_SHARED_MAILBOX=1; shift ;;
      -h|--help)
        sed -n '2,6p' "$0"
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

_registry_python() {
  python3 - "$@" <<'PY'
import json, hashlib, os, sys
from pathlib import Path

cmd = sys.argv[1]
path = Path(sys.argv[2])

def load():
    if not path.is_file():
        return {}
    try:
        return json.loads(path.read_text())
    except json.JSONDecodeError:
        return {}

def save(data):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(data, indent=2, sort_keys=True) + "\n")

if cmd == "reconcile":
    data = load()
    stale = [k for k, v in data.items() if not Path(v.get("root", "")).exists()]
    for k in stale:
        del data[k]
    save(data)
    sys.exit(0)

if cmd == "used_ports":
    data = load()
    ports = set()
    for v in data.values():
        p = v.get("wui_port")
        if isinstance(p, int):
            ports.add(p)
    print(json.dumps(sorted(ports)))
    sys.exit(0)

if cmd == "derive_port":
    forge_port = int(sys.argv[3])
    esc = sys.argv[4]
    data = load()
    if esc in data and isinstance(data[esc].get("wui_port"), int):
        print(data[esc]["wui_port"])
        sys.exit(0)
    used = {forge_port}
    for v in data.values():
        p = v.get("wui_port")
        if isinstance(p, int):
            used.add(p)
    port = forge_port
    idx = 0
    while port in used:
        idx += 1
        port = forge_port + idx
    print(port)
    sys.exit(0)

if cmd == "mailbox_fp":
    env_path = Path(sys.argv[3])
    host = user = ""
    if env_path.is_file():
        for line in env_path.read_text().splitlines():
            line = line.strip()
            if not line or line.startswith("#") or "=" not in line:
                continue
            k, v = line.split("=", 1)
            k, v = k.strip(), v.strip().strip('"')
            if k == "SDDIA_EMAIL_IMAP_HOST":
                host = v
            if k == "SDDIA_EMAIL_IMAP_USER":
                user = v
    fp = hashlib.sha256(f"{host}|{user}".encode()).hexdigest() if host and user else ""
    print(fp)
    sys.exit(0)

if cmd == "check_mailbox":
    fp = sys.argv[3]
    esc = sys.argv[4]
    if not fp:
        sys.exit(0)
    data = load()
    for k, v in data.items():
        if k == esc:
            continue
        if v.get("mailbox_fingerprint") == fp:
            sys.exit(1)
    sys.exit(0)

if cmd == "upsert":
    esc, root, profile, port, fp = sys.argv[3:8]
    data = load()
    from datetime import datetime, timezone
    data[esc] = {
        "root": root,
        "profile": profile,
        "wui_port": int(port),
        "mailbox_fingerprint": fp or None,
        "created_at": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
    }
    save(data)
    sys.exit(0)

if cmd == "remove":
    esc = sys.argv[3]
    data = load()
    data.pop(esc, None)
    save(data)
    sys.exit(0)

if cmd == "channel_keys":
    env_path = Path(sys.argv[3])
    keys = sys.argv[4].split(",")
    present = {}
    text = env_path.read_text() if env_path.is_file() else ""
    for key in keys:
        ok = False
        for line in text.splitlines():
            line = line.strip()
            if line.startswith(f"{key}="):
                val = line.split("=", 1)[1].strip().strip('"')
                ok = bool(val)
                break
        present[key] = ok
    print(json.dumps(present))
    sys.exit(0)

sys.exit(2)
PY
}

registry_reconcile() {
  _registry_python reconcile "$HOST_REGISTRY"
}

derive_wui_port() {
  registry_reconcile
  PLAN_WUI_PORT="$(_registry_python derive_port "$HOST_REGISTRY" "$FORGE_WUI_PORT" "$ESC")"
  if [[ -n "$VAULT_FLAG" ]]; then
    PLAN_PORT_SOURCE="vault"
  else
    PLAN_PORT_SOURCE="derived"
  fi
}

registry_upsert() {
  local fp
  fp="$(_registry_python mailbox_fp "$HOST_REGISTRY" "$ROOT/.SddIA/.dev/.env")"
  _registry_python upsert "$HOST_REGISTRY" "$ESC" "$ROOT" "$BUNDLE_PROFILE" "$PLAN_WUI_PORT" "$fp"
}

registry_remove() {
  _registry_python remove "$HOST_REGISTRY" "$ESC"
}

check_shared_mailbox() {
  [[ "$ALLOW_SHARED_MAILBOX" -eq 1 ]] && return 0
  local fp
  fp="$(_registry_python mailbox_fp "$HOST_REGISTRY" "$ROOT/.SddIA/.dev/.env")"
  if ! _registry_python check_mailbox "$HOST_REGISTRY" "$fp" "$ESC"; then
    _die "mailbox_fingerprint compartido en host (exit 4; usar --allow-shared-mailbox)" 4
  fi
}

stage_vault() {
  local blacklist
  blacklist="SDDIA_EMAIL_IMAP_SECRET,SDDIA_EMAIL_IMAP_USER,SDDIA_EMAIL_IMAP_HOST,TELEGRAM_BOT_TOKEN,TELEGRAM_ALLOWED_CHAT_ID,IOTA_WALLET_SECRET,IOTA_ANCHOR_PACKAGE_ID,GEMINI_API_KEY,CURSOR_API_KEY"
  mkdir -p "$WORK/vault"
  PLAN_VAULT_ROOT_SOURCE=""
  PLAN_VAULT_INSTANCE_SOURCE=""

  if [[ -f "$FORGE_ROOT_ENV" ]]; then
    PLAN_VAULT_ROOT_SOURCE="$FORGE_ROOT_ENV"
    python3 -c '
import sys
from pathlib import Path
src, dst, bl = Path(sys.argv[1]), Path(sys.argv[2]), set(sys.argv[3].split(","))
lines = []
for line in src.read_text().splitlines():
    if not line.strip() or line.lstrip().startswith("#"):
        lines.append(line)
        continue
    if "=" not in line:
        lines.append(line)
        continue
    k = line.split("=", 1)[0].strip()
    if k in bl:
        continue
    lines.append(line)
dst.write_text("\n".join(lines) + "\n")
' "$FORGE_ROOT_ENV" "$WORK/vault/root.dev.env" "$blacklist"
  fi

  if [[ -n "$VAULT_FLAG" ]]; then
    if [[ -d "$VAULT_FLAG" ]]; then
      PLAN_VAULT_INSTANCE_SOURCE="$(_abs "$VAULT_FLAG")"
      if [[ -f "$VAULT_FLAG/instance.SddIA.dev.env" ]]; then
        cp -f "$VAULT_FLAG/instance.SddIA.dev.env" "$WORK/vault/instance.SddIA.dev.env"
      elif [[ -f "$VAULT_FLAG/.SddIA/.dev/.env" ]]; then
        cp -f "$VAULT_FLAG/.SddIA/.dev/.env" "$WORK/vault/instance.SddIA.dev.env"
      fi
      if [[ -f "$VAULT_FLAG/root.dev.env" ]]; then
        cp -f "$VAULT_FLAG/root.dev.env" "$WORK/vault/root.dev.env"
      fi
    elif [[ -f "$VAULT_FLAG" ]]; then
      PLAN_VAULT_INSTANCE_SOURCE="$(_abs "$VAULT_FLAG")"
      cp -f "$VAULT_FLAG" "$WORK/vault/instance.SddIA.dev.env"
    else
      _die "vault_source no existe: $VAULT_FLAG"
    fi
  elif [[ -f "$STARTER_INSTANCE_ENV" ]]; then
    PLAN_VAULT_INSTANCE_SOURCE="starter-kit"
    cp -f "$STARTER_INSTANCE_ENV" "$WORK/vault/instance.SddIA.dev.env"
  fi

  derive_wui_port
  if [[ -f "$WORK/vault/instance.SddIA.dev.env" ]]; then
    python3 -c '
import sys
from pathlib import Path
p, port = Path(sys.argv[1]), sys.argv[2]
lines, found = [], False
for line in p.read_text().splitlines():
    if line.startswith("SDDIA_CLIENT_PORT="):
        lines.append(f"SDDIA_CLIENT_PORT={port}")
        found = True
    else:
        lines.append(line)
if not found:
    lines.append(f"SDDIA_CLIENT_PORT={port}")
p.write_text("\n".join(lines) + "\n")
' "$WORK/vault/instance.SddIA.dev.env" "$PLAN_WUI_PORT"
  fi

  VAULT_DIR="$WORK/vault"
}

emit_plan() {
  local live_flag="${1:-0}"
  local channel_json="{}"
  if [[ -f "$WORK/vault/instance.SddIA.dev.env" ]]; then
    channel_json="$(_registry_python channel_keys "$HOST_REGISTRY" "$WORK/vault/instance.SddIA.dev.env" \
      "SDDIA_EMAIL_IMAP_SECRET,TELEGRAM_BOT_TOKEN,TELEGRAM_ALLOWED_CHAT_ID,IOTA_WALLET_SECRET,GEMINI_API_KEY,CURSOR_API_KEY")"
  fi
  python3 -c '
import json, sys
extra = json.loads(sys.argv[9]) if sys.argv[9] else {}
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
    "vault_root_source": sys.argv[10] or None,
    "vault_instance_source": sys.argv[11] or None,
    "wui_port": int(sys.argv[12]) if sys.argv[12] else None,
    "port_source": sys.argv[13] or None,
    "channel_keys_present": extra,
}, separators=(",", ":")))
' "$CMD" "$ROOT" "$ESC" "$BUNDLE_PROFILE" \
    "$([[ -n "$VAULT_DIR" ]] && echo 1 || echo 0)" \
    "$FORCE" "$SKIP_BUILD" "$live_flag" \
    "$channel_json" \
    "${PLAN_VAULT_ROOT_SOURCE:-}" \
    "${PLAN_VAULT_INSTANCE_SOURCE:-}" \
    "${PLAN_WUI_PORT:-}" \
    "${PLAN_PORT_SOURCE:-}"
}

_unit_missing_keys() {
  local name="$1"
  local env_file="$ROOT/.SddIA/.dev/.env"
  [[ -f "$env_file" ]] || env_file="$WORK/vault/instance.SddIA.dev.env"
  [[ -f "$env_file" ]] || { echo "INSTANCE_ENV"; return 0; }
  case "$name" in
    telegram-watcher)
      grep -qE '^TELEGRAM_BOT_TOKEN=.+' "$env_file" 2>/dev/null || echo -n "TELEGRAM_BOT_TOKEN,"
      grep -qE '^TELEGRAM_ALLOWED_CHAT_ID=.+' "$env_file" 2>/dev/null || echo -n "TELEGRAM_ALLOWED_CHAT_ID,"
      ;;
    email-watcher)
      grep -qE '^SDDIA_EMAIL_IMAP_HOST=.+' "$env_file" 2>/dev/null || echo -n "SDDIA_EMAIL_IMAP_HOST,"
      grep -qE '^SDDIA_EMAIL_IMAP_USER=.+' "$env_file" 2>/dev/null || echo -n "SDDIA_EMAIL_IMAP_USER,"
      grep -qE '^SDDIA_EMAIL_IMAP_SECRET=.+' "$env_file" 2>/dev/null || echo -n "SDDIA_EMAIL_IMAP_SECRET,"
      ;;
    iota-publish-relay)
      if [[ -n "${SDDIA_IOTA_RELAY_DIR:-}" && -d "${SDDIA_IOTA_RELAY_DIR}" ]]; then
        return 0
      fi
      [[ -d "$ROOT/.SddIA/services/iota-publish-relay" ]] || echo -n "SDDIA_IOTA_RELAY_DIR,"
      ;;
  esac
}

_sha256_file() {
  sha256sum "$1" | awk '{print $1}'
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

_teardown_remove_templates_if_last() {
  local user_sd base other
  user_sd="${XDG_CONFIG_HOME:-$HOME/.config}/systemd/user"
  if systemctl --user list-units --all --plain --no-legend 'sddia-*@*.service' 2>/dev/null | grep -q .; then
    return 0
  fi
  shopt -s nullglob
  for base in "$user_sd"/sddia-*@.service; do
    [[ -f "$base" ]] || continue
    rm -f "$base"
  done
  shopt -u nullglob
  systemctl --user daemon-reload 2>/dev/null || true
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
  registry_remove
  _teardown_remove_templates_if_last
  if [[ -d "$ROOT" ]]; then
    rm -rf "$ROOT"
  fi
  echo "[installer] teardown ok (directorio ausente)" >&2
}

enable_units() {
  local user_sd src base name launcher missing
  user_sd="${XDG_CONFIG_HOME:-$HOME/.config}/systemd/user"
  mkdir -p "$user_sd"
  src="$ROOT/.SddIA/systemd"
  if [[ -d "$src" ]]; then
    shopt -s nullglob
    for unit in "$src"/sddia-*.service; do
      base="$(basename "$unit")"
      if [[ ! -f "$user_sd/$base" ]] || [[ "$(_sha256_file "$unit")" != "$(_sha256_file "$user_sd/$base")" ]]; then
        cp -f "$unit" "$user_sd/$base"
      fi
    done
    shopt -u nullglob
  fi
  systemctl --user daemon-reload
  for name in event-watcher event-sweeper kalma2-bridge email-watcher telegram-watcher github-bridge-watcher iota-publish-relay; do
    launcher="$ROOT/SddIA/scripts/daemons/${name}.sh"
    [[ -f "$launcher" ]] || continue
    missing="$(_unit_missing_keys "$name")"
    missing="${missing%,}"
    if [[ -n "$missing" ]]; then
      echo "[installer] skip sddia-${name}@${ESC}.service (missing: ${missing})" >&2
      continue
    fi
    echo "[installer] enable --now sddia-${name}@${ESC}.service" >&2
    systemctl --user enable --now "sddia-${name}@${ESC}.service"
  done
}

do_deploy() {
  local inputs creator_bin
  echo "[installer] deploy root=$ROOT profile=$BUNDLE_PROFILE" >&2
  validate_host
  registry_reconcile
  if is_live && [[ "$FORCE" -eq 1 ]]; then
    do_teardown
    escape_root
  fi
  stage_vault
  bundle_args=(--out "$ROOT" --profile "$BUNDLE_PROFILE")
  if [[ -n "$CODEX_FLAG" ]]; then
    bundle_args+=(--codex "$CODEX_FLAG")
  fi
  if [[ "$SKIP_BUILD" -eq 1 ]]; then
    bundle_args+=(--skip-build)
  fi
  (
    cd "$FORGE_ROOT"
    ./SddIA/scripts/build-release-bundle.sh "${bundle_args[@]}"
  )
  inputs="$(python3 -c '
import json, sys
payload = {
    "instance_root": sys.argv[1],
    "runtime_profile": "engineering",
    "skip_ignition": True,
}
if len(sys.argv) > 2 and sys.argv[2]:
    payload["vault_source"] = sys.argv[2]
if len(sys.argv) > 3 and sys.argv[3]:
    payload["codex_slug"] = sys.argv[3]
print(json.dumps(payload, separators=(",", ":")))
' "$ROOT" "${VAULT_DIR:-}" "${CODEX_FLAG:-}")"
  (
    cd "$FORGE_ROOT"
    ./sddia-run.sh --process instance-creator --inputs "$inputs"
  )
  check_shared_mailbox
  enable_units
  registry_upsert
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

  if [[ "$CMD" == "deploy" ]]; then
    stage_vault
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
