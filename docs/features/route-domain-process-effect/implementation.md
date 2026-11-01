---
feature_name: route-domain-process-effect
created: "2026-11-01"
process: feature
branch_name: Fix_Events_Multiple
persist_ref: docs/features/route-domain-process-effect
tracker_ref: AUT-61
historia_ref: AUT-20
---

# Implementación — AUT-61

## Alcance

Matriz `route_domain_process_effect_catalog_success` y endurecimiento de cuatro tests existentes en `SddIA/engine/execute-process/tests/eda_subscriber_ecst_redb.rs`.

## Entorno temporal

- Scratch bajo `{checkout}/.tmp/sddia-ecst-redb-*` (evita `/tmp` lleno).
- Symlinks: `SddIA/target`, `events`, `process`, codex `process/`, esqueleto DI (taxonomy, contracts, `git-manager`, `linear-tracker-adapter`).
- Copias: `.SddIA/project.md`, `projects/core.md`, `active-domain-profile.json`, core JSON.
- Git lab: `main` + remoto bare `.lab-origin.git`, rama `feat/lab`, docs PPR con `implementation.md`.
- Seeds: PBI TQM, suite `smoke` vacía, payloads ECST alineados a handlers.

## Lab env (router)

`SDDIA_EVENT_STORE=redb`, `SDDIA_LAB_ROUTE_SYNC=1`, `SDDIA_LAB_MOCK_OUTBOUND=1`, `SDDIA_CLI_FOREGROUND=1`, `SDDIA_LAB_ALLOW_DIRTY=1`, `SDDIA_EXECUTE_PROCESS_BIN`, `SDDIA_TQM_STOP_AFTER=design` en catalog/kalma2. `ModeGuard` restaura `SDDIA_AGENT_RUNTIME_COMMAND` del host.

## Verificación

Desde `SddIA/`:

`cargo test -p execute-process --test eda_subscriber_ecst_redb -- --test-threads=1`
