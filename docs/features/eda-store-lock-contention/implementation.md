---
feature_name: eda-store-lock-contention
created: "2026-11-10"
process: feature
branch_name: feat/eda-store-lock-contention
persist_ref: docs/features/eda-store-lock-contention
pbi_ref: MAN-42
execution_id: "cd45182d-1075-43f1-89d5-f2fda26a09f5"
---

# Implementación — eda-store-lock-contention

## Touchpoints

| Área | Cambio |
|------|--------|
| `SddIA/core/fracture-signatures.json` | Firma `F-EDA-STORE-LOCK-CONTENTION` (`suppress`, consumidores `dcc_suppress` + `mayeuta`) |
| `execute-process/src/core/store_lock_contention.rs` | Helper compartido L-SYMPTOM |
| `handlers/refine_item/run.rs` | Sin `System_Fracture_Detected` ante lock; `refined_event_pending` |
| `handlers/forge_pbi.rs` | Clasificador alineado al helper compartido |
| `eda_bus_topology.rs` | `promote_witness`: lock puro → no `move_to_dlq` |

## No-goals respetados

- `F-REFINE-ITEM-REDB-LOCK` sin reescritura.
- Sin cambio de presupuestos `DEFAULT_OPEN_BUDGET_MS` / host open budget.
- Sin rediseño MAN-38 (`with_productive_port`).
