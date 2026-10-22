---
feature_name: tracker-lifecycle-contract
created: "2026-10-22"
process: feature
phases:
  - T0-catalogo
  - T1-binding
  - T2-schema
  - T3-sello
execution_id: "3e0a8c0f-1b97-4b9c-a467-b149494b3e28"
document_id: PBI-ARQUITECTURA-TRACKER-LIFECYCLE-01
---

# Plan — tracker-lifecycle-contract

| Fase | Hecho | Verificación |
|------|--------|----------------|
| T0 | Alta `tracker:lifecycle` en el catálogo y fila del índice | id con dos puntos; versión de norma `1.0.9`; hash estable |
| T1 | Una fila de binding | cero duplicados de `capability_id` |
| T2 | Schema `emit_v2` | test `ac_tracker_lifecycle_emit_v2_body` |
| T3 | `evolution-register` + `emit-domain-mutation` update de la norma | `Domain_Entity_Updated`; gate evolution en el cierre |
