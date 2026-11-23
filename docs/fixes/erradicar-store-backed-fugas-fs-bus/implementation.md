---




feature_name: erradicar-store-backed-fugas-fs-bus
created: "2026-11-23"
process: bug-fix
branch_name: fix/erradicar-store-backed-fugas-fs-bus
persist_ref: docs/fixes/erradicar-store-backed-fugas-fs-bus
tracker_ref: AUT-114
execution_id: "4b8e4543-d8d2-45aa-97af-b3ee5b35b3b3"
---
# Implementación — erradicar-store-backed-fugas-fs-bus (`AUT-114`)

## 1. Cambios en código productivo y tests

1. **`SddIA/engine/execute-process/src/engine/route_domain_core.rs`**:
   - Reemplazada `load_route_ecst_from_logical_path` (que comprobaba `if event_path.is_file()`) por `load_route_ecst_by_id(repo, event_id)` invocando `eda_port::load_dispatched(repo, clean_id, Path::new(""))`.
   - Eliminado el parámetro `event_path: Option<&Path>`, la variable `store_backed`, el prefijo `store_backed &&` en el despacho cognitivo y la guarda `if store_backed { ... "store-backed: acción nativa requiere ECST en bus FS" ... }` en `dispatch_subscriber`.
   - Eliminado `pending_path: &Path` en `handle_subscriber` (pasando `None` a `promote_witness`) y eliminada la construcción de la ruta sintética `PathBuf::from(format!("{}.json", uri.event_id))` y ramas muertas `from_store` / `ensure_processing_header` en `route_domain_event` y `route_domain_batch`.
   - Añadido oráculo `tier_dispatch::aut114_pull_request_merged_from_store_invokes_notify_humanized`.
2. **`SddIA/engine/execute-process/src/engine/route_fractal_core.rs`**:
   - Actualizadas las llamadas a `dispatch_subscriber` para eliminar el quinto argumento `Some(&event_path)`.
3. **`SddIA/engine/execute-process/src/engine/ecst_input_loader.rs`**:
   - Eliminada la construcción de `PathBuf::from(format!("{}.json", uri.event_id))` / `format!("{event_id}.json")` en las rutas `EventUri` y `event_id` de `load_from_event_uri` y `load_ecst_from_inputs`, pasando `Path::new("")` a `load_from_store`.
4. **`SddIA/tools/sddia-qa/src/gate_event_manager_port.rs`**:
   - Añadidas las reglas `SUBSTRATE_LEAK_FORBIDDEN` (`"store_backed"` en todo código productivo fuera de allowlist) y `EVENT_PATH_IS_FILE_FORBIDDEN` (`"event_path.is_file()"`, `"pending_path.is_file()"` en `event_substrate_scope`).
   - Añadido test `aut114_rejects_store_backed_and_event_path_is_file_in_prod`.

## 2. Auditoría transversal de fugas de sustrato (`SddIA/{engine,daemons,tools,interfaces}`)

Barrido ejecutado con `git grep -n "store_backed\|from_store\|is_file()\|pending_path" -- SddIA/engine/ SddIA/daemons/ SddIA/tools/ SddIA/interfaces/`:
- **Fugas detectadas y eliminadas:** `route_domain_core.rs` (`store_backed`, `from_store`, `event_path.is_file()`, rutas sintéticas `<uuid>.json`) y `ecst_input_loader.rs` (rutas sintéticas `<uuid>.json` para `EventUri`/`event_id`).
- **Usos legítimos fuera de alcance de bus Redb (no infieren `FS` vs `Redb`):** comprobaciones de ficheros locales de configuración/artefactos (`.env`, `cumulo.paths.json`, `Cargo.toml`, `validacion.md`, `spec.md`, binarios de cápsulas, `.eml` en `email-watcher`, y helpers de bus fractal/telemetría por fichero en `fractal_bus.rs`, `route_fractal_core.rs`, `radamanto_batch_core.rs`).
