---




feature_name: erradicar-store-backed-fugas-fs-bus
created: "2026-11-23"
process: bug-fix
base: main
scope: Erradicar ramificación store_backed y fugas de sustrato FS vs Redb en el runtime productivo
branch_name: fix/erradicar-store-backed-fugas-fs-bus
persist_ref: docs/fixes/erradicar-store-backed-fugas-fs-bus
tracker_ref: AUT-114
execution_id: "4b8e4543-d8d2-45aa-97af-b3ee5b35b3b3"
---
# Especificación Técnica — erradicar-store-backed-fugas-fs-bus (`AUT-114`)

## 1. Diagnóstico y Causa Raíz

En `SddIA/engine/execute-process/src/engine/route_domain_core.rs`:
- `route_domain_event` construía una ruta lógica sintética `<event_id>.json` (`PathBuf::from(format!("{}.json", uri.event_id))`) con `from_store = true` tras cargar el ECST desde `EventManagerPort`.
- `handle_subscriber` pasaba `Some(pending_path)` a `dispatch_subscriber`.
- `dispatch_subscriber` evaluaba `let store_backed = event_path.is_some_and(|path| !path.is_file());` y abortaba `notify-humanized-pr-merged` con `"store-backed: acción nativa requiere ECST en bus FS"`, enviando todos los `PullRequest_Merged` a `dead-letter` sin notificar por Telegram.

## 2. Criterios de Aceptación y Oráculo Mecánico (Ola 0)

| ID | Criterio | Oráculo mecánico ejecutable (Ola 0) |
| --- | --- | --- |
| AC-1 | Oráculo Ola 0 (`PullRequest_Merged` desde `EventManagerPort` → `notify-humanized-pr-merged` en `success`) | `cargo test -p execute-process --lib aut114_pull_request_merged_from_store_invokes_notify_humanized` (en rojo antes del fix con `store-backed: acción nativa requiere ECST en bus FS`; en verde tras el fix) |
| AC-2 | Erradicación de `store_backed`, ruta sintética `<uuid>.json` y `!path.is_file()` en `route_domain_core.rs` | `dispatch_subscriber`, `handle_subscriber` y `route_domain_event` operan exclusivamente sobre `EventUri` (`event_id`) y `event: &Value` |
| AC-3 | Test anti-runtime-cognitivo sigue garantizando `!marker.exists()` sin depender de `store_backed` | `cargo test -p execute-process --lib store_backed_native_capsule_does_not_use_cognitive_runtime` |
| AC-4 | Auditoría transversal en `engine/`, `daemons/`, `tools/`, `interfaces/` sin fugas `Redb` vs `FS` | Barrido documentado en `implementation.md`; limpieza en `route_domain_core.rs`, `route_fractal_core.rs` y `ecst_input_loader.rs` |
| AC-5 | Gate estático anti-regresión en `sddia-qa` | `sddia-qa gate-event-manager-port` (`ok: true`, `violation_count: 0`) + test unitario `aut114_rejects_store_backed_and_event_path_is_file_in_prod` |
| AC-6 | Workspace en verde sin warnings (`-Dwarnings`) | `cargo check --workspace` (`RUSTFLAGS="-Dwarnings"`), `cargo test --workspace`, `gate-eda-unit-no-real-backend`, `audit-eda-coverage --scan` |
