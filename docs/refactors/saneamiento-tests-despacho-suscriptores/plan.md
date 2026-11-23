---
feature_name: saneamiento-tests-despacho-suscriptores
created: "2026-11-23"
process: refactorization
branch_name: refactor/saneamiento-tests-despacho-suscriptores
persist_ref: docs/refactors/saneamiento-tests-despacho-suscriptores
tracker_ref: AUT-115
---
# Plan — saneamiento-tests-despacho-suscriptores (`AUT-115`)

1. Introducir `SubscriberDispatchPort` y `dispatch_subscriber_with_ports` en `route_domain_core.rs`.
2. Reemplazar `store_backed_native_capsule_does_not_use_cognitive_runtime` por `native_action_subscriber_executes_without_cognitive_runtime` con `SpyDispatchPort`.
3. Sanear `tier_none_does_not_spawn_and_cognitive_receives_ecst_and_workspace` y `aut114_pull_request_merged_from_store_invokes_notify_humanized` y auditar `tests/`.
4. Verificar `cargo test`, `RUSTFLAGS="-Dwarnings" cargo check --workspace` y `sddia-qa gate-event-manager-port`, cerrar PR y mergear en `main`.
