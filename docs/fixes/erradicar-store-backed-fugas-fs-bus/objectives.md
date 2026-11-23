---




feature_name: erradicar-store-backed-fugas-fs-bus
created: "2026-11-23"
process: bug-fix
branch_name: fix/erradicar-store-backed-fugas-fs-bus
persist_ref: docs/fixes/erradicar-store-backed-fugas-fs-bus
tracker_ref: AUT-114
execution_id: "f464f55f-a2d5-40d6-b63b-e57f510f9709"
---
# Objetivos y Criterios de Aceptación — erradicar-store-backed-fugas-fs-bus (`AUT-114`)

- [x] **AC-1 (Oráculo Ola 0):** Test `aut114_pull_request_merged_from_store_invokes_notify_humanized` que ejercita `dispatch_subscriber` para `PullRequest_Merged` / `argos.notify-humanized-pr-merged` solo desde `EventManagerPort` (sin fichero en disco), en rojo antes del fix y en verde tras el fix.
- [x] **AC-2 (Sin `store_backed` productivo):** Eliminados de `route_domain_core.rs` la ruta sintética `<uuid>.json`, `!path.is_file()` y la guarda `if store_backed` que abortaba nativas; el despacho usa `EventUri` (`event_id`) y el valor en memoria.
- [x] **AC-3 (Test anti-cognitivo):** `store_backed_native_capsule_does_not_use_cognitive_runtime` sigue en verde verificando `!marker.exists()` sin depender del rechazo `store_backed`.
- [x] **AC-4 (Auditoría transversal):** Barrido documentado en `implementation.md` sobre `engine/`, `daemons/`, `tools/` e `interfaces/` sin fugas de inferencia de sustrato fuera de la allowlist de infraestructura.
- [x] **AC-5 (Gate QA):** `sddia-qa gate-event-manager-port` ampliado para vetar `store_backed` y `.is_file()` sobre rutas/ids de evento (`ok: true`, `violation_count: 0`).
- [x] **AC-6 (Host):** Tests y gates en verde sin warnings (`RUSTFLAGS="-Dwarnings"`), binarios `--release` y servicios `systemd --user` sincronizados tras el merge en `main`.
