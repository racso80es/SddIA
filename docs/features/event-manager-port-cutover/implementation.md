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
| T1 | hecho (tax/bind) | Core **1.2.0** + defaults; Redb DLT/list_deliveries; schema library; `capability-taxonomy` **1.0.11** (`eda:event-manager` catalog **1.2.0**); bindings **1.9.0** sin fallback FS |
| T2 | hecho | Host `resolve_*` + bootstrap CLI/`run_process`; `event-watcher`/productores vía host; tests DI 1.2 |
| T3 | hecho (núcleo) | `ecst_input_loader` verify inline; `route_domain_core` carga estricta vía puerto; sin lectura FS en route/batch/reanchor load |
| T4 | hecho | DLT/verify + suite 1.2 |
| T5 | hecho | kalma2-bridge + lancedb → host |
| T6 | hecho | event-bus-audit + repo-retention-purger → puerto |
| T7 | hecho (núcleo) | `eda_port` productivo solo puerto; legacy en `eda_port_legacy` + `#[cfg(test)]`; testigos por `list_deliveries` |
| T8 | parcial | `execution.md` + `validacion.md`; CI pendiente |

## Próximo

1. PR + gate CI workspace.
2. Linear MAN-35 → Done tras merge.
