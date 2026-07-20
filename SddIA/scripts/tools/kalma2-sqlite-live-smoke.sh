#!/usr/bin/env bash
# HOST-D — SQLite live bajo L-WAL: backup consistente de state.vscdb real + write + verify.
# No escribe la DB live con Cursor abierto (contienda WAL); evidencia sobre copia del host.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$ROOT"
PY="$ROOT/SddIA/scripts/tools/kalma2-agent-runtime-cursor.py"
SRC="${SDDIA_CURSOR_VSCDB_SRC:-$HOME/.config/Cursor/User/globalStorage/state.vscdb}"
TD="$(mktemp -d /tmp/kalma2-sqlite-live-XXXXXX)"
DB="$TD/state.vscdb"

if [[ ! -f "$SRC" ]]; then
  echo "FAIL: no hay DB host en $SRC" >&2
  exit 1
fi

echo "=== HOST-D backup L-WAL desde $SRC ==="
if command -v sqlite3 >/dev/null 2>&1; then
  sqlite3 "$SRC" ".backup '$DB'"
else
  python3 - <<PY
import sqlite3
src, dst = "$SRC", "$DB"
src_con = sqlite3.connect(f"file:{src}?mode=ro", uri=True)
dst_con = sqlite3.connect(dst)
src_con.backup(dst_con)
dst_con.close()
src_con.close()
print("python backup ok")
PY
fi

export PATH="${HOME}/.local/bin:${PATH}"
export SDDIA_CURSOR_VSCDB="$DB"
export SDDIA_CURSOR_SQLITE_WRITE=1
export SDDIA_LLM_REQUIRE_INFER=1
unset SDDIA_LLM_CHAT_MOCK SDDIA_AGENT_RUNTIME_MOCK SDDIA_CURSOR_IDE_WATCH_ONLY || true
# Prefer vault; fallback autodetection (--trust inyectado en prótesis)
if [[ -z "${SDDIA_LLM_INFER_COMMAND:-}" ]]; then
  export SDDIA_LLM_INFER_COMMAND="${HOME}/.local/bin/cursor-agent --print --mode ask --trust"
fi

echo "=== CHAT_STREAM live-infer + sqlite write (copia host) ==="
out="$(printf '%s' '{"operation":"CHAT_STREAM","prompt":"HOST-D di solo: hostd-ok","repo_root":"'"$ROOT"'"}' | python3 "$PY")"
echo "$out" | tail -8
echo "$out" | grep -q '\[kalma2-meta\].*"backend": "cli"'
echo "$out" | grep -q 'kalma2-sqlite ok'
echo "$out" | grep -qv 'sqlite-ack' || { echo "$out" | grep -q 'backend.: .sqlite-ack' && exit 1; true; }

python3 - <<PY
import sqlite3, json
db = "$DB"
con = sqlite3.connect(db)
keys = [r[0] for r in con.execute(
  "SELECT key FROM cursorDiskKV WHERE key LIKE 'composerData:%' OR key LIKE 'bubbleId:%'"
)]
comp = [k for k in keys if k.startswith("composerData:")]
bub = [k for k in keys if k.startswith("bubbleId:")]
assert comp, "sin composerData"
assert len(bub) >= 2, f"bubbles={len(bub)}"
# Composer Kalma2 HOST-D
found = False
for k in reversed(comp):
    cid = k.split(":", 1)[1]
    raw = con.execute("SELECT value FROM cursorDiskKV WHERE key=?", (f"composerData:{cid}",)).fetchone()[0]
    doc = json.loads(raw)
    if "Kalma2" in (doc.get("name") or "") and "HOST-D" in (doc.get("name") or ""):
        found = True
        print("HOST-D composer OK", cid[:8], doc.get("name")[:60])
        break
assert found, "no composer Kalma2 HOST-D"
print("HOST-D AC8 live-copy OK")
PY

echo "HOST-D smoke OK ($DB)"
