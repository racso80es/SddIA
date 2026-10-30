---
feature_name: dcc-pre-pr-qa-gate
created: "2026-10-30"
process: feature
pbi_ref: AUT-38
---

# Especificación

## Motor

- Fase `Gate pre-PR QA` en `delivery-close-cycle` v1.5.0 (antes de Publicación remota).
- Handler `capsule_gate_pre_pr_qa` en `delivery_close.rs`.
- Reutiliza `qa_attestation::validate_attestation_for_branch_head` y `worktree_violations_except_validacion`.
- Lab: `SDDIA_LAB_SKIP_PRE_PR_QA_GATE`.

## Envelope

- `status: blocked`, `friction_id: qa_attestation_required` | `worktree_not_closed`.
- Supresión de fractura en `emit_dcc_phase_fractures`.
