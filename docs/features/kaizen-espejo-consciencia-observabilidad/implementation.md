---
feature_name: kaizen-espejo-consciencia-observabilidad
created: "2026-08-29"
process: feature
uuid: 97d96117-49cf-4db7-b860-acd65bee216a
execution_id: "a15ad28b-27a3-491c-902e-f78c100ffd43"
items:
  - path: SddIA/ecosystem-health/
    action: add
    note: Crate sddia-ecosystem-health — compile_map_snapshot + fuse_ecosystem_health (SSOT merge)
  - path: SddIA/core/cumulo.paths.json
    action: modify
    note: observability.map_snapshot + ecosystem_health; version 1.8.0
  - path: SddIA/core/event-domain-subscriptions.json
    action: modify
    note: Domain_Entity_Created/Updated/Deleted → compile-ecosystem-map-snapshot
  - path: SddIA/process/compile-ecosystem-map-snapshot.md
    action: add
    note: entity-manager uuid e7f09165-c445-49ae-965d-41abb4738679
  - path: SddIA/process/query-ecosystem-health.md
    action: add
    note: entity-manager uuid 2b337302-e794-46b8-ad4e-f65bafd21c94
  - path: SddIA/engine/execute-process/src/engine/handlers/ecosystem_health.rs
    action: add
    note: Handlers nativos + dispatch mod.rs
  - path: SddIA/interfaces/kalma2-bridge/src/main.rs
    action: modify
    note: GET /api/system-health
  - path: interfaces/kalma2/index.html
    action: modify
    note: Panel Espejo de consciencia
  - path: interfaces/kalma2/app.js
    action: modify
    note: loadSystemHealth fetch pull
  - path: interfaces/kalma2/style.css
    action: modify
    note: health-* color tokens
  - path: SddIA/evolution/97d96117-49cf-4db7-b860-acd65bee216a.md
    action: add
---

# Implementación — kaizen-espejo-consciencia-observabilidad

## T0 — Topología

- `cumulo.paths.json` v1.8.0: claves `observability.map_snapshot` y `observability.ecosystem_health`.
- Evolution `97d96117-49cf-4db7-b860-acd65bee216a`.

## T1 — Merge Core

- Crate `SddIA/ecosystem-health` (`sddia-ecosystem-health`): parseo índices, compilación mapa, fusión L8, tests OBS-CA2/CA3/CA7.

## T2 — Procesos y EDA

- `entity-manager` → `compile-ecosystem-map-snapshot` (e7f09165…) y `query-ecosystem-health` (2b337302…).
- Handlers nativos en `execute-process`; sin referencias en `daemon_heartbeat.rs` ni `radamanto_batch_core.rs` (OBS-CA6).

## T3 — Bridge

- `GET /api/system-health` consume `fuse_ecosystem_health(compile_map: false)`.
- Test `telemetry_routes_exist_in_dispatch` ampliado con ruta system-health.

## T4 — WUI

- Sección `Espejo de consciencia` tras Pulso cognitivo; matriz por `color`/`reason`.

## Pendiente (Argos / cierre)

- `validacion.md` APTO + PBI → `docs/todos/done/`.
- Smoke E2E lab con `event-watcher` tras eventos domain.
- Rebuild release en Paciente 0 (`CARGO_TARGET_DIR` explícito).
