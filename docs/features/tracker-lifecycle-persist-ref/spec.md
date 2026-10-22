---
feature_name: tracker-lifecycle-persist-ref
created: "2026-10-22"
process: feature
base: main
scope: traza persist_ref por Work_Initiated
execution_id: "76a499d1-59eb-4b02-8e18-5586b41190f0"
document_id: PBI-ARQUITECTURA-TRACKER-LIFECYCLE-04
---

# Spec — tracker-lifecycle-persist-ref

Ficheros del techo, fijados antes de cualquier edición de genoma. Ninguno se modifica en este PBI.

| Papel | Path | Genoma |
|-------|------|--------|
| Emisor | `SddIA/engine/execute-process/src/engine/workspace_init.rs` | no |
| Consumidor | `SddIA/engine/execute-process/src/engine/handlers/tracker_stamp.rs` | no |
| Proceso | `SddIA/process/tracker-stamp.md` | sí, sin diff |

## Comportamiento

`Work_Initiated` con `tracker_ref` y `persist_ref`:

- operación `update_issue_state` hacia `in_progress` si el issue no está ya ahí;
- operación `create_comment` con la rama y el `persist_ref`;
- el workflow no usa la clave `done`.

`feature.md`, `bug-fix.md` y `refactorization.md` no contienen `tracker-operations` ni `tool:`.

Con `tracker.provider: file`, `stamp_event` responde `no-op: provider file` y no crea el jsonl de salida.

Escribir `spec.md` o `validacion.md` no es un evento propio. La señal de la forja es `Work_Initiated`, no una transición a `state_map.done`.
