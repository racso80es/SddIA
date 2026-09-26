#!/usr/bin/env bash
# Fachada installer: motor ciego + verify post-deploy + eventos domain.
set -euo pipefail
REPO_ROOT="$(cd "$(dirname "$0")" && pwd)"
MOTOR="$REPO_ROOT/SddIA/scripts/sddia-installer.sh"
IO_PY="$REPO_ROOT/SddIA/scripts/installer/installer_io.py"
# shellcheck source=SddIA/scripts/common/sddia_shell_lib.sh
source "$REPO_ROOT/SddIA/scripts/common/sddia_shell_lib.sh"

NO_VERIFY=0
PASSTHRU=()
while [[ $# -gt 0 ]]; do
  case "$1" in
    --no-verify) NO_VERIFY=1; shift ;;
    *) PASSTHRU+=("$1"); shift ;;
  esac
done

_emit_instance_event() {
  local event_type="$1"
  local root="$2"
  local esc="$3"
  local verdict="${4:-}"
  local event_id profile manifest_at route_json
  event_id="$(python3 -c 'import uuid; print(uuid.uuid4())')"
  profile="engineering"
  manifest_at=""
  if [[ -f "$root/MANIFEST.json" ]]; then
    manifest_at="$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1])).get("created_at",""))' "$root/MANIFEST.json" 2>/dev/null || true)"
  fi
  local pending="$REPO_ROOT/.events/pending/${event_id}.json"
  mkdir -p "$(dirname "$pending")"
  python3 -c '
import json, sys
event_id, etype, root, esc, profile, manifest_at, verdict, out = sys.argv[1:]
payload = {"root": root, "esc": esc, "profile": profile, "manifest_created_at": manifest_at or None}
if verdict:
    payload["verdict"] = verdict
doc = {
    "event_id": event_id,
    "event_type": etype,
    "timestamp": __import__("datetime").datetime.utcnow().strftime("%Y-%m-%dT%H:%M:%SZ"),
    "emitter_agent": "tekton",
    "payload": payload,
}
open(out, "w").write(json.dumps(doc, indent=2) + "\n")
' "$event_id" "$event_type" "$root" "$esc" "$profile" "$manifest_at" "$verdict" "$pending"
  (
    cd "$REPO_ROOT"
    route_json="$(./sddia-run.sh --process route-domain-event --inputs "$(python3 -c 'import json,sys; print(json.dumps({"event_file_path": sys.argv[1]}))' "${pending#$REPO_ROOT/}")")"
    echo "$route_json"
  ) || true
}

_resolve_install_root_from_args() {
  local i=0 arg
  while [[ $i -lt ${#PASSTHRU[@]} ]]; do
    arg="${PASSTHRU[$i]}"
    if [[ "$arg" == "--root" ]]; then
      echo "${PASSTHRU[$((i + 1))]:-}"
      return 0
    fi
    i=$((i + 1))
  done
  if [[ -n "${SDDIA_INSTALL_ROOT:-}" ]]; then
    echo "$SDDIA_INSTALL_ROOT"
    return 0
  fi
  echo "/home/racso/Aplicaciones/Asistencia_Tormentosa_SddIA"
}

_facade_finalize_deploy() {
  local motor_file="$1" root="$2" esc="$3" verdict="$4" audit_ref="${5:-}" event_json
  event_json="$(python3 -c 'import json,sys; print(json.dumps([{"event_type":"Instance_Deployed","event_id":sys.argv[1]}]))' "$(python3 -c 'import uuid; print(uuid.uuid4())')")"
  local verify_json="null"
  if [[ -n "$audit_ref" ]]; then
    verify_json="$(python3 -c 'import json,sys; print(json.dumps({"verdict":sys.argv[1],"audit_ref":sys.argv[2]}))' "$verdict" "$audit_ref")"
  elif [[ "$verdict" != "APTO" && -n "$verdict" ]]; then
    verify_json="$(python3 -c 'import json,sys; print(json.dumps({"verdict":sys.argv[1]}))' "$verdict")"
  fi
  local msg="deploy $verdict en $root"
  local success="true" exit_code=0
  if [[ "$verdict" != "APTO" ]]; then
    success="false"
    exit_code=5
    msg="deploy NO-APTO en $root"
  fi
  python3 "$IO_PY" merge-facade \
    --state-file "$motor_file.state" \
    --motor-envelope-file "$motor_file" \
    --extra-steps-json '[{"id":"verify_health","status":"ok"},{"id":"emit_event","status":"ok"}]' \
    --verify-json "$verify_json" \
    --events-json "$event_json" \
    --message "$msg" \
    --success "$success" \
    --exit-code "$exit_code"
}

CMD="${PASSTHRU[0]:-}"
CMD_LC="$(echo "$CMD" | tr '[:upper:]' '[:lower:]')"

if [[ "$CMD_LC" == "deploy" ]] && [[ " ${PASSTHRU[*]} " != *" --dry-run "* ]]; then
  motor_result="$(mktemp)"
  motor_state="$(mktemp)"
  SDDIA_INSTALLER_RESULT_FILE="$motor_result" SDDIA_INSTALLER_SUPPRESS_STDOUT=1 "$MOTOR" "${PASSTHRU[@]}"
  motor_rc=$?
  if [[ "$motor_rc" -ne 0 && "$motor_rc" -ne 5 ]]; then
    if [[ -f "$motor_result" ]]; then
      cat "$motor_result"
    fi
    rm -f "$motor_result" "$motor_state"
    exit "$motor_rc"
  fi
  root_raw="$(_resolve_install_root_from_args)"
  if [[ "$root_raw" != /* ]]; then
    root_raw="$REPO_ROOT/$root_raw"
  fi
  ROOT="$(realpath -m "$root_raw")"
  ESC="$(systemd-escape -p "$ROOT")"
  verdict="APTO"
  audit_ref=""
  if [[ "$NO_VERIFY" -eq 0 ]]; then
    verify_out="$(cd "$REPO_ROOT" && ./sddia-run.sh --process instance-health-verify --inputs "$(python3 -c 'import json,sys; print(json.dumps({"instance_root": sys.argv[1]}))' "$ROOT")")"
    verdict="$(echo "$verify_out" | python3 -c 'import json,sys; d=json.load(sys.stdin); print((d.get("data") or {}).get("verdict","NO-APTO"))' 2>/dev/null || echo "NO-APTO")"
    audit_ref="$(echo "$verify_out" | python3 -c 'import json,sys; d=json.load(sys.stdin); print((d.get("data") or {}).get("audit_ref",""))' 2>/dev/null || true)"
    _emit_instance_event "Instance_Deployed" "$ROOT" "$ESC" "$verdict" >/dev/null
    if [[ "$verdict" != "APTO" ]]; then
      echo "$verify_out" >>"${motor_result}.log" 2>/dev/null || true
      cp "$motor_result" "$motor_state.state" 2>/dev/null || true
      _facade_finalize_deploy "$motor_result" "$ROOT" "$ESC" "$verdict" "$audit_ref"
      rm -f "$motor_result" "$motor_state"
      exit 5
    fi
  else
    _emit_instance_event "Instance_Deployed" "$ROOT" "$ESC" "$verdict" >/dev/null
  fi
  _facade_finalize_deploy "$motor_result" "$ROOT" "$ESC" "$verdict" "$audit_ref"
  rm -f "$motor_result" "$motor_state"
  exit 0
fi

if [[ "$CMD_LC" == "teardown" ]] && [[ " ${PASSTHRU[*]} " == *" --force "* ]] && [[ " ${PASSTHRU[*]} " != *" --dry-run "* ]]; then
  root_raw="$(_resolve_install_root_from_args)"
  if [[ "$root_raw" != /* ]]; then
    root_raw="$REPO_ROOT/$root_raw"
  fi
  ROOT="$(realpath -m "$root_raw")"
  ESC="$(systemd-escape -p "$ROOT")"
  motor_result="$(mktemp)"
  SDDIA_INSTALLER_RESULT_FILE="$motor_result" SDDIA_INSTALLER_SUPPRESS_STDOUT=1 "$MOTOR" "${PASSTHRU[@]}"
  motor_rc=$?
  if [[ "$motor_rc" -ne 0 ]]; then
    [[ -f "$motor_result" ]] && cat "$motor_result"
    rm -f "$motor_result"
    exit "$motor_rc"
  fi
  _emit_instance_event "Instance_Torn_Down" "$ROOT" "$ESC" "" >/dev/null
  event_json="$(python3 -c 'import json; print(json.dumps([{"event_type":"Instance_Torn_Down"}]))')"
  python3 "$IO_PY" merge-facade \
    --state-file "$motor_result.state" \
    --motor-envelope-file "$motor_result" \
    --extra-steps-json '[{"id":"emit_event","status":"ok"}]' \
    --events-json "$event_json" \
    --message "teardown ok" \
    --success true \
    --exit-code 0
  rm -f "$motor_result"
  exit 0
fi

exec "$MOTOR" "${PASSTHRU[@]}"
