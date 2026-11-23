---
feature_name: saneamiento-tests-despacho-suscriptores
created: "2026-11-23"
process: refactorization
branch_name: refactor/saneamiento-tests-despacho-suscriptores
persist_ref: docs/refactors/saneamiento-tests-despacho-suscriptores
tracker_ref: AUT-115
---
# Objetivos y Criterios de Aceptación — saneamiento-tests-despacho-suscriptores (`AUT-115`)

- [x] **CA-1:** 0 referencias a `store_backed` en `SddIA/engine/execute-process/`; `store_backed_native_capsule_does_not_use_cognitive_runtime` reemplazado por `native_action_subscriber_executes_without_cognitive_runtime`.
- [x] **CA-2:** Validación positiva y hermética por DI (`SubscriberDispatchPort` + `SpyDispatchPort`): la acción nativa (`notify-humanized-pr-merged` y `process` nativos) retorna `status == "success"`, `code == 0` y `cognitive_calls == 0` sin rutas `lab-bus/pending/*.json` ni dependencia de binarios en `target/`.
- [x] **CA-3:** Barrido de tests hermanos en `route_domain_core.rs` (`tier_dispatch`) y `SddIA/engine/execute-process/tests/` (`eda_store_io_matrix_*.rs`, `event_store_writers.rs`, `dlt_telemetry.rs`, `memory_evolution_ingest.rs`) documentado en `implementation.md`.
- [x] **CA-4:** `cargo test -p execute-process` en verde sin warnings (`RUSTFLAGS="-Dwarnings"`) y `sddia-qa gate-event-manager-port` con `violation_count: 0`.
