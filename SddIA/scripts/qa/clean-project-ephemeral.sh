#!/usr/bin/env bash
# Limpieza de ficheros temporales de la instancia Core-self.
# SSOT de rutas: SddIA/core/cumulo.paths.json
# Por defecto solo inventaría. El borrado exige --apply.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO="$(cd "$SCRIPT_DIR/../../.." && pwd)"

APPLY=0
JSON=0
NO_WAIT=0
WITH_DAEMON_LOGS=0
WITH_EVENTS_FS=0
WITH_CARGO_DEPS=0
DAEMON_LOG_DAYS=14
SCRATCH_HOURS=168
PAUSE_DONE=0
CARGO_HELPER="$SCRIPT_DIR/clean-project-ephemeral-cargo-deps.py"

pause_for_close() {
  [[ "$PAUSE_DONE" -eq 1 ]] && return 0
  [[ "$NO_WAIT" -eq 1 || "$JSON" -eq 1 ]] && return 0
  [[ -r /dev/tty && -w /dev/tty ]] || return 0
  PAUSE_DONE=1
  printf '\nPulsa una tecla para cerrar.\n' >/dev/tty
  read -r -n 1 -s _sddia_key </dev/tty || true
}
trap pause_for_close EXIT

usage() {
  cat <<'EOF'
Uso: SddIA/scripts/qa/clean-project-ephemeral.sh [opciones]

Inventario (defecto) o purga de volátiles regenerables del repo.
No toca .git, events.redb, vector_store, proofs, services ni
SddIA/target/native-debug-bins. Generaciones viejas en debug/deps y
release/deps solo con --with-cargo-deps (AUT-79). No usa repo-retention-purger.

Opciones:
  --repo PATH            Raíz del repositorio (defecto: padre de SddIA/)
  --apply                Ejecuta el borrado. Sin esta bandera solo informa.
  --json                 Resumen en una línea JSON. No espera tecla.
  --no-wait              No espera tecla aunque haya consola.
  --with-daemon-logs     Incluye .SddIA/daemons/logs más viejos que --daemon-log-days
  --daemon-log-days N    Antigüedad mínima de logs de daemon (defecto: 14)
  --scratch-hours N      Antigüedad mínima de scratch (defecto: 168)
  --with-events-fs       JSON residual bajo .events/ solo si el store efectivo es redb
  --with-cargo-deps      Generaciones Cargo obsoletas en SddIA/target/*/deps (AUT-79)
  -h, --help             Esta ayuda

Slices por defecto (inventario y, con --apply, borrado):
  workspaces-empty       Directorios de ejecución vacíos bajo .SddIA/workspaces/
  runtime-logs           .SddIA/runtime/*.log
  scratch                .tmp/ tmp/ docs/tmp/ más viejos que --scratch-hours
EOF
}

die() {
  echo "clean-project-ephemeral: $*" >&2
  exit 1
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --repo)
      [[ $# -ge 2 ]] || die "--repo exige PATH"
      REPO="$(cd "$2" && pwd)"
      shift 2
      ;;
    --apply) APPLY=1; shift ;;
    --json) JSON=1; shift ;;
    --no-wait) NO_WAIT=1; shift ;;
    --with-daemon-logs) WITH_DAEMON_LOGS=1; shift ;;
    --daemon-log-days)
      [[ $# -ge 2 ]] || die "--daemon-log-days exige N"
      DAEMON_LOG_DAYS="$2"
      shift 2
      ;;
    --scratch-hours)
      [[ $# -ge 2 ]] || die "--scratch-hours exige N"
      SCRATCH_HOURS="$2"
      shift 2
      ;;
    --with-events-fs) WITH_EVENTS_FS=1; shift ;;
    --with-cargo-deps) WITH_CARGO_DEPS=1; shift ;;
    -h|--help) usage; exit 0 ;;
    *) die "opción desconocida: $1" ;;
  esac
done

[[ "$DAEMON_LOG_DAYS" =~ ^[0-9]+$ ]] || die "--daemon-log-days debe ser entero >= 0"
[[ "$SCRATCH_HOURS" =~ ^[0-9]+$ ]] || die "--scratch-hours debe ser entero >= 0"
[[ -f "$REPO/SddIA/core/cumulo.paths.json" ]] || die "no es un repo SddIA: $REPO"

WORKSPACES="$REPO/.SddIA/workspaces"
RUNTIME="$REPO/.SddIA/runtime"
DAEMON_LOGS="$REPO/.SddIA/daemons/logs"
EVENTS="$REPO/.events"

WS_COUNT=0
LOG_COUNT=0
SCRATCH_COUNT=0
DAEMON_COUNT=0
EVENTS_COUNT=0
CARGO_COUNT=0
CARGO_BYTES=0
WS_ACTION="skip"
LOG_ACTION="skip"
SCRATCH_ACTION="skip"
DAEMON_ACTION="skip"
EVENTS_ACTION="skip"
CARGO_ACTION="skip"

under_repo() {
  local target="$1"
  local real
  real="$(realpath -m "$target")"
  [[ "$real" == "$REPO" || "$real" == "$REPO/"* ]]
}

refuse_protected() {
  local rel="$1"
  case "$rel" in
    .git/*|.git) return 0 ;;
    .SddIA/runtime/events.redb|.SddIA/runtime/events.redb.*) return 0 ;;
    .SddIA/vector_store|.SddIA/vector_store/*) return 0 ;;
    .SddIA/proofs|.SddIA/proofs/*) return 0 ;;
    .SddIA/services|.SddIA/services/*) return 0 ;;
    SddIA/target/native-debug-bins|SddIA/target/native-debug-bins/*) return 0 ;;
    .events/processing|.events/processing/*) return 0 ;;
    .SddIA/daemons/state|.SddIA/daemons/state/*) return 0 ;;
  esac
  return 1
}

store_mode() {
  if [[ -n "${SDDIA_EVENT_STORE:-}" ]]; then
    printf '%s' "$SDDIA_EVENT_STORE"
    return
  fi
  local envf line
  for envf in "$REPO/.SddIA/.dev/.env" "$REPO/.dev/.env"; do
    [[ -f "$envf" ]] || continue
    line="$(grep -E '^SDDIA_EVENT_STORE=' "$envf" | tail -n 1 || true)"
    if [[ -n "$line" ]]; then
      printf '%s' "${line#SDDIA_EVENT_STORE=}"
      return
    fi
  done
  printf '%s' "redb"
}

cargo_build_active_on_target() {
  local target target_canon
  target="$REPO/SddIA/target"
  target_canon="$(realpath -m "$target")"
  [[ -d /proc ]] || return 2
  shopt -s nullglob
  shopt -u nullglob
  local pid comm raw cmd lower env_raw line dir canon
  for entry in /proc/[0-9]*/; do
    [[ -d "$entry" ]] || continue
    pid="${entry#/proc/}"
    pid="${pid%/}"
    [[ "$pid" == "$$" ]] && continue
    comm="$entry/comm"
    [[ -r "$comm" ]] || continue
    [[ "$(cat "$comm" 2>/dev/null || true)" == "cargo" || "$(cat "$comm" 2>/dev/null || true)" == "rustc" ]] || continue
    raw="$(tr '\0' ' ' <"$entry/cmdline" 2>/dev/null || true)"
    cmd="$raw"
    lower="$(printf '%s' "$cmd" | tr '[:upper:]' '[:lower:]')"
    if [[ -r "$entry/environ" ]]; then
      env_raw="$(tr '\0' '\n' <"$entry/environ" 2>/dev/null || true)"
      line="$(printf '%s\n' "$env_raw" | grep -E '^CARGO_TARGET_DIR=' | tail -n 1 || true)"
      if [[ -n "$line" ]]; then
        dir="${line#CARGO_TARGET_DIR=}"
        canon="$(realpath -m "$dir" 2>/dev/null || printf '%s' "$dir")"
        if [[ "$canon" == "$target_canon" || "$canon" == "$target_canon/"* ]]; then
          return 0
        fi
      fi
    fi
    if [[ "$lower" == *sddia/target* || "$cmd" == *SddIA/target* ]]; then
      return 0
    fi
  done
  return 1
}

format_gib() {
  local bytes="$1"
  awk -v b="$bytes" 'BEGIN { if (b < 1073741824) printf "%.2f MiB", b/1048576; else printf "%.2f GiB", b/1073741824 }'
}

watcher_active() {
  local comm pid
  [[ -d /proc ]] || return 2
  shopt -s nullglob
  local comms=(/proc/[0-9]*/comm)
  shopt -u nullglob
  [[ "${#comms[@]}" -gt 0 ]] || return 2
  for comm in "${comms[@]}"; do
    [[ -r "$comm" ]] || continue
    pid="${comm#/proc/}"
    pid="${pid%/comm}"
    [[ "$pid" == "$$" ]] && continue
    if [[ "$(cat "$comm" 2>/dev/null || true)" == "event-watcher" ]]; then
      return 0
    fi
  done
  return 1
}

resolve_qa() {
  if [[ -n "${SDDIA_QA_BIN:-}" && -x "$SDDIA_QA_BIN" ]]; then
    printf '%s' "$SDDIA_QA_BIN"
    return 0
  fi
  local candidate
  for candidate in \
    "$REPO/SddIA/target/release/sddia-qa" \
    "$REPO/SddIA/target/debug/sddia-qa"
  do
    if [[ -x "$candidate" ]]; then
      printf '%s' "$candidate"
      return 0
    fi
  done
  return 1
}

prune_empty_workspaces() {
  local qa pruned=0 process exec_dir
  if qa="$(resolve_qa)"; then
    if [[ "$APPLY" -eq 1 ]]; then
      local out
      out="$(cd "$REPO" && "$qa" workspace-prune --empty --json)"
      pruned="$(printf '%s' "$out" | sed -n 's/.*"pruned_count":[[:space:]]*\([0-9][0-9]*\).*/\1/p' | head -n 1)"
      [[ -n "$pruned" ]] || die "sddia-qa no devolvió pruned_count"
      WS_COUNT="$pruned"
      WS_ACTION="pruned"
      return
    fi
  fi
  [[ -d "$WORKSPACES" ]] || { WS_COUNT=0; WS_ACTION="absent"; return; }
  shopt -s nullglob
  for process in "$WORKSPACES"/*/; do
    [[ -d "$process" ]] || continue
    for exec_dir in "$process"*/; do
      [[ -d "$exec_dir" ]] || continue
      if [[ -z "$(ls -A "$exec_dir")" ]]; then
        pruned=$((pruned + 1))
        if [[ "$APPLY" -eq 1 ]]; then
          rmdir "$exec_dir"
        fi
      fi
    done
  done
  shopt -u nullglob
  WS_COUNT="$pruned"
  if [[ "$pruned" -eq 0 ]]; then
    WS_ACTION="empty"
  elif [[ "$APPLY" -eq 1 ]]; then
    WS_ACTION="pruned"
  else
    WS_ACTION="would-prune"
  fi
}

collect_files() {
  local path
  for path in "$@"; do
    [[ -e "$path" ]] || continue
    under_repo "$path" || die "fuera del repo: $path"
    local rel="${path#"$REPO"/}"
    if refuse_protected "$rel"; then
      continue
    fi
    printf '%s\n' "$path"
  done
}

prune_runtime_logs() {
  [[ -d "$RUNTIME" ]] || { LOG_COUNT=0; LOG_ACTION="absent"; return; }
  local files=() path
  shopt -s nullglob
  for path in "$RUNTIME"/*.log; do
    files+=("$path")
  done
  shopt -u nullglob
  mapfile -t files < <(collect_files "${files[@]+"${files[@]}"}")
  LOG_COUNT="${#files[@]}"
  if [[ "$APPLY" -eq 1 && "$LOG_COUNT" -gt 0 ]]; then
    rm -f -- "${files[@]}"
    LOG_ACTION="pruned"
  elif [[ "$LOG_COUNT" -eq 0 ]]; then
    LOG_ACTION="empty"
  else
    LOG_ACTION="would-prune"
  fi
}

prune_scratch() {
  local roots=("$REPO/.tmp" "$REPO/tmp" "$REPO/docs/tmp")
  local root files=() path
  local minutes=$((SCRATCH_HOURS * 60))
  for root in "${roots[@]}"; do
    [[ -d "$root" ]] || continue
    under_repo "$root" || die "scratch fuera del repo: $root"
    while IFS= read -r -d '' path; do
      files+=("$path")
    done < <(find "$root" -type f -mmin "+$minutes" -print0)
  done
  if [[ "${#files[@]}" -gt 0 ]]; then
    mapfile -t files < <(collect_files "${files[@]}")
  fi
  SCRATCH_COUNT="${#files[@]}"
  if [[ "$APPLY" -eq 1 && "$SCRATCH_COUNT" -gt 0 ]]; then
    rm -f -- "${files[@]}"
    SCRATCH_ACTION="pruned"
  elif [[ "$SCRATCH_COUNT" -eq 0 ]]; then
    SCRATCH_ACTION="empty"
  else
    SCRATCH_ACTION="would-prune"
  fi
}

prune_daemon_logs() {
  if [[ "$WITH_DAEMON_LOGS" -ne 1 ]]; then
    DAEMON_ACTION="opt-in"
    DAEMON_COUNT=0
    return
  fi
  [[ -d "$DAEMON_LOGS" ]] || { DAEMON_COUNT=0; DAEMON_ACTION="absent"; return; }
  local files=() path
  while IFS= read -r -d '' path; do
    files+=("$path")
  done < <(find "$DAEMON_LOGS" -type f -mtime "+$DAEMON_LOG_DAYS" -print0)
  if [[ "${#files[@]}" -gt 0 ]]; then
    mapfile -t files < <(collect_files "${files[@]}")
  fi
  DAEMON_COUNT="${#files[@]}"
  if [[ "$APPLY" -eq 1 && "$DAEMON_COUNT" -gt 0 ]]; then
    rm -f -- "${files[@]}"
    DAEMON_ACTION="pruned"
  elif [[ "$DAEMON_COUNT" -eq 0 ]]; then
    DAEMON_ACTION="empty"
  else
    DAEMON_ACTION="would-prune"
  fi
}

prune_events_fs() {
  if [[ "$WITH_EVENTS_FS" -ne 1 ]]; then
    EVENTS_ACTION="opt-in"
    EVENTS_COUNT=0
    return
  fi
  local mode
  mode="$(store_mode)"
  if [[ "$mode" != "redb" ]]; then
    EVENTS_ACTION="skipped-store-$mode"
    EVENTS_COUNT=0
    return
  fi
  local watch_rc=0
  watcher_active || watch_rc=$?
  if [[ "$watch_rc" -eq 0 ]]; then
    EVENTS_ACTION="skipped-watcher-active"
    EVENTS_COUNT=0
    return
  fi
  if [[ "$watch_rc" -eq 2 ]]; then
    EVENTS_ACTION="skipped-proc-unreadable"
    EVENTS_COUNT=0
    return
  fi
  [[ -d "$EVENTS" ]] || { EVENTS_COUNT=0; EVENTS_ACTION="absent"; return; }
  local rel files=() path
  local allow=(
    pending
    processed
    telemetry
    domain
    orchestration
    progress
  )
  for rel in "${allow[@]}"; do
    [[ -d "$EVENTS/$rel" ]] || continue
    while IFS= read -r -d '' path; do
      files+=("$path")
    done < <(find "$EVENTS/$rel" -type f -name '*.json' -print0)
  done
  if [[ "${#files[@]}" -gt 0 ]]; then
    mapfile -t files < <(collect_files "${files[@]}")
  fi
  EVENTS_COUNT="${#files[@]}"
  if [[ "$APPLY" -eq 1 && "$EVENTS_COUNT" -gt 0 ]]; then
    rm -f -- "${files[@]}"
    EVENTS_ACTION="pruned"
  elif [[ "$EVENTS_COUNT" -eq 0 ]]; then
    EVENTS_ACTION="empty"
  else
    EVENTS_ACTION="would-prune"
  fi
}

prune_cargo_deps() {
  if [[ "$WITH_CARGO_DEPS" -ne 1 ]]; then
    CARGO_ACTION="opt-in"
    CARGO_COUNT=0
    CARGO_BYTES=0
    return
  fi
  [[ -f "$CARGO_HELPER" ]] || die "falta $CARGO_HELPER"
  local cargo_rc=0
  cargo_build_active_on_target || cargo_rc=$?
  if [[ "$cargo_rc" -eq 0 ]]; then
    CARGO_ACTION="skipped-cargo-active"
    CARGO_COUNT=0
    CARGO_BYTES=0
    return
  fi
  if [[ "$cargo_rc" -eq 2 ]]; then
    CARGO_ACTION="skipped-proc-unreadable"
    CARGO_COUNT=0
    CARGO_BYTES=0
    return
  fi
  local py_out apply_flag=()
  [[ "$APPLY" -eq 1 ]] && apply_flag=(--apply)
  py_out="$(python3 "$CARGO_HELPER" "$REPO" "${apply_flag[@]+"${apply_flag[@]}"}")"
  CARGO_COUNT="$(printf '%s' "$py_out" | sed -n 's/.*"cargo_deps":[[:space:]]*\([0-9][0-9]*\).*/\1/p' | head -n 1)"
  CARGO_BYTES="$(printf '%s' "$py_out" | sed -n 's/.*"cargo_deps_bytes":[[:space:]]*\([0-9][0-9]*\).*/\1/p' | head -n 1)"
  CARGO_ACTION="$(printf '%s' "$py_out" | sed -n 's/.*"cargo_deps_action":[[:space:]]*"\([^"]*\)".*/\1/p' | head -n 1)"
  [[ -n "$CARGO_COUNT" && -n "$CARGO_ACTION" ]] || die "salida inválida de cargo-deps: $py_out"
  CARGO_BYTES="${CARGO_BYTES:-0}"
}

prune_empty_workspaces
prune_runtime_logs
prune_scratch
prune_daemon_logs
prune_events_fs
prune_cargo_deps

MODE="dry-run"
[[ "$APPLY" -eq 1 ]] && MODE="apply"

if [[ "$JSON" -eq 1 ]]; then
  printf '{"mode":"%s","repo":"%s","workspaces_empty":%s,"workspaces_action":"%s","runtime_logs":%s,"runtime_logs_action":"%s","scratch":%s,"scratch_action":"%s","daemon_logs":%s,"daemon_logs_action":"%s","events_fs":%s,"events_fs_action":"%s","cargo_deps":%s,"cargo_deps_bytes":%s,"cargo_deps_action":"%s"}\n' \
    "$MODE" "$REPO" \
    "$WS_COUNT" "$WS_ACTION" \
    "$LOG_COUNT" "$LOG_ACTION" \
    "$SCRATCH_COUNT" "$SCRATCH_ACTION" \
    "$DAEMON_COUNT" "$DAEMON_ACTION" \
    "$EVENTS_COUNT" "$EVENTS_ACTION" \
    "$CARGO_COUNT" "$CARGO_BYTES" "$CARGO_ACTION"
else
  action_es() {
    case "$1" in
      would-prune) printf 'se borrarían' ;;
      pruned) printf 'borrados' ;;
      empty) printf 'nada que borrar' ;;
      absent) printf 'ruta ausente' ;;
      opt-in) printf 'omitido' ;;
      skipped-watcher-active) printf 'omitido: event-watcher activo' ;;
      skipped-proc-unreadable) printf 'omitido: /proc no legible' ;;
      skipped-cargo-active) printf 'omitido: cargo/rustc activo' ;;
      hold-incomplete) printf 'omitido: generación incompleta' ;;
      skipped-store-*) printf 'omitido: store %s' "${1#skipped-store-}" ;;
      *) printf '%s' "$1" ;;
    esac
  }
  if [[ "$APPLY" -eq 1 ]]; then
    printf 'Resultado: purga aplicada\n'
  else
    printf 'Resultado: inventario (no se ha borrado nada)\n'
  fi
  printf 'Repositorio: %s\n\n' "$REPO"
  printf '  %-22s %6s   %s\n' "workspaces vacíos" "$WS_COUNT" "$(action_es "$WS_ACTION")"
  printf '  %-22s %6s   %s\n' "logs de runtime" "$LOG_COUNT" "$(action_es "$LOG_ACTION")"
  printf '  %-22s %6s   %s\n' "scratch (>${SCRATCH_HOURS} h)" "$SCRATCH_COUNT" "$(action_es "$SCRATCH_ACTION")"
  printf '  %-22s %6s   %s\n' "logs de daemon (>${DAEMON_LOG_DAYS} d)" "$DAEMON_COUNT" "$(action_es "$DAEMON_ACTION")"
  printf '  %-22s %6s   %s\n' "residuo .events" "$EVENTS_COUNT" "$(action_es "$EVENTS_ACTION")"
  printf '  %-22s %6s   %s (%s)\n' "deps Cargo (hash viejo)" "$CARGO_COUNT" "$(action_es "$CARGO_ACTION")" "$(format_gib "$CARGO_BYTES")"
fi
