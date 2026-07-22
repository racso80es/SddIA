---
feature_name: ppr-rehab-revoked-entities
created: "2026-07-22"
process: bug-fix
branch_name: fix/ppr-rehab-revoked-entities
persist_ref: docs/fixes/ppr-rehab-revoked-entities
agents: tekton
uuid: 23a81b0e-3930-4589-b5db-25ddd8eb5717
---

# Implementation

| # | Cambio | Estado |
|---|--------|--------|
| 1 | Instancia: remove `pull-request-review` de `revoked_entities.json` | done |
| 2 | Instancia: `entities.pull-request-review.status=healthy` + poda outlier | done |
| 3 | `radamanto_batch_core.rs`: `is_latency_threshold_exempt` | done |
| 4 | Test unitario exención | done |
| 5 | Archivo conjunto PBI #124 + #125 → `done/` | done |
