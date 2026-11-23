---
feature_name: erradicar-store-backed-fugas-fs-bus
created: "2026-11-23"
process: bug-fix
branch_name: fix/erradicar-store-backed-fugas-fs-bus
persist_ref: docs/fixes/erradicar-store-backed-fugas-fs-bus
tracker_ref: AUT-114
---
# Registro de Ejecución — erradicar-store-backed-fugas-fs-bus (`AUT-114`)

## Baseline hermético Ola 0
- Rama origen: `main` (`git status --porcelain` vacío).
- Raíz libre de artefactos sueltos (`.events`, `orchestration`, `telemetry`, `_lab` ausentes).
- Variables `SDDIA_*` en entorno de shell: limpias (sin `SDDIA_EVENT_STORE` ni `SDDIA_EXECUTE_PROCESS_BIN`).

## Oráculo Ola 0 (rojo previo al fix)
- Comando: `cd SddIA && cargo test -p execute-process --lib aut114_pull_request_merged_from_store_invokes_notify_humanized -- --nocapture`
- Resultado observado: **FAILED** (`left: "failed", right: "success"`, `err=Some("store-backed: acción nativa requiere ECST en bus FS") code=1`).

## Ejecución de tareas
- **T1:** Eliminación de `store_backed`, ruta sintética `<uuid>.json`, `from_store` y `event_path` en `route_domain_core.rs`, `route_fractal_core.rs` y `ecst_input_loader.rs`.
- **T2:** Ampliación de `gate_event_manager_port.rs` con `SUBSTRATE_LEAK_FORBIDDEN` y `EVENT_PATH_IS_FILE_FORBIDDEN` + test unitario.
- **T3:** Verificación de `cargo check --workspace` (`RUSTFLAGS="-Dwarnings"`), `cargo test --workspace` y gates `sddia-qa`.
