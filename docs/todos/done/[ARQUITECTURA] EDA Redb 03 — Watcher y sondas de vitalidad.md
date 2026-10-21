---
document_id: PBI-EDA-REDB-03-WATCHER-VITALITY
uuid: "9b4e7c25-8a1d-4e63-b5f0-2d8c6a1e4f97"
title: "[ARQUITECTURA] EDA Redb 03 — Watcher y sondas de vitalidad"
format: markdown
version: "1.0.0"
created: "2026-10-21"
status: done
priority: alta
type: arquitectura
process: feature
dispatch: false
historia_ref: "docs/todos/done/historias/HU-SDDIA-EDA-REDB-001: Evolución del Bus de Eventos a Redb, Jurisdicción Cognitiva y Estandarización del Marco SddIA.md"
historia_document_id: HU-SDDIA-EDA-REDB-001
historia_uuid: "7c4e9a12-8b3d-4f61-a9e0-2d5c6b8f1a47"
suggested_branch: feat/eda-redb-03-watcher-vitality
persist_ref_suggested: docs/features/eda-redb-03-watcher-vitality
blocked_by:
  - PBI-EDA-REDB-01-STORE
  - PBI-EDA-REDB-02-WRITERS
unblocks:
  - PBI-EDA-REDB-07-CUTOVER
laudos:
  - L-REDB-06
  - L-REDB-08
related:
  - SddIA/daemons/event-watcher/
  - SddIA/daemons/event-watcher.md
  - SddIA/daemons/event-sweeper/src/main.rs
  - SddIA/engine/execute-process/src/engine/handlers/system_vitality.rs
  - SddIA/engine/execute-process/src/engine/cerbero_di_rbac.rs
acceptance:
  - AC-4
  - AC-7
---

# Watcher sobre `redb` y mudanza de sondas

Historia madre: §3.4, §2, L-REDB-06, L-REDB-08. `event-sweeper` sigue instalado. Este PBI no lo borra.

## 1. Entrega

`event-watcher` (`SddIA/daemons/event-watcher/`, ficha v1.1.0, UUID `f995cc89-22a7-488d-9b25-ddb1e5e3a4a4`):

- `SDDIA_EVENT_STORE=filesystem`: comportamiento actual (scan de `.events/pending/` y delegación en `route-domain-event`).
- `dual` o `redb`: reclama filas `subscriber_delivery = pending` y delega el mismo despacho. No abre el ECST para decidir negocio.

El despacho distingue el tier del suscriptor (`agents-contract` v1.1.0, leído del genoma, no inventado):

- `none` (`cerbero`, `cumulo`, `radamanto`): handler determinista. Cero spawn de `SDDIA_AGENT_RUNTIME_COMMAND`. Cerbero sigue en `cerbero_di_rbac.rs`.
- `high` / `medium` / `low`: `AGENT_PHASE` con el ECST leído del store y `workspace_path` acotado a lo que Cerbero autorizó.

Sondas que hoy están en `SddIA/daemons/event-sweeper/src/main.rs` y pasan al tick de `event-watcher`:

1. `execute-process --process daemon-heartbeat-audit --inputs '{"sweep":true}'` (periodo del heartbeat del watcher, 30 s).
2. `execute-process --process system-vitality-probe --inputs '{}'` cada `SDDIA_VITALITY_PROBE_SECONDS` (default 300).

El sweeper puede seguir invocándolas en este PBI. La prueba de AC-7 arranca el watcher con el proceso del sweeper ausente y exige que ambas sondas se hayan ejecutado. La baja del binario, de `REQUIRED_DAEMONS` y de la unidad systemd es el PBI 07.

`probe_bus_topology` no se reescribe aquí: sigue exigiendo directorios `eda_fractal` mientras el default sea `filesystem`.

Bump de `event-watcher.md` (jurisdicción y versión) vía `entity-manager`. Prohibido editar la ficha a mano.

## 2. Fuera de alcance

- Quitar `event-sweeper` de `start-sddia.sh`, del sello de cápsulas o de `eda-coverage.json`.
- CLI `query-events` (PBI 04).
- Cambiar `llm_profile` de ningún agente.

## 3. Criterios

| ID | Criterio |
| :--- | :--- |
| AC-4 | Suscriptor `tier: none` no spawnea runtime LLM. Suscriptor cognitivo recibe el ECST desde `redb` y un `workspace_path` ya autorizado. |
| AC-7 | Con el proceso `event-sweeper` sin ejecutar, un tick del watcher dispara `daemon-heartbeat-audit` sweep y `system-vitality-probe`. |
| CA-06 | `filesystem`: el watcher sigue drenando `.events/pending/` como antes del PR. |
