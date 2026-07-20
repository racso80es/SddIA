#!/usr/bin/env bash
# Smoke S3 — AGENT_PHASE no-soft (cableado). Live host: cursor-agent + REQUIRE_CLI.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$ROOT"
PY="$ROOT/SddIA/scripts/tools/kalma2-agent-runtime-cursor.py"
LAB="$ROOT/SddIA/scripts/tools/kalma2-agent-runtime-lab.sh"
chmod +x "$LAB"

payload="$(python3 - <<PY
import json
print(json.dumps({
  "operation": "AGENT_PHASE",
  "process_name": "feature",
  "phase_name": "Ejecución",
  "agents": ["tekton"],
  "persist_ref": "docs/features/kalma2-llm-live",
  "repo_root": "$ROOT",
  "correlation_id": "00000000-0000-4000-8000-0000000000s3",
}))
PY
)"

echo "=== MOCK → executed ==="
out="$(printf '%s' "$payload" | SDDIA_AGENT_RUNTIME_MOCK=1 python3 "$PY")"
echo "$out" | python3 -c 'import sys,json; d=json.load(sys.stdin); assert d["success"] and d["data"]["status"]=="executed" and d["data"]["backend"]=="mock"'

echo "=== LAB_AUTO → executed ==="
out="$(printf '%s' "$payload" | SDDIA_AGENT_RUNTIME_COMMAND="$LAB" SDDIA_AGENT_RUNTIME_LAB_AUTO=1 "$LAB")"
# lab script reads stdin itself — invoke lab directly
out="$(printf '%s' "$payload" | SDDIA_AGENT_RUNTIME_LAB_AUTO=1 bash "$LAB")"
echo "$out" | python3 -c 'import sys,json; d=json.load(sys.stdin); assert d["success"] and d["data"]["status"]=="executed"'

echo "=== CLI missing + REQUIRE_CLI → failed (no awaiting soft) ==="
set +e
out="$(printf '%s' "$payload" | env -u SDDIA_AGENT_RUNTIME_MOCK \
  SDDIA_AGENT_RUNTIME_BACKEND=cli \
  SDDIA_AGENT_RUNTIME_CLI=cursor-agent-does-not-exist-xyz \
  SDDIA_AGENT_RUNTIME_REQUIRE_CLI=1 \
  python3 "$PY" 2>/dev/null)"
rc=$?
set -e
echo "$out"
echo "$out" | python3 -c 'import sys,json; d=json.load(sys.stdin); assert d["data"]["status"]=="failed"; assert "awaiting" not in d["data"]["status"]'
test "$rc" -ne 0

echo "=== execute-process agent_runtime unit ==="
cd SddIA && CARGO_TARGET_DIR=target cargo test -p execute-process --lib agent_runtime -- --test-threads=1 >/tmp/s3-agent-rt.txt 2>&1
tail -8 /tmp/s3-agent-rt.txt
grep -E 'test result: ok\.' /tmp/s3-agent-rt.txt | grep -vq '0 passed' || grep -q '3 passed' /tmp/s3-agent-rt.txt

echo "S3 smoke OK (MOCK/LAB_AUTO executed + REQUIRE_CLI failed + unit tests)"
