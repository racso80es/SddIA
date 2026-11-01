---
feature_name: refine-item-mayeuta-blocked
created: "2026-11-01"
process: bug-fix
branch_name: Fix_Events_Multiple
tracker_ref: AUT-34
execution_id: "9913ec04-8f19-4e24-920a-e689bf213c8b"
---

# Implementación — AUT-34

## Touchpoints

| Área | Cambio |
|------|--------|
| `kalma2-agent-runtime-cursor.py` | Handoff Mayeuta en prompt; workspace MCP en `~/.cache/sddia-agent-mcp/{execution_id}`; `--workspace` en CLI; humo `mcp list`; parser Markdown; `transcript.md` sin `persist_ref` |
| `agent_runtime.rs` | Mismo root MCP por `execution_id`; `workspace_server_binary` por mtime |
| `refine_item/run.rs` | Detalle `error`/`message`; fractura; `executed` sin `verdict.json` → `failed` |
| `test_kalma2_runtime_timeout.py` | Caso `Veredicto: **blocked**` |
| `SddIA/evolution/a4b2c3d1-e5f6-4789-a012-3456789abcd0.md` | Trazabilidad AUT-34 |

## Tareas plan (T1–T4)

T1–T4 cubiertas en el diff anterior. Relevo `SDDIA_AGENT_RELAY_IDE=1` sin cambios.
