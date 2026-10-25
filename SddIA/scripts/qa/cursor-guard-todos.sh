#!/usr/bin/env bash
# Cursor preToolUse: bloquea escrituras en cola docs/todos cuando tracker.provider=linear.
set -euo pipefail
ROOT=$(git rev-parse --show-toplevel 2>/dev/null || pwd)
# shellcheck source=git-hooks/hook_common.sh
source "$ROOT/SddIA/scripts/qa/git-hooks/hook_common.sh"

allow() {
  printf '%s\n' '{"permission":"allow"}'
  exit 0
}

deny() {
  local msg
  msg=$(linear_todos_intake_blocked_message | tr '\n' ' ')
  python3 -c 'import json,sys; print(json.dumps({"permission":"deny","user_message":sys.argv[1],"agent_message":sys.argv[2]}))' \
    "$msg" \
    "Con provider linear usa forge-pbi / refine-pbi en Linear; no escribas en docs/todos/pending ni docs/todos/done."
  exit 2
}

if ! tracker_provider_is_linear; then
  allow
fi

input=$(cat)
path=$(printf '%s' "$input" | python3 -c '
import json, sys
raw = sys.stdin.read()
try:
    doc = json.loads(raw)
except json.JSONDecodeError:
    sys.exit(0)
ti = doc.get("tool_input") or doc.get("input") or {}
if isinstance(ti, str):
    try:
        ti = json.loads(ti)
    except json.JSONDecodeError:
        ti = {}
if not isinstance(ti, dict):
    sys.exit(0)
for key in ("path", "file_path", "target_file", "filePath"):
    v = ti.get(key)
    if isinstance(v, str) and v.strip():
        print(v.strip())
        sys.exit(0)
' || true)

[[ -n "${path:-}" ]] || allow

if path_under_todos_bucket "$path"; then
  deny
fi

allow
