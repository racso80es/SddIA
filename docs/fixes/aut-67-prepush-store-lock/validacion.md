---
feature_name: aut-67-prepush-store-lock
process: bug-fix
phase: validation
tracker_ref: AUT-67
uuid: e46d8e0c-b9fa-4d18-a035-99e9cf66ac07
persist_ref: docs/fixes/aut-67-prepush-store-lock
branch_name: Fix_Events_Multiple_02
global: APTO
pbi_archived: false
done_gate: linear
---

# Validación — aut-67-prepush-store-lock

## Criterios

| ID | Estado | Evidencia |
|----|--------|-----------|
| AC1 | Lab parcial | Reintento hook + yield watcher; push con watcher activo pendiente de operador. |
| AC2 | PASS | Lab: presupuesto agotado → exit 2 y `STORE_LOCK_CONTENTION`. |
| AC3 | PASS | `test-aut-67-prepush-store-lock.sh` + test unitario watcher. |
| AC4 | PASS | Diff no altera `DEFAULT_OPEN_BUDGET_MS` en `sddia-event-store`. |

**global: APTO** — implementación T1–T3 en rama `Fix_Events_Multiple_02`. Issue AUT-67 en Linear queda para Done humano tras merge.
