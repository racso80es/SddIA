---
feature_name: erradicar-env-bypass-por-di
created: "2026-11-23"
process: refactorization
branch_name: refactor/erradicar-env-bypass-por-di
persist_ref: docs/refactors/erradicar-env-bypass-por-di
tracker_ref: AUT-116
---
# Plan — erradicar-env-bypass-por-di (`AUT-116`)

1. Inventariar todas las lecturas `SDDIA_LAB_*` y `ParallelLabGuard` bajo `SddIA/`.
2. Introducir `HumanizedNotifyPort` en `notify_humanized_pr_merged.rs`, retirar `SDDIA_LAB_SKIP_OUTBOUND_TELEGRAM` y las mutaciones `SDDIA_LAB_SKIP_ACCEPT_PR_HANDOFF` en `route_domain_core.rs`.
3. Migrar el test `aut114_pull_request_merged_from_store_invokes_notify_humanized` a `MockNotifyPort` sin `ParallelLabGuard`.
4. Añadir `gate-no-env-bypass` en `sddia-qa` con tests unitarios y lista decreciente `LEGACY_SKIP_PENDING_DI`.
5. Registrar en `SddIA/evolution/` y cerrar ciclo `delivery-close-cycle` → `accept-pr`.
