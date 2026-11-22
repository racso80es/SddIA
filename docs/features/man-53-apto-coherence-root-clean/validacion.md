---

feature_name: man-53-apto-coherence-root-clean
created: 2026-11-22
process: feature
branch_name: feat/man-53-apto-coherence-root-clean
persist_ref: docs/features/man-53-apto-coherence-root-clean
tracker_ref: MAN-53
branch: feat/man-53-apto-coherence-root-clean
global: APTO
verdict: aprobado
pbi_archived: false
checks:
  F2_DOC_GATE: APTO
  F3_TECH_GATE: APTO
  F4_RBAC_GATE: APTO
  ROOT_TOPOLOGY_CLEAN: APTO
  KA_AC_PARTIAL_1: APTO
  KA_GATE_TRUNK_2: APTO
  KA_ROOT_PHYS_3: APTO
  KA_CLOSE_4: APTO
git_changes:
- SddIA/engine/execute-process/src/engine/apto_coherence.rs
- SddIA/engine/execute-process/src/engine/qa_attestation.rs
- SddIA/engine/execute-process/src/engine/pull_request_review.rs
- SddIA/engine/execute-process/src/engine/residual_runner.rs
- SddIA/engine/execute-process/src/engine/delivery_close.rs
- SddIA/engine/execute-process/src/engine/accept_pr.rs
- docs/features/man-53-apto-coherence-root-clean/
validated_base_sha: 22ad1ba493ade3545ecaa5aeb221ba927ade4654
validated_head_sha: de5e31dee2a6eaefffe4d5287c8804559f3c590b
validated_tree_sha: b208c2cbfdbe8e3596996fe96efac3ae072a69aa
diff_digest: sha256:243cc494644d7369f172a7dfccebbd632d9ab13cc25b71f8c065d097c9d3df5b
execution_id: "a3971880-ee8b-49c3-97d4-40f61477aa47"
---
# Validación — man-53-apto-coherence-root-clean

## Veredicto

- **Global:** `APTO`
- **Contrafactual MAN-50 (`KA-CLOSE-4`):** Con las reglas C5 (`apto_coherence_ac_partial`), veto de checks troncales (`apto_coherence_trunk_gate`) y detección física de directorios efímeros en raíz (`root_topology_dirty`), el cierre de MAN-50 bajo las condiciones de H-NEW-06 habría sido rechazado en `apto_coherence`, `pull-request-review`, `delivery-close-cycle` y `accept-pr`.

## Matriz de Criterios

| Criterio | Estado | Evidencia |
|----------|--------|-----------|
| AC-1 (`KA-AC-PARTIAL-1`) | **APTO** | `ka_ac_partial_1_matrix_rejects_apto` |
| AC-2 (`KA-GATE-TRUNK-2`) | **APTO** | `ka_gate_trunk_2_no_apto_rejects_apto`, `ka_gate_trunk_2_handoff_blocks_on_f3_no_apto` |
| AC-3 (`KA-ROOT-PHYS-3`) | **APTO** | `ka_root_phys_3_empty_ephemeral_dirs_detected`, `ka_root_phys_3_dcc_gate_blocks_empty_ephemeral_dir_without_fracture`, `ka_root_phys_3_genomic_audit_aborts_on_empty_ephemeral_dir` |
| AC-4 (`KA-CLOSE-4`) | **APTO** | `ka_close_4_man50_counterfactual` |
| AC-5 (Regresión) | **APTO** | `cargo test -p execute-process --lib` + `verify-process-integrity: OK` |