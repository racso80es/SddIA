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
status: executed
verdict: executed
norm_ref: features-documentation-pattern@1.3.1
agent: tekton
execution_id: "e8cd02b1-e76c-4a3c-a44c-1cef29bc4704"
items_applied:
  - Ola 0
  - Ola 1
  - Ola 2
  - Ola 3
  - Ola 4
phases:
  Ola 0:
    status: closed
  Ola 1:
    status: closed
  Ola 2:
    status: closed
  Ola 3:
    status: closed
  Ola 4:
    status: closed
---
# Registro de Ejecución — man-51-gate-sddia-daemon-runtime-residuos-events

## Baseline (Ola 0) y Oráculo en Rojo

- **Rama:** `feat/man-51-gate-sddia-daemon-runtime-residuos-events`
- **Raíz del repositorio:** limpia (sin `.events/`, `orchestration/`, `telemetry/`, `pending/`, `domain/`, `dead-letter/`, `progress/`, `_lab/`).
- **Oráculo en rojo (`task-200`):** tras retirar `fractal_bus.rs`, `instance_creator.rs` y `transit-event-payload` de `VIRTUAL_PATH_ALLOW` y ampliar `BODY_FORBIDDEN`, `eda_event_token_hygiene` falló en `eda_event_token_hygiene_no_virtual_event_paths_workspace` (6 hits en los 3 ficheros) y `eda_event_token_hygiene_genome_and_readme_body` (`EventStore::` en `SddIA/events/events-contract.md`).
- **Ampliación de gate (`task-210`):** al activar `workspace_member_roots` en `gate-eda-unit-no-real-backend`, el escaneo pasó de `203` a `239` ficheros y detectó 10 violaciones ocultas en `#[cfg(test)]` de `sddia-daemon-runtime` y `user-preference-core`.

## Evidencia de Verificación (Ola 4)

| Check | Resultado |
|-------|-----------|
| `cargo test -p execute-process --lib eda_event_token_hygiene` | **OK** (`9 passed; 0 failed`) |
| `cargo test -p sddia-qa gate_` | **OK** (`13 passed; 0 failed`) |
| `cargo run -p sddia-qa -- gate-event-manager-port --json` | **OK** (`scanned_files: 265`, `violation_count: 0`) |
| `cargo run -p sddia-qa -- gate-eda-unit-no-real-backend --json` | **OK** (`scanned_files: 239`, `violation_count: 0`) |
| `cargo run -p sddia-qa -- verify-process-integrity` | **OK** |
| `cargo run -p sddia-qa -- verify-tools-index` | **OK** |
| `cargo run -p sddia-qa -- audit-eda-coverage --scan --json` | **OK** (`indexed_entities: 147`, `orphan_count: 0`) |
| `RUSTFLAGS="-Dwarnings" cargo check --workspace` | **OK** |
| `cargo test --workspace --no-fail-fast -- --test-threads=1` + re-run `instance_creator` (9/9) + `kalma2`/`refine` (lock release) | **OK** |
| Purga de `SddIA/target/release/event-sweeper` | **OK** (ausente) |

## Matriz de Criterios de Aceptación

| AC | Estado | Evidencia |
|----|--------|-----------|
| **AC-1** | **PASS** | `workspace_member_roots` + patrones `concat!` en ambos gates; `265` / `239` ficheros escaneados con `0` violaciones. |
| **AC-2** | **PASS** | `instance_creator.rs` y `fractal_bus.rs` sin `.events` y fuera de `VIRTUAL_PATH_ALLOW`; `tests/instance_creator.rs` 9/9 OK. |
| **AC-3** | **PASS** | `transit-event-payload` y `hook_common.sh` sin `.events/*`; binario fósil `event-sweeper` eliminado. |
| **AC-4** | **PASS** | `SddIA/events/events-contract.md` (§§ 3–4) migrado a `EventManagerPort` v2 + `EventUri`; `BODY_FORBIDDEN` ampliado y `eda_event_token_hygiene` 9/9 OK. |
