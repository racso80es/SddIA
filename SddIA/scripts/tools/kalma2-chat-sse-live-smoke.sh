#!/usr/bin/env bash
# HOST-B2 — SSE live vía kalma2-bridge POST /api/chat (bóveda + cursor-agent).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$ROOT"
# shellcheck source=/dev/null
source "$ROOT/SddIA/scripts/common/sddia_shell_lib.sh"
_sddia_load_vault "$ROOT"
export PATH="${HOME}/.local/bin:${PATH}"
unset SDDIA_LLM_CHAT_MOCK SDDIA_AGENT_RUNTIME_MOCK SDDIA_CURSOR_IDE_WATCH_ONLY || true
export SDDIA_LLM_REQUIRE_INFER="${SDDIA_LLM_REQUIRE_INFER:-1}"
export SDDIA_CURSOR_SQLITE_WRITE="${SDDIA_CURSOR_SQLITE_WRITE:-0}"
export SDDIA_CLIENT_PORT="${SDDIA_CLIENT_PORT:-8765}"
export SDDIA_REPO_ROOT="$ROOT"

BRIDGE="$ROOT/SddIA/target/debug/kalma2-bridge"
if [[ ! -x "$BRIDGE" ]]; then
  (cd SddIA && CARGO_TARGET_DIR=target cargo build -p kalma2-bridge -p mayeuta-llm -q)
fi

# Liberar puerto si hay instancia huérfana de smoke anterior
if ss -ltn 2>/dev/null | grep -q ":${SDDIA_CLIENT_PORT} "; then
  echo "WARN: puerto ${SDDIA_CLIENT_PORT} ocupado — usando 18765"
  export SDDIA_CLIENT_PORT=18765
fi

"$BRIDGE" >"/tmp/kalma2-bridge-live-sse.log" 2>&1 &
BPID=$!
cleanup() { kill "$BPID" 2>/dev/null || true; }
trap cleanup EXIT

for i in $(seq 1 40); do
  if curl -sf "http://127.0.0.1:${SDDIA_CLIENT_PORT}/api/status" >/dev/null 2>&1 \
    || curl -sf "http://127.0.0.1:${SDDIA_CLIENT_PORT}/" >/dev/null 2>&1; then
    break
  fi
  sleep 0.25
done

echo "=== POST /api/chat SSE ==="
out="$(curl -sS -N -X POST "http://127.0.0.1:${SDDIA_CLIENT_PORT}/api/chat" \
  -H 'Content-Type: application/json' \
  -H 'Accept: text/event-stream' \
  --max-time 120 \
  -d '{"prompt":"responde solo: sse-live-ok","repo_root":"'"$ROOT"'"}' || true)"
echo "$out" | tail -30 | tee /tmp/kalma2-sse-live.out

echo "$out" | grep -q 'data:'
# frames SSE deben incluir meta cli o tokens reales
echo "$out" | grep -Eiq 'backend.: .cli.|sse-live-ok|kalma2-meta' 
echo "$out" | grep -Eivq 'sqlite-ack|infer-lab' || {
  echo "$out" | grep -qi 'sqlite-ack' && exit 1
  true
}
echo "HOST-B2 SSE live OK (port=${SDDIA_CLIENT_PORT})"
