---
feature_name: man-36-event-manager-instance-config
created: "2026-11-09"
process: refactorization
phase: implementacion
branch_name: refactor/man-36-event-manager-instance-config
persist_ref: docs/features/man-36-event-manager-instance-config
pbi_ref: MAN-36
tracker_ref: MAN-36
document_id: PBI-0604fb3c
agent: tekton
execution_id: "4fc6eadd-78b9-4022-8991-ed39d911e9af"
---

# Implementación — man-36-event-manager-instance-config

## T0–T1 — Bóveda y host

- `SddIA/sddia-event-manager-host/src/config.rs`: resolución `SDDIA_EVENT_MANAGER_*`, validación L-PATH-POLICY, precedencia SO > instancia > global.
- `lib.rs`: factory productiva desde config; API `resolved_redb_path`, `productive_store_present`, `is_resolved_redb_path`; retirado `events_redb_path` público.
- `.dev/.env.example`: bloque MAN-36; deprecación `SDDIA_EVENT_STORE` productivo.

## T2 — Consumidores

- `runtime_bridge.rs`: `PublishSeal` con token lógico `.events/{family}/{id}.json`.
- `eda_port.rs`: `runtime_db_path` delega en host.
- `github_bridge`, `event-bus-audit`, `kalma2-bridge`, `linear_bridge`, purger: sin API path cableada legacy.

## T3 — Cúmulo

- `cumulo.paths.json` v1.12.0: eliminados `event_bus`, `eda_bus.store`, `eda_fractal.store`.

## T4 — Documentación

- `README.md`, `CONSTITUTION_CORE.md` §3.1: límite Core/instancia y bóveda.
- Genomas tool/daemon: referencias al puerto (sin path físico en descripciones).

## Tests

- `sddia-event-manager-host`: 15 tests (precedencia, rutas, roles).
- Compilación: `execute-process`, `sddia-daemon-runtime`, `event-bus-audit`, `kalma2-bridge`.
