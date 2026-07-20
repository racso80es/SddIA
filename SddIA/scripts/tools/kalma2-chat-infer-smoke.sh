#!/usr/bin/env bash
# Smoke S1/S2 — inferencia ≠ sqlite-ack / mock (cableado).
# Live real: instalar Cursor Agent CLI y usar cursor-agent --print (ver runbook-infer.md).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$ROOT"
PY="$ROOT/SddIA/scripts/tools/kalma2-agent-runtime-cursor.py"
LAB="$ROOT/SddIA/scripts/tools/kalma2-llm-infer-lab.sh"
chmod +x "$LAB"

export SDDIA_LLM_INFER_COMMAND="$LAB"
unset SDDIA_LLM_CHAT_MOCK SDDIA_AGENT_RUNTIME_MOCK || true
export SDDIA_CURSOR_SQLITE_WRITE=0
export SDDIA_LLM_REQUIRE_INFER=1

out="$(printf '%s' '{"operation":"CHAT_STREAM","prompt":"ping S1 infer","repo_root":"'"$ROOT"'"}' | python3 "$PY")"
echo "$out"
echo "$out" | grep -q '\[kalma2-meta\].*"backend": "cli"'
echo "$out" | grep -q '\[infer-lab\]'
echo "$out" | grep -qv 'sqlite-ack' || {
  # meta no debe ser sqlite-ack; el cuerpo tampoco
  echo "$out" | grep -q 'backend.: .sqlite-ack' && exit 1
  true
}
# REQUIRE_INFER sin CLI debe fallar
unset SDDIA_LLM_INFER_COMMAND SDDIA_AGENT_RUNTIME_CLI SDDIA_LLM_CLI_COMMAND || true
set +e
printf '%s' '{"operation":"CHAT_STREAM","prompt":"fail","repo_root":"'"$ROOT"'"}' | SDDIA_LLM_REQUIRE_INFER=1 python3 "$PY"
rc=$?
set -e
test "$rc" -ne 0
echo "S1/S2 smoke OK (lab infer + require-infer fail)"
