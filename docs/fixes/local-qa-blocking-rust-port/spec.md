---
feature_name: local-qa-blocking-rust-port
created: "2026-07-11"
process: bug-fix
branch_name: fix/local-qa-blocking-rust-port
persist_ref: docs/fixes/local-qa-blocking-rust-port
pbi_ref: docs/todos/pending/Barrera Táctil Local Interceptación QA Síncrona Bloqueante.md
related_feature: docs/features/husky-pre-push-blocking-route
---

# Especificación — modo `--blocking` nativo en `route-domain-event` (Rust)

## Problema

Tras el porte P17 de `route-domain-event` a Rust (`route_domain_core.rs`), el orquestador nativo solo aceptaba `event_file_path` y delegaba en modo async por defecto. El modo síncrono bloqueante (`--blocking` / `SDDIA_LAB_ROUTE_SYNC=1`) existía únicamente en el `__main__` de `route_domain_event_core.py`, quedando fuera del path de producción del binario `execute-process`.

Los hooks Git exigen veredicto síncrono (`exit 0|1`) antes de devolver control a Husky.

## Causa raíz

Brecha de paridad post-porte Rust: handler `handlers/route_domain.rs` no materializaba eventos desde `event_type` ni activaba dispatch sync bajo demanda.

## Cambio

| Artefacto | Modificación |
|-----------|--------------|
| `engine/execute-process/.../route_domain_core.rs` | `SyncRouteGuard`, `resolve_route_event_path`, validación suscriptores blocking, bypass precheck `Local_QA_Requested` |
| `engine/execute-process/.../handlers/route_domain.rs` | Inputs `blocking`/`sync`/`event_type`/`target`; propagación exit code |
| `SddIA/scripts/qa/git-hooks/pre_push_gate.py` | Invocación `route-domain-event` blocking antes de `delivery-close-cycle` |
| `SddIA/scripts/qa/route_domain_event_core.py` | Paridad bypass precheck Local QA |

## Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| CA1 | Bloqueo efectivo: índice roto → push aborta | `pre_push_gate` + `Local_QA_Requested` sync |
| CA2 | Liberación sólida: QA OK → exit 0 | dispatch sync Argos → pull-request-review |
| CA3 | Rechazo si evento/agente inválido en blocking | `validate_blocking_subscribers` + tests |
| CA4 | Paridad Python precheck Local QA | `route_domain_event_core.py` |
| CA5 | Tests Rust verdes | `cargo test -p execute-process --lib` |

## No objetivos

- Reescribir `cli.js` legacy (`.SddIA/core/cli.js` no existe en repo).
- Sustituir `delivery-close-cycle` en pre-push; Local QA es aduana previa.
