#!/usr/bin/env bash
# kalma2-agent-runtime-lab.sh — wrapper lab para SDDIA_AGENT_RUNTIME_COMMAND (slice B).
# Lee JSON AGENT_PHASE por stdin; registra handoff bajo persist_ref; responde awaiting_agents
# (no finge executed sin Cursor/SDK). Con SDDIA_AGENT_RUNTIME_LAB_AUTO=1 marca executed
# tras escribir el handoff (solo laboratorio).
set -euo pipefail

INPUT="$(cat || true)"
if [[ -z "${INPUT// }" ]]; then
  printf '%s\n' '{"success":false,"data":null,"error":"stdin vacío"}'
  exit 1
fi

python3 - "$INPUT" <<'PY'
import json, os, sys
from datetime import datetime, timezone
from pathlib import Path

raw = sys.argv[1]
try:
    doc = json.loads(raw)
except json.JSONDecodeError as e:
    print(json.dumps({"success": False, "data": None, "error": f"JSON inválido: {e}"}))
    sys.exit(1)

repo = Path(doc.get("repo_root") or os.getcwd())
persist = (doc.get("persist_ref") or "").strip()
phase = doc.get("phase_name") or "?"
agents = doc.get("agents") or []
process = doc.get("process_name") or "?"
pbi_ref = doc.get("pbi_ref")
corr = doc.get("correlation_id")

handoff_rel = None
if persist:
    d = repo / persist
    d.mkdir(parents=True, exist_ok=True)
    handoff = d / "_agent_handoff.md"
    ts = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    block = (
        f"\n## {ts} — {phase}\n"
        f"- process: `{process}`\n"
        f"- agents: {', '.join(f'`{a}`' for a in agents) or '(ninguno)'}\n"
        f"- correlation_id: `{corr or ''}`\n"
        f"- pbi_ref: `{pbi_ref or ''}`\n"
        f"- runtime: kalma2-agent-runtime-lab\n"
    )
    if not handoff.exists():
        handoff.write_text(
            "---\n"
            "generated_by: kalma2-agent-runtime-lab\n"
            f"persist_ref: {persist}\n"
            "---\n\n# Agent handoff log\n",
            encoding="utf-8",
        )
    with handoff.open("a", encoding="utf-8") as f:
        f.write(block)
    handoff_rel = str(Path(persist) / "_agent_handoff.md")

auto = os.environ.get("SDDIA_AGENT_RUNTIME_LAB_AUTO", "").lower() in ("1", "true", "yes", "on")
status = "executed" if auto else "awaiting_agents"
msg = (
    f"Handoff registrado en {handoff_rel}" if handoff_rel else "Sin persist_ref; sin handoff en disco"
)
print(
    json.dumps(
        {
            "success": True,
            "data": {"status": status, "message": msg, "handoff_path": handoff_rel},
            "error": None,
        },
        ensure_ascii=False,
    )
)
PY
