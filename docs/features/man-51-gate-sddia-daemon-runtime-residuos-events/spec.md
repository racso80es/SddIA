---
feature_name: man-51-gate-sddia-daemon-runtime-residuos-events
created: "2026-11-22"
updated: "2026-11-22"
process: feature
base: main
scope: "Cobertura de crates en gates sddia-qa, erradicación de .events en motor/legacy y convergencia de events-contract.md"
branch_name: feat/man-51-gate-sddia-daemon-runtime-residuos-events
persist_ref: docs/features/man-51-gate-sddia-daemon-runtime-residuos-events
pbi_ref: MAN-51
parent_ref: MAN-43
historia_ref: MAN-43
document_id: PBI-1377d0ec
document_uuid: "c3aff1f0-495b-40ab-a042-4420dd241400"
capability_id: "eda:event-manager"
contract: "eda.event_manager"
agent: dedalo
phase: diseno-blueprint
version_spec: "1.0.0"
status: blueprint-ready
audit_ref: docs/audits/informe_de_diagnostico_desacople_eventos_sddia.md
norm_ref: features-documentation-pattern@1.3.1
tracker_channel: ".SddIA/state/tracker-outbound/MAN-51.jsonl"
execution_id: "e8cd02b1-e76c-4a3c-a44c-1cef29bc4704"
---
# Especificación técnica — man-51-gate-sddia-daemon-runtime-residuos-events

## 1. Gates QA (`AC-1`)
- **`SddIA/tools/sddia-qa/src/gate_event_manager_port.rs`:**
  - Implementar `workspace_member_roots(repo: &Path) -> Vec<PathBuf>` leyendo `SddIA/Cargo.toml` (`members = [...]`, resolviendo entradas directas y globs `dir/*`).
  - Añadir a `FORBIDDEN` patrones anti-evasión: `concat!('.', "events"`, `concat!(".events"`.
  - Mantener `fs_ops_scope` activo sobre `SddIA/daemons/event-watcher/` y `SddIA/sddia-daemon-runtime/src/eda_*.rs`.
- **`SddIA/tools/sddia-qa/src/gate_eda_unit_no_real_backend.rs`:**
  - Reutilizar el descubrimiento dinámico de miembros de `SddIA/Cargo.toml` en `run()`.
  - Añadir a `FORBIDDEN` los patrones `concat!('.', "events"`, `concat!(".events"`.

## 2. Motor (`AC-2`)
- **`SddIA/engine/execute-process/src/engine/handlers/instance_creator.rs`:**
  - Eliminar la creación física de `instance_root.join(".events")` y sus 5 subdirectorios en `Topologia` (L603–606).
- **`SddIA/engine/execute-process/src/engine/fractal_bus.rs`:**
  - Reemplazar la búsqueda de `Some(".events")` en `repo_root_from_event` por búsqueda ascendente del marcador `SddIA/core/cumulo.paths.json` con fallback a `crate::core::repo::find_repo_root().ok()`.
- **`SddIA/engine/execute-process/src/engine/eda_event_token_hygiene.rs`:**
  - Retirar `fractal_bus.rs` e `instance_creator.rs` de `VIRTUAL_PATH_ALLOW`.

## 3. Legacy y fósiles (`AC-3`)
- **`SddIA/tools/transit-event-payload/src/main.rs`:**
  - Eliminar los defaults `.events/{pending,processing,processed,dead_letter}` y la creación automática `fs::create_dir_all` en `ensure_event_bus_topology` / `main`. Si `cumulo.paths.json` o `env` no declaran rutas FS explícitas de laboratorio, devolver error claro indicando migración a `EventManagerPort`. Retirar de `VIRTUAL_PATH_ALLOW`.
- **`SddIA/scripts/qa/git-hooks/hook_common.sh`:**
  - Eliminar `eda_bus_dirs()` y la lectura de `.events/*` en `scan_presented_for_branch()`.
- **Binario fósil:**
  - Eliminar `SddIA/target/release/event-sweeper` del host.

## 4. Contrato e higiene documental (`AC-4`)
- **`SddIA/events/events-contract.md`:**
  - Actualizar § 3 (`delivery_state`) y § 4 (diagrama Mermaid + pasos 1–8) para reflejar exclusivamente el flujo `EventManagerPort` v2 + `EventUri` sin referencias a `filesystem/dual`, `pending/` JSON, `processing/subscribers/`, `EventStore::replay` ni `event-sweeper`.
- **`SddIA/engine/execute-process/src/engine/eda_event_token_hygiene.rs`:**
  - Ampliar `BODY_FORBIDDEN` en `scan_genome_and_readme_body` con `"EventStore::"`, `"StoreMode::"`, `"filesystem/dual"` y `"processing/subscribers"`.
