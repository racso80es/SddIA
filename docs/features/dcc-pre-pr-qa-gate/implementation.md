---
feature_name: dcc-pre-pr-qa-gate
created: "2026-10-30"
process: feature
pbi_ref: AUT-38
---

# Implementación

- `SddIA/engine/execute-process/src/engine/qa_attestation.rs`: `worktree_violations_except_validacion`, `validate_attestation_for_branch_head`.
- `SddIA/engine/execute-process/src/engine/delivery_close.rs`: `capsule_gate_pre_pr_qa`, supresión fractura pre-PR.
- `SddIA/library/codexes/codex-software-engineering/process/delivery-close-cycle.md`: v1.5.0, fase Gate pre-PR QA.
