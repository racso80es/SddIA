---

feature_name: pr-merged-witness-store
created: "2026-10-24"
process: bug-fix
items:
  - SddIA/engine/execute-process/src/engine/actions.rs
  - SddIA/engine/execute-process/src/engine/route_domain_core.rs
  - SddIA/engine/execute-process/src/engine/accept_pr.rs
  - SddIA/engine/execute-process/src/engine/phase_capsules.rs
  - SddIA/engine/execute-process/src/engine/tracker_pbi_meta.rs
execution_id: "f0d1f9df-997e-4739-b698-2771dc57867a"
---
# Implementación

La alta de acción usa la misma compuerta que la mutación de dominio: store si `writes_store`, JSON si `writes_filesystem`.

La promoción fallida deja el `delivery_status` de la delegación. El fallback añade `result_status` junto a `error_trace`.

`tracker_ref` entra por el input, por el presentado (pending, processed o fila de store) o por `objectives.md`. No sustituye un valor ya puesto.
