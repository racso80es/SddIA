---

feature_name: eda-redb-03-watcher-vitality
created: "2026-10-21"
process: feature
branch_name: feat/eda-redb-03-watcher-vitality
persist_ref: docs/features/eda-redb-03-watcher-vitality
pbi_ref: "docs/todos/pending/[ARQUITECTURA] EDA Redb 03 — Watcher y sondas de vitalidad.md"
document_id: PBI-EDA-REDB-03-WATCHER-VITALITY
pbi_uuid: "9b4e7c25-8a1d-4e63-b5f0-2d8c6a1e4f97"
execution_id: "d707b171-c91f-4933-a68b-0415c19a390d"
---
# Objetivos — eda-redb-03-watcher-vitality

## Misión

El watcher drena `.events/pending/` en `filesystem`. En `dual` o `redb` reclama filas `subscriber_delivery = pending` y delega el mismo despacho sin abrir el ECST. `tier: none` no spawnea `SDDIA_AGENT_RUNTIME_COMMAND`. `high` / `medium` / `low` reciben `AGENT_PHASE` con el ECST y un `workspace_path` que Cerbero autorizó. Un tick del watcher dispara `daemon-heartbeat-audit` y `system-vitality-probe` aunque `event-sweeper` no esté en ejecución.

## Criterios

- AC-4. El suscriptor `none` no arranca el runtime. El cognitivo recibe el ECST leído del store y el workspace acotado.
- AC-7. Con el proceso sweeper ausente, un tick del watcher invoca las dos sondas.
- CA-06. En `filesystem` el pending JSON sigue enrutándose y no se abre `events.redb`.

## Fuera de alcance

Quitar `event-sweeper` de `start-sddia.sh`, del sello de cápsula o de `eda-coverage.json`. CLI `query-events`. Cambiar `llm_profile`. Reescribir `probe_bus_topology`.
