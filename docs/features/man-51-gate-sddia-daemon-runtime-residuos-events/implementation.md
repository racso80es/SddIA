---


feature_name: man-51-gate-sddia-daemon-runtime-residuos-events
created: "2026-11-22"
updated: "2026-11-22"
process: feature
branch_name: feat/man-51-gate-sddia-daemon-runtime-residuos-events
persist_ref: docs/features/man-51-gate-sddia-daemon-runtime-residuos-events
pbi_ref: MAN-51
parent_ref: MAN-43
document_id: PBI-1377d0ec
phase: ejecucion
status: implemented
norm_ref: features-documentation-pattern@1.3.1
agent: tekton
execution_id: "d577eabc-9947-4781-b087-7d0475963e29"
---
# Implementación — man-51-gate-sddia-daemon-runtime-residuos-events

## 1. Gates QA (`AC-1`)
- `SddIA/tools/sddia-qa/src/gate_event_manager_port.rs`: añadido `workspace_member_roots(repo)` leyendo `[workspace].members` desde `SddIA/Cargo.toml` (cubre `sddia-daemon-runtime`, `sddia-core`, `sddia-runtime-parity`, `ecosystem-health` y el resto de miembros), activando `fs_ops_scope` sobre `sddia-daemon-runtime`. Añadidos patrones anti-evasión `concat!` en `FORBIDDEN` y tests unitarios (`workspace_member_roots_includes_daemon_runtime_and_core`, `detects_concat_evasion_in_prod`).
- `SddIA/tools/sddia-qa/src/gate_eda_unit_no_real_backend.rs`: reutilización de `workspace_member_roots`, patrones `concat!` en `FORBIDDEN`, test `detects_concat_evasion_in_unit_test`.
- Limpieza de 10 violaciones descubiertas en `#[cfg(test)]` al ampliar el alcance del gate a `sddia-daemon-runtime` (`eda_sweep.rs`, `github_bridge.rs`, `lib.rs`, `linear_bridge.rs`, `linear_bridge_webhook.rs`) y `user-preference-core/src/lib.rs`.

## 2. Motor sin `.events` (`AC-2`)
- `SddIA/engine/execute-process/src/engine/handlers/instance_creator.rs`: eliminada la creación de `instance_root.join(".events")` y sus 5 subdirectorios en `Topologia`.
- `SddIA/engine/execute-process/tests/instance_creator.rs`: actualizadas las aserciones de `creates_topology_and_skips_smoke` y `smoke_native_without_skip` para exigir ausencia de `.events`.
- `SddIA/engine/execute-process/src/engine/fractal_bus.rs`: `repo_root_from_event` busca el marcador `SddIA/core/cumulo.paths.json` hacia arriba con fallback a `find_repo_root().ok()`.
- `SddIA/engine/execute-process/src/engine/eda_event_token_hygiene.rs`: retirados `fractal_bus.rs`, `instance_creator.rs` y `transit-event-payload/src/main.rs` de `VIRTUAL_PATH_ALLOW`; añadido `gate_event_manager_port.rs` a `SCAN_EXCLUDE`.

## 3. Legacy y fósiles (`AC-3`)
- `SddIA/tools/transit-event-payload/src/main.rs`: retirados los defaults `.events/*` y `fs::create_dir_all` en `ensure_event_bus_topology`.
- `SddIA/scripts/qa/git-hooks/hook_common.sh`: eliminada `eda_bus_dirs()` y la lectura física de `.events/*` en `scan_presented_for_branch()`.
- Purgado el binario fósil `SddIA/target/release/event-sweeper`.

## 4. Contrato e higiene documental (`AC-4`)
- `SddIA/events/events-contract.md`: actualizadas §§ 3–4 al flujo exclusivo `EventManagerPort` v2 + `EventUri` (sin diagrama ni reglas `filesystem/dual`, `pending/` JSON, `processing/subscribers/`, `EventStore::replay` ni `event-sweeper`).
- `README.md`: sustituida la cita literal `EventStore::open` en § Bus ECST.
- `SddIA/engine/execute-process/src/engine/eda_event_token_hygiene.rs`: ampliado `BODY_FORBIDDEN` con `EventStore::`, `StoreMode::`, `filesystem/dual` y `processing/subscribers`.
