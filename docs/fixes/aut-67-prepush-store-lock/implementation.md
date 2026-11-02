---
feature_name: aut-67-prepush-store-lock
process: bug-fix
phase: implementation
tracker_ref: AUT-67
uuid: e46d8e0c-b9fa-4d18-a035-99e9cf66ac07
persist_ref: docs/fixes/aut-67-prepush-store-lock
branch_name: Fix_Events_Multiple_02
---

# Implementación — aut-67-prepush-store-lock

## Touchpoints

| Área | Cambio |
|------|--------|
| `SddIA/daemons/event-watcher/src/main.rs` | `routes_block_store_claim`: difiere `claim_store_pending` y `claim_store_fractal` si `in_flight > 0`. |
| `SddIA/scripts/qa/git-hooks/hook_common.sh` | `hook_stderr_is_store_lock_contentions`, `invoke_local_qa_route_domain_event` (presupuesto `SDDIA_PREPUSH_STORE_LOCK_RETRY_MS`, default 15000). |
| `SddIA/scripts/qa/git-hooks/pre_push_gate.sh` | `Local_QA_Requested` usa reintento; exit 2 → mensaje `STORE_LOCK_CONTENTION`, no «QA no aprobada». |
| `SddIA/scripts/qa/test-aut-67-prepush-store-lock.sh` | Lab sin red: clasificación, reintento con mock, agotamiento de presupuesto, AC4. |

## Laudos aplicados

- L-DISTINGUISH / L-EXPLICIT: contención nombrada en stderr del hook.
- L-HOOK-RETRY: presupuesto independiente de `SDDIA_EVENT_STORE_OPEN_BUDGET_MS`.
- L-WATCHER-YIELD: sin `open` de claim en tick con rutas hijas en vuelo.
- L-NO-BUDGET: sin cambio en `DEFAULT_OPEN_BUDGET_MS` del crate event-store.
