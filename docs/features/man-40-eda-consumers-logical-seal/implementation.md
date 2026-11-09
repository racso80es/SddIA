---
feature_name: man-40-eda-consumers-logical-seal
created: "2026-11-09"
updated: "2026-11-09"
process: feature
phase: implementacion
branch_name: feat/man-40-eda-consumers-logical-seal
persist_ref: docs/features/man-40-eda-consumers-logical-seal
pbi_ref: MAN-40
tracker_ref: MAN-40
document_id: PBI-1b004e99
execution_id: "d5f40be9-1866-4813-a5b2-28a26faed56c"
agent: tekton
---
# Implementación — man-40-eda-consumers-logical-seal

## T0 — Sello lógico

- `github_bridge::materialize_to_bus`: `publish_ecst` → token `.events/domain/{event_id}.json`; sin `resolved_redb_path`.

## T1 — Topología / route

- `write_processing_witness`: token lógico bajo `processing_subscribers`.
- `route_domain_core`: cabecera `from_store` = `.events/pending/{uuid}.json`; `is_runtime_db_event_path` solo vía host.
- `terminal_witness_exists`: sin fallback FS en build productivo (`#[cfg(test)]` conserva witnesses disco).

## T2 — Modo / auditoría

- `event-bus-audit`: escaneo vía puerto; anomalías sin path Redb.
- `kalma2-bridge/eda_productive`, `repo-retention-purger`: sin `productive_store_present` como gate de modo.

## T3 — event-watcher

- `SDDIA_LAB_WATCH_FRACTAL` default `0` (claim fractal FS desactivado por defecto).
- Telemetría claim: etiqueta `port` en métricas (no `redb`).

## T5 — Gate

- `gate_event_manager_port`: veto `resolved_redb_path` / `runtime_db_path` / `DEFAULT_REDB_REL` fuera de host + allowlists (MAN-40).

## Fuera de alcance en este delta

- Stub `claim_store_fractal` (familias fractal vía puerto = siguiente hito).
- Scripts/installer T4 y lab forge-doc-drift (deuda acotada en spec; sin regresión de sello).
