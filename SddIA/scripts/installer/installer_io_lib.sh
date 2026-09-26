#!/usr/bin/env bash
# shellcheck shell=bash
# Helpers I/O envelope para sddia-installer (motor + fachada).
set -euo pipefail

_INST_IO_PY="${_INST_IO_PY:-}"
if [[ -z "$_INST_IO_PY" ]]; then
  _INST_IO_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
  _INST_IO_PY="${_INST_IO_SCRIPT_DIR}/installer_io.py"
fi

_io_die() {
  local msg="$1"
  local code="${2:-1}"
  local err_code="${3:-}"
  if [[ -n "${INST_IO_STATE:-}" && -f "${INST_IO_STATE:-}" ]]; then
    local ec="$err_code"
    if [[ -z "$ec" ]]; then
      case "$code" in
        2) ec="ROOT_LIVE_REQUIRES_FORCE" ;;
        3) ec="TEARDOWN_REQUIRES_FORCE" ;;
        4) ec="MAILBOX_SHARED" ;;
        5) ec="VERIFY_NOT_APTO" ;;
        6) ec="STEP_FAILED" ;;
        7) ec="REQUEST_INVALID" ;;
        *) ec="INVALID_ARGS" ;;
      esac
    fi
    python3 "$_INST_IO_PY" fail \
      --state-file "$INST_IO_STATE" \
      --exit-code "$code" \
      --message "$msg" \
      --error-code "$ec" >/dev/null 2>&1 || true
    if [[ -z "${SDDIA_INSTALLER_SUPPRESS_STDOUT:-}" ]]; then
      python3 "$_INST_IO_PY" emit --state-file "$INST_IO_STATE" --to-stdout 2>/dev/null || true
    fi
  else
    echo "[installer] ERROR: $msg" >&2
  fi
  exit "$code"
}

_io_init_session() {
  local forge="$1" command="$2" dry_run="$3" profile="${4:-full-node}"
  INST_IO_STATE="${WORK:?}/.installer/state.json"
  mkdir -p "$(dirname "$INST_IO_STATE")"
  local init_out
  init_out="$(python3 "$_INST_IO_PY" init \
    --state-file "$INST_IO_STATE" \
    --forge-root "$forge" \
    --command "$command" \
    --dry-run "$dry_run" \
    --bundle-profile "$profile" \
    --correlation-id "${SDDIA_INSTALLER_CORRELATION_ID:-}" \
    --progress-mode "${SDDIA_INSTALLER_PROGRESS:-auto}" \
    --log-dir "${SDDIA_INSTALLER_LOG_DIR:-}")"
  INST_LOG="$(echo "$init_out" | python3 -c 'import json,sys; print(json.load(sys.stdin)["log_path"])')"
  INST_LOG_REF="$(echo "$init_out" | python3 -c 'import json,sys; print(json.load(sys.stdin)["log_ref"])')"
  export INST_IO_STATE INST_LOG INST_LOG_REF
}

_io_set_context() {
  python3 "$_INST_IO_PY" set-context \
    --state-file "$INST_IO_STATE" \
    --root "$ROOT" \
    --esc "$ESC" \
    --force "$FORCE" \
    --skip-build "$SKIP_BUILD" \
    --dry-run "$DRY_RUN" \
    --bundle-profile "$BUNDLE_PROFILE" >/dev/null
}

_io_set_plan_from_emit() {
  local live_flag="${1:-0}"
  local channel_json="{}"
  if [[ -f "$WORK/vault/instance.SddIA.dev.env" ]]; then
    channel_json="$(_registry_python channel_keys "$HOST_REGISTRY" "$WORK/vault/instance.SddIA.dev.env" \
      "SDDIA_EMAIL_IMAP_SECRET,TELEGRAM_BOT_TOKEN,TELEGRAM_ALLOWED_CHAT_ID,IOTA_WALLET_SECRET,GEMINI_API_KEY,CURSOR_API_KEY")"
  fi
  local plan_json
  plan_json="$(python3 -c '
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
    "dry_run": sys.argv[14] == "1",
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
    "${PLAN_PORT_SOURCE:-}" \
    "$DRY_RUN")"
  python3 "$_INST_IO_PY" set-plan --state-file "$INST_IO_STATE" --plan-json "$plan_json" >/dev/null
}

_io_step() {
  local moment="$1" step_id="$2" status="${3:-}" reason="${4:-}"
  python3 "$_INST_IO_PY" step \
    --state-file "$INST_IO_STATE" \
    --step-id "$step_id" \
    --moment "$moment" \
    ${status:+--status "$status"} \
    ${reason:+--reason "$reason"} >/dev/null
}

_io_step_begin() { _io_step begin "$1"; }
_io_step_end() { _io_step end "$1" "${2:-ok}" "${3:-}"; }

_io_success_emit() {
  local msg="$1"
  python3 "$_INST_IO_PY" success --state-file "$INST_IO_STATE" --message "$msg" >/dev/null
  if [[ -n "${SDDIA_INSTALLER_RESULT_FILE:-}" ]]; then
    python3 "$_INST_IO_PY" emit --state-file "$INST_IO_STATE" >/dev/null
  else
    python3 "$_INST_IO_PY" emit --state-file "$INST_IO_STATE" --to-stdout
  fi
}

_io_run_step() {
  local step_id="$1"
  shift
  _io_step_begin "$step_id"
  local rc=0
  if ! "$@" >>"$INST_LOG" 2>&1; then
    rc=$?
    local tail
    tail="$(tail -n 20 "$INST_LOG" 2>/dev/null || true)"
    python3 "$_INST_IO_PY" fail \
      --state-file "$INST_IO_STATE" \
      --exit-code 6 \
      --message "Paso $step_id falló (rc=$rc)" \
      --error-code STEP_FAILED \
      --step "$step_id" \
      --child-exit "$rc" \
      --detail-tail "$tail" >/dev/null
    python3 "$_INST_IO_PY" emit --state-file "$INST_IO_STATE" --to-stdout 2>/dev/null || true
    exit 6
  fi
  _io_step_end "$step_id" ok
  return 0
}

_io_set_registry() {
  local action="$1"
  python3 "$_INST_IO_PY" set-field \
    --state-file "$INST_IO_STATE" \
    --field registry \
    --json-blob "$(python3 -c 'import json; print(json.dumps({"ref":"instance.host_registry","action":sys.argv[1]}))' "$action")" >/dev/null
}
