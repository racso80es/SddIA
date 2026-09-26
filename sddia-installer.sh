#!/usr/bin/env bash
# Fachada installer: motor ciego + verify post-deploy + eventos domain.
set -euo pipefail
REPO_ROOT="$(cd "$(dirname "$0")" && pwd)"
MOTOR="$REPO_ROOT/SddIA/scripts/sddia-installer.sh"
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
  local event_id profile manifest_at
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
    ./sddia-run.sh --process route-domain-event --inputs "$(python3 -c 'import json,sys; print(json.dumps({"event_file_path": sys.argv[1]}))' "${pending#$REPO_ROOT/}")"
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

CMD="${PASSTHRU[0]:-}"
CMD_LC="$(echo "$CMD" | tr '[:upper:]' '[:lower:]')"

if [[ "$CMD_LC" == "deploy" ]] && [[ " ${PASSTHRU[*]} " != *" --dry-run "* ]]; then
  "$MOTOR" "${PASSTHRU[@]}"
  motor_rc=$?
  if [[ "$motor_rc" -ne 0 ]]; then
    exit "$motor_rc"
  fi
  root_raw="$(_resolve_install_root_from_args)"
  if [[ "$root_raw" != /* ]]; then
    root_raw="$REPO_ROOT/$root_raw"
  fi
  ROOT="$(realpath -m "$root_raw")"
  ESC="$(systemd-escape -p "$ROOT")"
  verdict="APTO"
  if [[ "$NO_VERIFY" -eq 0 ]]; then
    verify_out="$(cd "$REPO_ROOT" && ./sddia-run.sh --process instance-health-verify --inputs "$(python3 -c 'import json,sys; print(json.dumps({"instance_root": sys.argv[1]}))' "$ROOT")")"
    verdict="$(echo "$verify_out" | python3 -c 'import json,sys; d=json.load(sys.stdin); print((d.get("data") or {}).get("verdict","NO-APTO"))' 2>/dev/null || echo "NO-APTO")"
    if [[ "$verdict" != "APTO" ]]; then
      _emit_instance_event "Instance_Deployed" "$ROOT" "$ESC" "$verdict"
      exit 5
    fi
  fi
  _emit_instance_event "Instance_Deployed" "$ROOT" "$ESC" "$verdict"
  exit 0
fi

if [[ "$CMD_LC" == "teardown" ]] && [[ " ${PASSTHRU[*]} " == *" --force "* ]] && [[ " ${PASSTHRU[*]} " != *" --dry-run "* ]]; then
  root_raw="$(_resolve_install_root_from_args)"
  if [[ "$root_raw" != /* ]]; then
    root_raw="$REPO_ROOT/$root_raw"
  fi
  ROOT="$(realpath -m "$root_raw")"
  ESC="$(systemd-escape -p "$ROOT")"
  "$MOTOR" "${PASSTHRU[@]}"
  _emit_instance_event "Instance_Torn_Down" "$ROOT" "$ESC" ""
  exit 0
fi

exec "$MOTOR" "${PASSTHRU[@]}"
