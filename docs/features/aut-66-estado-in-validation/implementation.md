---
feature_name: aut-66-estado-in-validation
process: refactorization
created: "2026-11-03"
pbi_ref: AUT-66
---

# Implementación — AUT-66

## Motor

| Pieza | Ubicación |
| --- | --- |
| Emisión | `emit_validation_started` + ruta nativa `emit-validation-started-event` en `actions.rs` |
| Disparo | `maybe_emit_validation_started_after_tekton_agent` en `workspace_init.rs`; cableado en `executor.rs` y `residual_runner.rs` tras fase `agent:tekton` con `status: executed` |
| Columna | `COLUMN_IN_VALIDATION` en `item_lifecycle_frontmatter.rs` |
| Anti-retroceso | `state_rank`: `in_validation = 3`, `in_review = 4`, `done = 5` en `tracker_sync_replay.rs` |
| Sello | Rama `Validation_Started` en `tracker_stamp.rs` + suscripción en `event-domain-subscriptions.json` |

## Genoma

- Evento `SddIA/events/domain/validation-started.md`
- Acción `SddIA/actions/emit-validation-started-event.md`
- `tracker.state_map.in_validation` en `.SddIA/project.md`

## Pendiente operativo

- Crear workflow state **In Validation** en equipo Linear AUT (entre In Progress e In Review) si aún no existe en el tablero remoto.
