---
document_id: b2e4c891-3f7a-4d2e-9c1b-8a5f6e0d2c47
title: Kalma2 — ola mejora post-auditoría EV-AUD-005 (K1–K5)
type: bug-fix
status: done
priority: high
created: "2026-08-13"
suggested_branch: fix/kalma2-post-ev-aud-005-ola
source_audit: docs/fixes/execute-process-phase-failure-propagation/clarify.md
parent_pbi: docs/todos/done/[FIX] execute-process — fallo de fase debe fallar ejecución global (EV-AUD-005).md
parent_correlation_id: dcb9efed-2268-4298-8108-7a55cf4db323
persist_ref_suggested: docs/fixes/kalma2-post-ev-aud-005-ola
findings:
  - KALMA2-AUD-EV005-001
related_wip:
  - interfaces/kalma2/app.js
  - SddIA/engine/execute-process/src/engine/thermodynamic.rs
  - SddIA/daemons/event-watcher/src/main.rs
  - SddIA/engine/execute-process/src/engine/handlers/task_queue_manager.rs
  - SddIA/interfaces/kalma2-bridge/src/main.rs
---

# Kalma2 — ola mejora post-auditoría EV-AUD-005 (K1–K5)

## Problema

Auditoría del ciclo Kalma2 `dcb9efed-…` (forja EV-AUD-005): la UX y la orquestación introdujeron fricción **independiente** del fix de agregación terminal. Mezclar ese WIP con el PR EV-AUD-005 provocó `SCOPE_WIP_CONTAMINATION` en Argos.

## Objetivo

Materializar mejoras Kalma2/TQM/watcher/bridge en **PR separado**, con evidencia de runtime, sin contaminar el cierre del PBI EV-AUD-005.

## Alcance (tareas)

| ID | Tarea | Touchpoint | Estado WIP local |
|----|--------|------------|------------------|
| **K1** | Poll UI: no cortar sondeo en `initialized`/`awaiting_agents`; timeout extendido hasta `completed`/`failed` | `interfaces/kalma2/app.js` | parcial |
| **K2** | Early PEC TQM: `cycle_phase=awaiting_agents` (no `initialized`) | `thermodynamic::emit_initialized_pec` | parcial |
| **K3** | `event-watcher`: rutas async + tope in-flight (evitar inanición domain) | `SddIA/daemons/event-watcher/src/main.rs` | parcial |
| **K4** | TQM: `branch_name`/`fix_name` desde `suggested_branch` del frontmatter PBI | `task_queue_manager.rs` | parcial + test |
| **K5** | Single-flight por `correlation_id` (evitar doble `bug-fix` concurrente) | `task_queue_manager.rs` + `route_domain_core` | pendiente |
| **K6** | Bridge: PEC correlacionado = evento **más reciente** por `timestamp` | `kalma2-bridge/src/main.rs` | parcial |

## Fuera de alcance

- Lógica `phase_terminal` / EV-AUD-005 (PR hermano).
- Mutación genoma vía IDE; forja vía `bug-fix`/`feature` + `entity-manager` si aplica.

## Criterios de aceptación

- Forja Kalma2 de un PBI con `suggested_branch` usa esa rama (no slug truncado).
- UI muestra `awaiting_agents` y **sigue** sondeando hasta terminal.
- Evento `Kalma2_Process_Requested` en domain se enruta sin quedar huérfano >120s con pending saturado (smoke documentado).
- Re-forja mismo `correlation_id` no lanza segundo hijo en paralelo (K5).
- PR aislado; sin diffs EV-AUD-005 en el mismo merge.
- `validacion.md` APTO en persist propio o feature doc.

## Referencias

- Auditoría: `docs/fixes/execute-process-phase-failure-propagation/clarify.md`
- Spec pendientes P6: `docs/fixes/execute-process-phase-failure-propagation/spec.md`
- Procedimiento retoma: `docs/fixes/execute-process-phase-failure-propagation/procedimiento-retoma.md`
