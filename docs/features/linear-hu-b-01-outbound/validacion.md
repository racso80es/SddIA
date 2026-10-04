---
feature_name: linear-hu-b-01-outbound
persist_ref: docs/features/linear-hu-b-01-outbound
pbi_ref: docs/todos/done/[ARQUITECTURA] Linear HU-B 01 — Registro saliente anti-eco.md
document_id: PBI-LINEAR-B-01-OUTBOUND
branch: feat/linear-hu-b-01-outbound
global: APTO
pbi_archived: true
pr_url: https://github.com/racso80es/SddIA/pull/336
---
# Validación — HU-B 01

| Criterio | Estado |
|----------|--------|
| AC-OUT-1 | APTO — tests `ac_out_1_*` en `tracker_outbound` |
| AC-OUT-2 | APTO — `ac_out_2_line_has_no_token_or_graphql_body` |
| CI `eda-bus-e2e-smoke` | APTO — `run-linear-direct-cycle-e2e-lab` (fix WASI lab + env `SDDIA_*`) |

| Gate documental | Estado |
|-----------------|--------|
| PBI en `docs/todos/done/` | APTO |
| `pbi_archived: true` | APTO |
