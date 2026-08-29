---
feature_name: route-domain-event-fracture-b3a715381787
created: "2026-08-29"
updated: "2026-08-29T18:12:00Z"
process: bug-fix
branch_name: fix/route-domain-event-fracture-b3a715381787
persist_ref: docs/fixes/route-domain-event-fracture-b3a715381787
pbi_ref: docs/todos/done/[FIX] route-domain-event — fractura sistémica (b3a715381787).md
document_id: PBI-FIX-FRACTURE-b3a715381787
uuid: b3a71538-1787-4000-8000-000000000001
incident_ref: "System_Fracture_Detected — b3a715381787"
global: APTO
pbi_archived: true
branch: fix/route-domain-event-fracture-b3a715381787
approval_status: aprobado
verdict: aprobado
resolution: DONE_CODIGO_TAXONOMIA
checks:
  CA1_PUBLISH_ERROR_PREFIX: APTO
  CA2_FRICTION_PUBLISH_ERROR: APTO
  CA3_UNREACHABLE_UNCHANGED: APTO
  CA4_FRACTURE_UNIT_TESTS: APTO
  CA5_INSTANCE_500_ROOT: DIFERIDO
  CA5_PUBLISH_E2E_GREEN: DIFERIDO
  CA6_REANCHOR_QUEUE: APTO
  CASCADE_SPEC: APTO
  CASCADE_PLAN: APTO
  CASCADE_IMPLEMENTATION: APTO
  CASCADE_EXECUTION: APTO
  CASCADE_VALIDACION: APTO
  SEAL_ANCHOR_PUBLISHER: APTO
  DELIVERY_CLOSE_CYCLE: PENDIENTE
git_changes:
  - SddIA/tools/iota-immutable-publisher/src/main.rs
  - SddIA/tools/iota-immutable-publisher.md
  - SddIA/engine/execute-process/src/engine/route_domain_core.rs
  - docs/fixes/route-domain-event-fracture-b3a715381787/
  - docs/todos/done/[FIX] route-domain-event — fractura sistémica (b3a715381787).md
---

# Validación — fractura `b3a715381787` (Argos)

## Veredicto

**APTO** — entrega de código y taxonomía DLT en rama `fix/route-domain-event-fracture-b3a715381787` (commits `36e318a`, `e5b445a`). Alcance cerrado: Kaizen observabilidad (`F-DLT-PUBLISH-ERROR`, prefijos `iota-relay-publish-error`). Publish E2E en verde y causa física exacta del HTTP 500 **diferidos** (relay inactivo en sesión de validación; bóveda presente).

## Checks

| Check | Estado | Evidencia |
|-------|--------|-----------|
| `CA1_PUBLISH_ERROR_PREFIX` | APTO | `publish_via_relay` distingue `Error::Status` |
| `CA2_FRICTION_PUBLISH_ERROR` | APTO | `classify_batch_anchor_friction` + test `emit_dlt_batch_fracture_publish_error_friction` |
| `CA3_UNREACHABLE_UNCHANGED` | APTO | test `emit_dlt_batch_fracture_writes_pending` |
| `CA4_FRACTURE_UNIT_TESTS` | APTO | `cargo test -p iota-immutable-publisher` (4/4) |
| `CA5_INSTANCE_500_ROOT` | DIFERIDO | bóveda OK; log hijo Node no disponible (relay off) |
| `CA5_PUBLISH_E2E_GREEN` | DIFERIDO | `GET /health` sin respuesta en `:8787` |
| `CA6_REANCHOR_QUEUE` | APTO | cola vacía |
| `SEAL_ANCHOR_PUBLISHER` | APTO | `entity-manager` seal-anchor release |
| `DELIVERY_CLOSE_CYCLE` | PENDIENTE | barrera `simulated` — PBI `c51acf014c0f` |

## Deuda operativa (fuera de este PR)

Reproducir publish con `iota-publish-relay` activo para confirmar candidato 2–3 del PBI y cerrar `CA5_*` en operación.
