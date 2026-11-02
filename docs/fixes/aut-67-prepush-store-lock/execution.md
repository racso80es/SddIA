---
feature_name: aut-67-prepush-store-lock
process: bug-fix
phase: execution
tracker_ref: AUT-67
uuid: e46d8e0c-b9fa-4d18-a035-99e9cf66ac07
persist_ref: docs/fixes/aut-67-prepush-store-lock
branch_name: Fix_Events_Multiple_02
---

# Ejecución — aut-67-prepush-store-lock

## Comandos

| Comando | Resultado esperado |
|---------|-------------------|
| `cargo test -p event-watcher store_claim_deferred_while_routes_in_flight` | PASS |
| `bash SddIA/scripts/qa/test-aut-67-prepush-store-lock.sh` | `OK test-aut-67-prepush-store-lock` |

## Notas

- Push real con `event-watcher` activo: verificar AC1 en rama `Fix_Events_Multiple_02` tras refrescar atestación QA si aplica.
- Cierre Linear AUT-67 → Done: operador post-merge (`done_gate: linear`).
