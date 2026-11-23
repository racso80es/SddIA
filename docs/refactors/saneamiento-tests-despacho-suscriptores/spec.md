---
feature_name: saneamiento-tests-despacho-suscriptores
created: "2026-11-23"
process: refactorization
branch_name: refactor/saneamiento-tests-despacho-suscriptores
persist_ref: docs/refactors/saneamiento-tests-despacho-suscriptores
tracker_ref: AUT-115
---
# Especificación Técnica — saneamiento-tests-despacho-suscriptores (`AUT-115`)

| ID | Criterio | Oráculo mecánico ejecutable (Ola 0) |
| --- | --- | --- |
| CA-1 | Cero `store_backed` en `execute-process` | `git grep -n "store_backed" -- SddIA/engine/execute-process/` (0 resultados) |
| CA-2 | Validación positiva hermética por DI | `cargo test -p execute-process --lib native_action_subscriber_executes_without_cognitive_runtime` |
| CA-3 | Barrido tests hermanos | Registro en `implementation.md` + `tier_none_does_not_spawn_and_cognitive_receives_ecst_and_workspace` sin `_missing` |
| CA-4 | Puertas en verde | `cargo test -p execute-process --lib route_domain_core` + `sddia-qa gate-event-manager-port` |
