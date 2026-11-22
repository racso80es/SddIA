---
feature_name: man-51-gate-sddia-daemon-runtime-residuos-events
created: "2026-11-22"
updated: "2026-11-22"
process: feature
purpose: Plan ejecutable — cierre punto ciego gates y residuos .events (MAN-51)
branch_name: feat/man-51-gate-sddia-daemon-runtime-residuos-events
persist_ref: docs/features/man-51-gate-sddia-daemon-runtime-residuos-events
pbi_ref: MAN-51
parent_ref: MAN-43
document_id: PBI-1377d0ec
agent: dedalo
phase: diseno-blueprint
version_plan: "1.0.0"
status: blueprint-ready
audit_ref: docs/audits/informe_de_diagnostico_desacople_eventos_sddia.md
norm_ref: features-documentation-pattern@1.3.1
execution_id: "e8cd02b1-e76c-4a3c-a44c-1cef29bc4704"
phases:
  - id: Ola 0
    name: Precondición y Oráculo en Rojo
  - id: Ola 1
    name: Gates QA (AC-1)
  - id: Ola 2
    name: Motor y Legacy (AC-2, AC-3)
  - id: Ola 3
    name: Contrato e Higiene Documental (AC-4)
  - id: Ola 4
    name: Verificación Completa y Paridad Host
---
# Plan de ejecución — man-51-gate-sddia-daemon-runtime-residuos-events

## Ola 0 — Precondición y Oráculo en Rojo
1. Verificar entorno hermético (`git status` en rama feature, raíz limpia sin `.events/`, `orchestration/`, `telemetry/`).
2. Capturar en rojo los oráculos de `AC-1`..`AC-4` (ampliación de `BODY_FORBIDDEN` sobre `events-contract.md` actual y retirada de `VIRTUAL_PATH_ALLOW` antes del fix).

## Ola 1 — Gates QA (`AC-1`)
1. Añadir descubrimiento dinámico de miembros de `SddIA/Cargo.toml` y patrones `concat!` en `gate_event_manager_port.rs` y `gate_eda_unit_no_real_backend.rs`, con tests unitarios.

## Ola 2 — Motor y Legacy (`AC-2`, `AC-3`)
1. Eliminar creación de `.events` en `instance_creator.rs:603-606`.
2. Sustituir detección `.events` por `SddIA/core/cumulo.paths.json` + `find_repo_root()` en `fractal_bus.rs:153-162` y retirar ambos de `VIRTUAL_PATH_ALLOW`.
3. Limpiar `transit-event-payload/src/main.rs` y `hook_common.sh`; purgar `SddIA/target/release/event-sweeper`.

## Ola 3 — Contrato e Higiene Documental (`AC-4`)
1. Actualizar `SddIA/events/events-contract.md` (§§ 3–4) al flujo exclusivo `EventManagerPort` v2 + `EventUri`.
2. Ampliar `BODY_FORBIDDEN` en `eda_event_token_hygiene.rs` y verificar verde.

## Ola 4 — Verificación Completa y Paridad Host
1. Ejecutar `cargo test --workspace`, `cargo clippy --workspace -- -D warnings`, `sddia-qa` gates y rebuild `--release`.
2. Documentar `implementation.md` y `execution.md` (`status: executed`).
