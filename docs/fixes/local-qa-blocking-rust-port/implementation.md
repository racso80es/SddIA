---
feature_name: local-qa-blocking-rust-port
created: "2026-07-11"
process: bug-fix
items:
  - id: blocking-core
    artifact: SddIA/engine/execute-process/src/engine/route_domain_core.rs
    change: SyncRouteGuard, resolve_route_event_path, validate_blocking_subscribers, is_local_qa_event
  - id: blocking-handler
    artifact: SddIA/engine/execute-process/src/engine/handlers/route_domain.rs
    change: blocking/sync/event_type/target inputs
  - id: pre-push-gate
    artifact: SddIA/scripts/qa/git-hooks/pre_push_gate.py
    change: Local_QA_Requested blocking antes delivery-close-cycle
  - id: python-parity
    artifact: SddIA/scripts/qa/route_domain_event_core.py
    change: bypass precheck pull-request-review para Local QA
---

# Implementación — blocking sync route-domain-event

## Núcleo Rust

- `resolve_route_event_path`: acepta `event_file_path` **o** `event_type` (+ payload opcional).
- Modo `blocking=true`: valida suscriptores SSOT; rechaza agente destino inexistente (`target`).
- `SyncRouteGuard`: inyecta `SDDIA_LAB_ROUTE_SYNC=1` solo durante la invocación.
- `is_local_qa_event`: omite precheck ciclo de vida PR remoto en aduana local pre-push.

## Hook pre-push

Secuencia por rama: `route-domain-event` (blocking, `Local_QA_Requested`) → `delivery-close-cycle`.

## Invocación orquestador

```json
{
  "event_type": "Local_QA_Requested",
  "blocking": true,
  "emitter_agent": "git-hook-pre-push",
  "payload": {"branch": "fix/example"}
}
```
