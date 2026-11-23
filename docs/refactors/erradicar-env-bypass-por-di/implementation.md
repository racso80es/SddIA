---
feature_name: erradicar-env-bypass-por-di
created: "2026-11-23"
process: refactorization
branch_name: refactor/erradicar-env-bypass-por-di
persist_ref: docs/refactors/erradicar-env-bypass-por-di
tracker_ref: AUT-116
---
# Implementación — erradicar-env-bypass-por-di (`AUT-116`)

## 1. Inventario clasificado de `SDDIA_*` (bóveda vs bypass)

- **Bóveda legítima (conservada):** `TELEGRAM_BOT_TOKEN`, `TELEGRAM_ALLOWED_CHAT_ID`, `LINEAR_API_TOKEN`, `SDDIA_GEMINI_MODEL`, `IOTA_WALLET_SECRET`.
- **Flags de bypass inventariados en `SddIA/**/*.rs` (línea base: 329 ocurrencias, 34 `ParallelLabGuard`):**
  - Migrados a DI en esta entrega (0 `SDDIA_LAB_SKIP_*` restantes): `notify_humanized_pr_merged.rs` (`SDDIA_LAB_SKIP_OUTBOUND_TELEGRAM` eliminado), `route_domain_core.rs` (eliminadas mutaciones globales `SDDIA_LAB_SKIP_ACCEPT_PR_HANDOFF` y `SDDIA_LAB_SKIP_OUTBOUND_TELEGRAM` + `ParallelLabGuard` en `aut114_pull_request_merged_from_store_invokes_notify_humanized`).
  - Pendientes de migración progresiva (controlados en lista decreciente `LEGACY_SKIP_PENDING_DI` dentro de `gate_no_env_bypass.rs`): `workspace_init.rs` (32), `phase_capsules.rs` (20), `task_queue_manager.rs` (10), `pull_request_review.rs` (6), `thermodynamic.rs` (4), `executor.rs` (4), `delivery_close.rs` (4), `runtime_recycle.rs` (2), y 16 módulos con 1 referencia cada uno.

## 2. Cambios aplicados

- `SddIA/engine/execute-process/src/engine/notify_humanized_pr_merged.rs`: añadido trait `HumanizedNotifyPort`, `ProdHumanizedNotifyPort` y `run_from_event_with_port`; eliminado `truthy_env("SDDIA_LAB_SKIP_OUTBOUND_TELEGRAM")`.
- `SddIA/engine/execute-process/src/engine/route_domain_core.rs`: eliminadas mutaciones globales `SDDIA_LAB_SKIP_ACCEPT_PR_HANDOFF` y migrado `aut114_pull_request_merged_from_store_invokes_notify_humanized` a `MockNotifyPort` sin `ParallelLabGuard`.
- `SddIA/tools/sddia-qa/src/gate_no_env_bypass.rs` + `main.rs`: nuevo gate `gate-no-env-bypass` con tests unitarios.
