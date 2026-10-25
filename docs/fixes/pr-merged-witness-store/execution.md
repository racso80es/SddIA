---

feature_name: pr-merged-witness-store
created: "2026-10-24"
process: bug-fix
items_applied:
  - T1
  - T2
  - T3
tracker_ref: MAN-23
execution_id: "f0d1f9df-997e-4739-b698-2771dc57867a"
---
# Ejecución

| Tarea | Resultado |
|-------|-----------|
| T1 | Alta de acción inserta la fila con `record_ecst` si `writes_store`. En `filesystem` solo queda el JSON. |
| T2 | Fallo de `promote_witness`: `delivery_status` conserva la delegación. El dead-letter lleva `result_status` y `error_trace`. |
| T3 | `tracker_ref` entra por input, por el presentado (pending, processed o store) o por `objectives.md`. No pisa un valor ya puesto. |

Tests: 8 passed en `execute-process --lib` (alta redb, alta filesystem, promoción, objectives, presented procesado, y las tres regresiones de emisión).
