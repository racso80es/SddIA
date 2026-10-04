---
feature_name: linear-hu-a-09-e2e-lab
document_id: PBI-LINEAR-A-09-E2E
branch: feat/linear-hu-a-09-e2e-lab
---
# Implementación — HU-A 09

- `SddIA/tools/linear-tracker-adapter/src/lab_state.rs` — issues/comentarios mock en disco.
- `SddIA/tools/linear-tracker-adapter/src/main.rs` — create/fetch/update contra store lab.
- `SddIA/engine/execute-process/src/engine/linear_direct_cycle_e2e.rs` — orquestación AC-7..10.
- `SddIA/tools/sddia-qa/src/linear_direct_cycle_e2e_lab.rs` — wrapper QA.
- Tests lab: `SDDIA_REPO_ROOT` + borrado de store al inicio/fin (no `SDDIA_LAB_RESET_STORE` por invocación).
