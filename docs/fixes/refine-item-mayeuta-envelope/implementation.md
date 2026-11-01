---
feature_name: refine-item-mayeuta-envelope
created: "2026-11-01"
process: bug-fix
branch_name: Fix_Events_Multiple
tracker_ref: AUT-37
execution_id: "1f6f7316-ed62-4ded-922a-6b95fbb9d44a"
---

# Implementación — AUT-37

## Touchpoints

| Área | Cambio |
|------|--------|
| `agent_runtime.rs` | `data.message` → `entry.error` si `blocked`/`failed` y `body.error` vacío; test `blocked_propagates_data_message_to_entry_error` |
| `kalma2-agent-runtime-cursor.py` | `write_workspace_transcript` en camino `not ok` de `run_agent_phase` |
| `test_kalma2_runtime_timeout.py` | Tres casos de `transcript.md` sin persist |
| `SddIA/evolution/c3d4e5f6-a7b8-4901-8c2d-3e4f5a6b7c8d.md` | Trazabilidad AUT-37 |

## Tareas plan (T1–T3)

T1–T3 aplicadas en esta sesión.
