---

feature_name: pr-merged-witness-store
created: "2026-10-24"
process: bug-fix
phases:
  - id: T1
    title: Alta con fila en el store
  - id: T2
    title: El fallback no pisa la delegación
  - id: T3
    title: tracker_ref en el merge de ciclo
execution_id: "f0d1f9df-997e-4739-b698-2771dc57867a"
---
# Plan

| Id | Tarea | Hecho |
|----|--------|--------|
| T1 | `write_pending_event` de las acciones inserta `record_ecst` según `SDDIA_EVENT_STORE`. | AC-1, AC-2 |
| T2 | `handle_subscriber` conserva el estado de la delegación y escribe `result_status` en el dead-letter. | AC-3 |
| T3 | `accept-pr`, la presentación y el cierre de entrega propagan `tracker_ref`. | AC-4 |
