#!/usr/bin/env bash
# Cursor / Antigravity preToolUse: bloquea escrituras en cola docs/todos cuando tracker.provider=linear.
set -euo pipefail
ROOT=$(git rev-parse --show-toplevel 2>/dev/null || pwd)
# shellcheck source=git-hooks/hook_common.sh
source "$ROOT/SddIA/scripts/qa/git-hooks/hook_common.sh"

input=$(cat || true)

parsed=$(printf '%s' "$input" | python3 -c '
import json, sys
raw = sys.stdin.read()
try:
    doc = json.loads(raw) if raw.strip() else {}
except json.JSONDecodeError:
    doc = {}
if not isinstance(doc, dict):
    doc = {}

client = "antigravity" if ("toolCall" in doc or "hookEventName" in doc) else "cursor"
ti = {}
tc = doc.get("toolCall")
if isinstance(tc, dict) and isinstance(tc.get("args"), dict):
    ti = tc["args"]
else:
    cand = doc.get("tool_input") or doc.get("input") or {}
    if isinstance(cand, str):
        try:
            cand = json.loads(cand)
        except json.JSONDecodeError:
            cand = {}
    if isinstance(cand, dict):
        ti = cand

path = ""
for key in ("Path", "path", "file_path", "target_file", "filePath"):
    v = ti.get(key)
    if isinstance(v, str) and v.strip():
        path = v.strip()
        break

print(f"{client}\t{path}")
' || printf 'cursor\t\n')

CLIENT="${parsed%%$'\t'*}"
path="${parsed#*$'\t'}"

allow() {
  if [[ "${CLIENT:-cursor}" == "antigravity" ]]; then
    printf '%s\n' '{"decision":"allow"}'
  else
    printf '%s\n' '{"permission":"allow"}'
  fi
  exit 0
}

deny() {
  local msg agent_msg
  msg=$(linear_todos_intake_blocked_message | tr '\n' ' ')
  agent_msg="Con provider linear usa forge-pbi / refine-item en Linear; no escribas en docs/todos/pending ni docs/todos/done."
  if [[ "${CLIENT:-cursor}" == "antigravity" ]]; then
    python3 -c 'import json,sys; print(json.dumps({"decision":"deny","reason":f"{sys.argv[1]} {sys.argv[2]}"}))' \
      "$msg" "$agent_msg"
    exit 0
  else
    python3 -c 'import json,sys; print(json.dumps({"permission":"deny","user_message":sys.argv[1],"agent_message":sys.argv[2]}))' \
      "$msg" "$agent_msg"
    exit 2
  fi
}

if ! tracker_provider_is_linear; then
  allow
fi

[[ -n "${path:-}" ]] || allow

path="${path//\\//}"
path="${path#"$ROOT"/}"
path="${path#./}"

if path_under_todos_bucket "$path"; then
  deny
fi

allow
