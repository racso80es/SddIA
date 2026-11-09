---


feature_name: man-36-event-manager-instance-config
created: "2026-11-09"
updated: "2026-11-09"
process: refactorization
phase: implementacion
branch_name: refactor/man-36-event-manager-instance-config
persist_ref: docs/features/man-36-event-manager-instance-config
pbi_ref: MAN-36
tracker_ref: MAN-36
document_id: PBI-0604fb3c
agent: tekton
execution_id: "716aba21-3983-4458-a49c-9315a538dd76"
---
# Implementación — man-36-event-manager-instance-config

## T0–T1 — Bóveda y host (previos; auditados)

- `SddIA/sddia-event-manager-host/src/config.rs`: resolución `SDDIA_EVENT_MANAGER_*`, L-PATH-POLICY, precedencia SO > instancia > global; `DEFAULT_REDB_REL` solo aquí.
- `lib.rs`: factory productiva; `resolved_redb_path` / `productive_store_present` / `is_resolved_redb_path`; sin `events_redb_path` público.
- `.dev/.env.example`: bloque MAN-36; legacy `SDDIA_EVENT_STORE` marcado retirado.

## T2 — Delta CA-4 (esta pasada Tekton)

- `eda_port.rs`: retirado fallback `repo.join(DEFAULT_REDB_REL)`; `runtime_db_path` delega solo a `resolved_redb_path` (pánico determinista si la bóveda es inválida). Helpers `path_is_resolved_redb` / `store_present`.
- `route_domain_core.rs`: detección vía `path_is_resolved_redb`; presencia store vía `store_present`.
- `fractal.rs` / `eda_bus.rs`: `target_path` = token lógico `.events/{family}/{id}.json` (L-LOGICAL-EVENTS).
- `system_vitality.rs`: sonda sin `create_dir_all` de ruta física; solo `probe_productive_handshake`.
- `event-bus-audit`: cero `DEFAULT_REDB_REL`; `resolved_redb_path_for_audit` → `Result`; escaneo vía `productive_store_present` + `with_productive_port`.

## T3 — Cúmulo

- `cumulo.paths.json` v1.12.0: sin `event_bus` / `eda_*.store` físicos (confirmado).

## T4 — Documentación

- Constitución §3.1 + plantilla bóveda ya alineadas; README/gates residuales para Argos si drift.

## Verificación

- T5 (2026-11-09): `cargo test -p sddia-event-manager-host --lib` — 15 passed; `gate-event-manager-port` — `violation_count: 0`; `gate-eda-unit-no-real-backend` — ok; `gate-evolution --range origin/main...HEAD` — ok (`f7687d4a-…`).
- Evidencia determinista: `t5-smoke-evidence.txt` (HEAD + stdout cápsulas).
- Grep allowlist: `DEFAULT_REDB_REL` solo en `sddia-event-manager-host` (+ comentario en `eda_port`).
