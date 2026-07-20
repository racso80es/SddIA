---
feature_name: kalma2-process-dispatch
created: "2026-07-20"
process: feature
items:
  - handlers/task_queue_manager.rs
  - invoke_orchestrator.rs
  - handlers/kalma2.rs extract_pbi_ref
  - engine/mod.rs registro TQM
---

# Implementation — kalma2-process-dispatch

## Touchpoints

| # | Path | Cambio |
|---|------|--------|
| 1 | `SddIA/engine/execute-process/src/engine/handlers/task_queue_manager.rs` | Handler nativo B′: paquete Kalma2 → despacho hijo; legado `tasks_path` |
| 2 | `SddIA/engine/execute-process/src/engine/handlers/mod.rs` | Export módulo |
| 3 | `SddIA/engine/execute-process/src/engine/mod.rs` | Early-return `task-queue-manager` |
| 4 | `SddIA/engine/execute-process/src/engine/invoke_orchestrator.rs` | `invoke_process_full_with_env` (L2 skips hijo) |
| 5 | `SddIA/engine/execute-process/src/engine/handlers/kalma2.rs` | A′: `extract_pbi_ref` vía anclas `docs/todos/…/*.md` |

## Decisiones de forja

- No mutación de subscriptions ni ECST del evento.
- No forja genoma `task-queue-manager.md` en este PR: el handler early-return evita `INPUT_VALIDATION` sin cambiar `declared_inputs` (T3 diferido).
- L2: con `correlation_id` y sin `SDDIA_TQM_FULL_CYCLE`, el hijo recibe `SDDIA_LAB_SKIP_PBI_ARCHIVE` + `SDDIA_LAB_SKIP_DELIVERY_CLOSE` si no estaban definidos.
