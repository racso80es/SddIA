---
feature_name: saneamiento-tests-despacho-suscriptores
created: "2026-11-23"
process: refactorization
branch_name: refactor/saneamiento-tests-despacho-suscriptores
persist_ref: docs/refactors/saneamiento-tests-despacho-suscriptores
tracker_ref: AUT-115
---
# Implementación — saneamiento-tests-despacho-suscriptores (`AUT-115`)

- `SddIA/engine/execute-process/src/engine/route_domain_core.rs`:
  - Añadido trait `SubscriberDispatchPort`, `ProdSubscriberDispatchPort` y `dispatch_subscriber_with_ports`.
  - Reemplazado `store_backed_native_capsule_does_not_use_cognitive_runtime` por `native_action_subscriber_executes_without_cognitive_runtime` usando `SpyDispatchPort` (sin `std::env::set_var`, sin `spawn.sh`, sin `lab-bus/pending`, aserción positiva `status == "success"` y `cognitive_calls == 0`).
  - Saneados `tier_none_does_not_spawn_and_cognitive_receives_ecst_and_workspace` y `aut114_pull_request_merged_from_store_invokes_notify_humanized` eliminando rutas `lab-bus/pending/<uuid>.json`.
- Barrido de `SddIA/engine/execute-process/tests/` (`eda_store_io_matrix_*.rs`, `event_store_writers.rs`, `dlt_telemetry.rs`, `memory_evolution_ingest.rs`): `git grep -n "store_backed\|pending/"` devuelve 0 coincidencias.
