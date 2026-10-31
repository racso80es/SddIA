---
feature_name: event-bus-audit-store-signals
created: "2026-10-31"
process: bug-fix
branch_name: Fix_Events_Multiple
persist_ref: docs/fixes/event-bus-audit-store-signals
tracker_ref: AUT-27
execution_id: "6cd0a99f-1d58-43ee-8e30-31e9e13a090b"
items_applied:
  - T1
  - T2
  - T3
  - T4
---

# Implementación — AUT-27

## T1

`audit_state_from_store` registra `store_unavailable` cuando `EventStore::open` falla. `build_report_md` añade la sección «Store no disponible» antes de los conteos.

## T2

`audit_domain_delivery_signals` cruza cada evento de dominio del store con `plan_route` / `subscriber_id` del registro de dominio (y códice si aplica). Emite `delivery_fuera_de_catalogo` y `suscriptor_sin_fila`.

## T3

Misma función: `success_sin_efecto` para entregas `Success` con agente cognitivo y workspace de despacho vacío; `tracker-stamp` exige jsonl en `tracker-outbound`. `audit_residual_pending_files` inventaría `*.json` en pending y enlaza fractura `F-EDA-RESIDUAL-FILESYSTEM` si la traza nombra el fichero.

## T4

Cuatro tests en `SddIA/tools/event-bus-audit/src/main.rs`. `cargo test -p event-bus-audit` → 13 passed.
