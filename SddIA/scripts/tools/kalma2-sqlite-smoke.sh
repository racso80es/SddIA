#!/usr/bin/env bash
# Smoke S4 — prótesis SQLite (copia segura; L-WAL).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$ROOT"
PY="$ROOT/SddIA/scripts/tools/kalma2-agent-runtime-cursor.py"
LAB="$ROOT/SddIA/scripts/tools/kalma2-llm-infer-lab.sh"
chmod +x "$LAB"

TD="$(mktemp -d /tmp/kalma2-sqlite-smoke-XXXXXX)"
DB="$TD/state.vscdb"
SRC="${SDDIA_CURSOR_VSCDB_SRC:-$HOME/.config/Cursor/User/globalStorage/state.vscdb}"

if [[ -f "$SRC" ]]; then
  echo "=== backup desde $SRC ==="
  sqlite3 "$SRC" ".backup '$DB'" || {
    # fallback: copia cruda si sqlite3 backup falla
    cp -f "$SRC" "$DB"
  }
else
  echo "=== schema mínimo (sin DB host) ==="
  python3 - <<PY
import sqlite3
con = sqlite3.connect("$DB")
con.executescript("""
CREATE TABLE ItemTable (key TEXT PRIMARY KEY, value TEXT);
CREATE TABLE cursorDiskKV (key TEXT PRIMARY KEY, value TEXT);
CREATE TABLE composerHeaders (
  composerId TEXT PRIMARY KEY, workspaceId TEXT, createdAt INTEGER, lastUpdatedAt INTEGER,
  isArchived INTEGER, isSubagent INTEGER, recency INTEGER, checkpointAt INTEGER, value TEXT
);
INSERT INTO ItemTable VALUES ('composer.composerHeaders', '{"allComposers":[]}');
""")
con.commit(); con.close()
PY
fi

export SDDIA_CURSOR_VSCDB="$DB"
export SDDIA_CURSOR_SQLITE_WRITE=1
export SDDIA_LLM_INFER_COMMAND="$LAB"
unset SDDIA_LLM_CHAT_MOCK SDDIA_AGENT_RUNTIME_MOCK || true

echo "=== CHAT_STREAM write ==="
out="$(printf '%s' '{"operation":"CHAT_STREAM","prompt":"smoke S4 sqlite persist","repo_root":"'"$ROOT"'"}' | python3 "$PY")"
echo "$out" | tail -3
echo "$out" | grep -q 'kalma2-sqlite ok'

python3 - <<PY
import sqlite3, json, sys
db = "$DB"
con = sqlite3.connect(db)
keys = [r[0] for r in con.execute("SELECT key FROM cursorDiskKV WHERE key LIKE 'composerData:%' OR key LIKE 'bubbleId:%'")]
comp = [k for k in keys if k.startswith("composerData:")]
bub = [k for k in keys if k.startswith("bubbleId:")]
assert comp, "sin composerData"
assert len(bub) >= 2, f"bubbles={len(bub)}"
# al menos un composer Kalma2 reciente
cid = comp[-1].split(":", 1)[1]
raw = con.execute("SELECT value FROM cursorDiskKV WHERE key=?", (f"composerData:{cid}",)).fetchone()[0]
doc = json.loads(raw)
assert "Kalma2" in (doc.get("name") or ""), doc.get("name")
hdr = con.execute("SELECT value FROM ItemTable WHERE key='composer.composerHeaders'").fetchone()
assert hdr and "Kalma2" in hdr[0]
print("AC8 keys OK", "composer", cid[:8], "bubbles", len(bub))
PY

echo "S4 smoke OK ($DB)"
# no borrar DB: útil para inspección; tmp se limpia en reboot
