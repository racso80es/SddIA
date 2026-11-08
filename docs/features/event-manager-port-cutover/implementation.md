---
feature_name: event-manager-port-cutover
created: "2026-11-08"
process: refactorization
phase: ejecucion-parcial
pbi_ref: MAN-35
document_id: PBI-176f9f2f
status: in-progress
---
# Implementation — MAN-35

## Hecho en rama `refactor/event-manager-port-cutover`

| Fase | Estado | Notas |
|------|--------|-------|
| T0 | hecho | `norma-redb-exclusiva.md` ids MAN-35 |
| T1 | parcial | Base MAN-33 importada: contrato **1.1.0** + ops consulta; falta bump **1.2.0** y ops admin/DLT (`entity-manager`) |
| T2 | parcial | `eda_port` + `sddia-event-manager-host` desde MAN-33 |
| T3 | hecho (núcleo) | `ecst_input_loader` verify inline; `route_domain_core` carga estricta vía puerto; sin lectura FS en route/batch/reanchor load |
| T4–T8 | pendiente | Ver `plan.md` |

## Próximo

1. T1: `forge` contrato 1.2 vía procesos oficiales + adapters.
2. T4–T7: fractal, productores, legacy FS.
3. T8: suite + `validacion.md`.
