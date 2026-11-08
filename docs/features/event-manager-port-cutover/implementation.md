---
feature_name: event-manager-port-cutover
created: "2026-11-08"
process: refactorization
phase: cierre
pbi_ref: MAN-35
document_id: PBI-176f9f2f
status: ready-for-pr
---
# Implementation — MAN-35

## Hecho en rama `refactor/event-manager-port-cutover`

| Fase | Estado | Notas |
|------|--------|-------|
| T0 | hecho | `norma-redb-exclusiva.md` ids MAN-35 |
| T1 | hecho (tax/bind) | Core **1.2.0** + defaults; Redb DLT/list_deliveries; schema library; `capability-taxonomy` **1.0.11**; bindings **1.9.0** sin fallback FS |
| T2 | hecho | Host `resolve_*` + bootstrap CLI/`run_process`; tests DI 1.2 |
| T3 | hecho | Loader `verify_inline_ecst`; rechazo JSON físico; `route_domain`/`eda_bus` productivo solo puerto |
| T4 | hecho | `fractal_bus`: ack/DLQ/purge vía `eda_port`; mutación FS fractal solo `#[cfg(test)]` |
| T5 | hecho | kalma2-bridge + lancedb → host |
| T6 | hecho | `event-bus-audit`: tipos locales + solo puerto sin redb; sin scan FS en release |
| T7 | hecho (núcleo) | `eda_port` productivo solo puerto; legacy en `eda_port_legacy` |
| T8 | hecho | `gate-event-manager-port` en `sddia-qa`; CI `sddia-index-qa`; README + Constitución; `validacion.md` **APTO** |

## Cierre

1. PR único → merge `main` + checks verdes.
2. Linear MAN-35 → `state_map.done` vía delivery-close-cycle.
