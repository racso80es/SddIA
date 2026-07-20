#!/usr/bin/env bash
# Smoke S5 / AC2 — colapso de prótesis / watchdog SSE → System_Fracture_Detected.
# No usa kill -9 externo: el hang + SDDIA_LLM_SSE_TIMEOUT_SECS dispara el watchdog del bridge.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$ROOT"

BRIDGE="$ROOT/SddIA/target/debug/kalma2-bridge"
PY="$ROOT/SddIA/scripts/tools/kalma2-agent-runtime-cursor.py"
HANG="$ROOT/SddIA/scripts/tools/kalma2-infer-hang.sh"

if [[ ! -x "$BRIDGE" ]]; then
  (cd SddIA && CARGO_TARGET_DIR=target cargo build -p kalma2-bridge -p mayeuta-llm -q)
fi

cat > "$HANG" <<'EOS'
#!/usr/bin/env bash
cat >/dev/null || true
sleep 300
EOS
chmod +x "$HANG"

PORT="${SDDIA_CLIENT_PORT:-18765}"
export SDDIA_CLIENT_PORT="$PORT"
export SDDIA_REPO_ROOT="$ROOT"
export SDDIA_LLM_CLI_COMMAND="python3 $PY"
export SDDIA_LLM_INFER_COMMAND="$HANG"
export SDDIA_CURSOR_SQLITE_WRITE=0
export SDDIA_LLM_SSE_TIMEOUT_SECS=3
unset SDDIA_LLM_CHAT_MOCK SDDIA_AGENT_RUNTIME_MOCK || true

PENDING="$ROOT/.events/pending"
mkdir -p "$PENDING"
before="$(find "$PENDING" -name '*.json' -printf '%f\n' 2>/dev/null | sort | wc -l)"

fuser -k "${PORT}/tcp" 2>/dev/null || true
sleep 0.2
"$BRIDGE" > /tmp/kalma2-ac2-bridge.log 2>&1 &
BPID=$!
sleep 0.5

# curl hasta timeout del cliente; el watchdog del bridge (3s) debe emitir fractura
curl -sN -m 12 -X POST "http://127.0.0.1:${PORT}/api/chat" \
  -H 'Content-Type: application/json' \
  -d '{"prompt":"AC2 watchdog fracture"}' > /tmp/kalma2-ac2-sse.out 2>/dev/null || true

sleep 1.0

found=0
for f in "$PENDING"/*.json; do
  [[ -f "$f" ]] || continue
  if grep -q 'System_Fracture_Detected' "$f" 2>/dev/null && grep -q 'kalma2-bridge' "$f" 2>/dev/null; then
    if grep -Eq 'prosthetic_collapse|sse_watchdog' "$f"; then
      echo "AC2 fracture OK: $f"
      found=1
      break
    fi
  fi
done

kill "$BPID" 2>/dev/null || true
fuser -k "${PORT}/tcp" 2>/dev/null || true
# limpiar hang residual
pkill -f 'kalma2-infer-hang.sh' 2>/dev/null || true
rm -f "$HANG"

if [[ "$found" -ne 1 ]]; then
  echo "FAIL: no System_Fracture_Detected (sse_watchdog|prosthetic_collapse)"
  echo "pending count before≈$before after=$(find "$PENDING" -name '*.json' 2>/dev/null | wc -l)"
  tail -40 /tmp/kalma2-ac2-bridge.log || true
  exit 1
fi
echo "S5/AC2 smoke OK (watchdog timeout → fracture)"
