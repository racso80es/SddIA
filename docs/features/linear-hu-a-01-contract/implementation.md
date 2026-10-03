---
feature_name: linear-hu-a-01-contract
created: "2026-10-03"
process: feature
---

# Implementación

| Touchpoint | Detalle |
|------------|---------|
| Contrato | `tracker.state_map` admite `todo`; labels `fix`, `kaizen`, `deuda`, `spike`, `editable`; `done_gate` ∈ {git, linear, both} |
| Binding | `CONTRACT_VERSION` = `1.3.0`; `TRACKER_STATE_KEYS` incluye `todo`; `DONE_GATE_HU_A_ONLY_GIT` rechaza linear/both |
| Tests | `contract_accepts_1_3_0_tracker_todo_and_done_gate_git`, `contract_1_3_0_rejects_done_gate_linear_until_hu_b` |

Verificación: `cargo test -p execute-process contract_accepts_1_3_0`.
