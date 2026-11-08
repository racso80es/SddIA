---
feature_name: event-manager-port-cutover
created: "2026-11-08"
process: refactorization
pbi_ref: MAN-35
document_id: PBI-176f9f2f
branch: refactor/event-manager-port-cutover
global: APTO-PARCIAL
pbi_archived: false
---
# Validación — event-manager-port-cutover (MAN-35)

## Alcance verificado

- Composition root `sddia-event-manager-host` + `eda_port` en motor.
- Contrato **1.2.0** (core + Redb + mock + suite).
- Carga estricta `route_domain_core` / `ecst_input_loader`.
- DLT `seal_dlt` / `get_dlt_seal` vía puerto.
- Productores **kalma2-bridge**, **lancedb_thought_repo** publican/consultan puerto.
- **event-bus-audit** / **repo-retention-purger** sin `EventStore::open` productivo.
- Taxonomía **1.0.11** + bindings **1.9.0** (`eda.event_manager` **1.2.0**, sin rol `fallback` FS).

## Pendiente antes de merge

- `cargo test --workspace` verde en CI.
- Tests integración `execute-process`: barrido `delivery_close` (54/55) y `route_domain_core` (57/57 salvo flaky paralelo) — fracturas/lab FS, witnesses, `publish_ecst` pending; pendiente CA12 evolution e2e.

## Criterio global

`APTO-PARCIAL` — listo para PR y CI; cierre Linear MAN-35 tras merge + gate verde.
