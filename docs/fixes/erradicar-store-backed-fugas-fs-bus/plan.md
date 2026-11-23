---




feature_name: erradicar-store-backed-fugas-fs-bus
created: "2026-11-23"
process: bug-fix
branch_name: fix/erradicar-store-backed-fugas-fs-bus
persist_ref: docs/fixes/erradicar-store-backed-fugas-fs-bus
tracker_ref: AUT-114
execution_id: "f464f55f-a2d5-40d6-b63b-e57f510f9709"
---
# Plan de Implementación — erradicar-store-backed-fugas-fs-bus (`AUT-114`)

1. **Ola 0 (Oráculo en rojo):** Añadir `aut114_pull_request_merged_from_store_invokes_notify_humanized` en `route_domain_core.rs` y verificar fallo con `"store-backed: acción nativa requiere ECST en bus FS"`.
2. **Erradicación en `route_domain_core.rs` y loaders:** Eliminar `store_backed`, rutas sintéticas `<uuid>.json`, `!path.is_file()` y `from_store` en `route_domain_core.rs`, `route_fractal_core.rs` y `ecst_input_loader.rs`.
3. **Gate estático `sddia-qa`:** Ampliar `gate_event_manager_port.rs` con `SUBSTRATE_LEAK_FORBIDDEN` (`store_backed`) y `EVENT_PATH_IS_FILE_FORBIDDEN` (`event_path.is_file()`, `pending_path.is_file()`).
4. **Verificación y cierre:** `cargo check --workspace` (`-Dwarnings`), `cargo test`, `sddia-qa` gates, `delivery-close-cycle`, `accept-pr` y reciclado `--release` en host.
