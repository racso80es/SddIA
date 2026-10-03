---
feature_name: project-tracker-contract
process: feature
branch: feat/tracker-operations-context
global: APTO
pbi_archived: true
pbi_ref: docs/todos/done/[ARQUITECTURA] Tracker — contrato de proyecto tracker.md
---

# Validación — project-tracker-contract

- `project-config-contract.md` v1.2.0 con `tracker.*`.
- `project_binding.rs` admite `1.0.0` | `1.1.0` | `1.2.0` y valida `tracker` (provider, team_key, state_map, labels).
- Tests `contract_accepts_1_2_0_tracker_and_rejects_bad_provider` y regresión 1.1.0.
