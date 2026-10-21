---
document_id: PBI-EDA-REDB-07-CUTOVER
uuid: "8e1b6d40-3a95-4c72-b8f1-9d2a5c7e0b64"
title: "[ARQUITECTURA] EDA Redb 07 — Cutover a redb y baja de event-sweeper"
format: markdown
version: "1.0.0"
created: "2026-10-21"
status: done
priority: alta
type: arquitectura
process: feature
dispatch: false
historia_ref: "docs/todos/pending/HU-SDDIA-EDA-REDB-001: Evolución del Bus de Eventos a Redb, Jurisdicción Cognitiva y Estandarización del Marco SddIA.md"
historia_document_id: HU-SDDIA-EDA-REDB-001
historia_uuid: "7c4e9a12-8b3d-4f61-a9e0-2d5c6b8f1a47"
suggested_branch: feat/eda-redb-07-cutover
persist_ref_suggested: docs/features/eda-redb-07-cutover
blocked_by:
  - PBI-EDA-REDB-03-WATCHER-VITALITY
  - PBI-EDA-REDB-04-READERS
  - PBI-EDA-REDB-05-DLT-FRONTIER
  - PBI-EDA-REDB-06-SEMANTIC-LAYER
unblocks: []
laudos:
  - L-REDB-02
  - L-REDB-06
related:
  - SddIA/core/cumulo.paths.json
  - SddIA/core/eda-coverage.json
  - SddIA/daemons/event-sweeper.md
  - SddIA/daemons/index.md
  - SddIA/engine/execute-process/src/engine/handlers/system_vitality.rs
  - start-sddia.sh
  - .gitignore
  - README.md
acceptance:
  - AC-12
---

# Cutover a `redb` y baja de `event-sweeper`

Historia madre: §3.4, §6 (cierre de rutas), AC-12, L-REDB-02, L-REDB-06. Último PBI. Exige 03, 04, 05 y 06 mergeados (02 y 01 van dentro de esa cadena).

## 1. Entrega

1. Default de `SDDIA_EVENT_STORE` pasa a `redb`. `filesystem` y `dual` siguen existiendo como escape explícito, no como omisión de la variable.
2. `SddIA/core/cumulo.paths.json`: `event_bus`, `eda_bus` y `eda_fractal` apuntan a `.SddIA/runtime/events.redb` y a los nombres de tabla (`events`, `subscriber_delivery`, `dead_letter_index`, `progress_traces`, `dlt_audit_seals`). Las claves `subscriptions`, `telemetry_subscriptions`, `orchestration_subscriptions` y `domain_subscriptions` conservan su JSON actual. No se tocan `eda_instance.proofs` ni `daemons_instance.state`.
3. `.gitignore`: ignorar `.SddIA/runtime/*.redb` y el lock de `redb`. La regla `/.events/` se mantiene para residuos locales; el runtime ya no la necesita como cola.
4. Baja de `event-sweeper` vía `entity-manager` (ficha `event-sweeper.md`, UUID `3eafa012-2b71-47e5-b47e-467b59a3fd52`; `SddIA/daemons/index.md`; `SddIA/core/eda-coverage.json`). Retirar el crate `SddIA/daemons/event-sweeper/` del workspace solo después de esa baja. Launchers `SddIA/daemons/event-sweeper.sh`, `SddIA/scripts/daemons/event-sweeper.sh` y `.bat` salen en el mismo PR.
5. `start-sddia.sh`: quitar `event-sweeper` de `REQUIRED_DAEMONS`, del `--seal-capsules` (`names`) y de la unidad `sddia-event-sweeper@.service`.
6. `probe_bus_topology` en `system_vitality.rs`: con store `redb` comprueba que `events.redb` abre y que los JSON de suscripciones existen. Deja de exigir directorios hoja de `eda_fractal`.
7. `README.md`: el diagrama de centinelas ya no lista `event-sweeper`. Coherente con el PBI 06, que lo conservaba a propósito.

No hay migración de ficheros históricos. La cola viva se drena en `dual` antes de cambiar el default; un `.events/pending/*.json` residual con el default `redb` se registra como fractura, no se reinyecta en silencio.

## 2. Fuera de alcance

- Borrar `.SddIA/proofs` o heartbeats.
- Volver a suscribir IOTA en los 16 eventos del PBI 05.
- Sustituir LanceDB.

## 3. Criterios

| ID | Criterio |
| :--- | :--- |
| AC-12 | `.gitignore` ignora `.SddIA/runtime/*.redb`. `event-sweeper` no está en `REQUIRED_DAEMONS` ni en `SddIA/daemons/index.md`. `probe_bus_topology` en verde sin carpetas `.events/{telemetry,orchestration,domain,progress}`. |
| CA-09 | Arranque lab: watcher drena un evento insertado solo en `redb`. Cero proceso `event-sweeper` en el árbol de procesos. |
| CA-10 | Variable explícita `SDDIA_EVENT_STORE=filesystem` sigue levantando el camino de ficheros (escape, no default). |
