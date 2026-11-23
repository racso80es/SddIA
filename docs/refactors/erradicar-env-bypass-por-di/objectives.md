---
feature_name: erradicar-env-bypass-por-di
created: "2026-11-23"
process: refactorization
branch_name: refactor/erradicar-env-bypass-por-di
persist_ref: docs/refactors/erradicar-env-bypass-por-di
tracker_ref: AUT-116
---
# Objetivos y Criterios de Aceptación — erradicar-env-bypass-por-di (`AUT-116`)

- [x] **AC-1 (Inventario):** Catálogo clasificado (bóveda `.env` legítima vs flags de bypass `SDDIA_LAB_*`) documentado en `implementation.md`.
- [x] **AC-2 (DI):** Frontera de notificación post-merge (`HumanizedNotifyPort` + `ProdHumanizedNotifyPort` + `run_from_event_with_port`) y despacho de suscriptores (`SubscriberDispatchPort`) operando por puertos inyectados sin mutar `env` global.
- [x] **AC-3 (Erradicación):** Retirado `SDDIA_LAB_SKIP_OUTBOUND_TELEGRAM` de `notify_humanized_pr_merged.rs` y `std::env::set_var("SDDIA_LAB_SKIP_ACCEPT_PR_HANDOFF", "0")` de `route_domain_core.rs`, eliminando el `ParallelLabGuard` de `aut114_pull_request_merged_from_store_invokes_notify_humanized`.
- [x] **AC-4 (Gate estático):** Nuevo gate `sddia-qa gate-no-env-bypass` activo (`ok: true`, `violation_count: 0`) con lista decreciente `LEGACY_SKIP_PENDING_DI` para los módulos pendientes de migración progresiva.
- [x] **AC-5 (Hermeticidad):** `cargo check --workspace` (`-Dwarnings`) y tests unitarios en verde.
