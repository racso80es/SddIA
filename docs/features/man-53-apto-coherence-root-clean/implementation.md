---
feature_name: man-53-apto-coherence-root-clean
created: "2026-11-22"
process: feature
branch_name: feat/man-53-apto-coherence-root-clean
persist_ref: docs/features/man-53-apto-coherence-root-clean
tracker_ref: MAN-53
items:
  - "SddIA/engine/execute-process/src/engine/apto_coherence.rs"
  - "SddIA/engine/execute-process/src/engine/qa_attestation.rs"
  - "SddIA/engine/execute-process/src/engine/pull_request_review.rs"
  - "SddIA/engine/execute-process/src/engine/residual_runner.rs"
  - "SddIA/engine/execute-process/src/engine/delivery_close.rs"
  - "SddIA/engine/execute-process/src/engine/accept_pr.rs"
---
# Implementación — man-53-apto-coherence-root-clean

## Touchpoints implementados

1. **`SddIA/engine/execute-process/src/engine/apto_coherence.rs`:**
   - Constantes `FRICTION_AC_PARTIAL` (`apto_coherence_ac_partial`) y `FRICTION_TRUNK_GATE` (`apto_coherence_trunk_gate`).
   - `check_c5_ac_matrix`: escanea tablas Markdown y listas `AC-x`/`CA-x` en `execution.md` y `validacion.md` buscando estados `PARCIAL`, `FAIL`, `NO_APTO`, `ABIERTO` o `PENDIENTE` (incl. `PENDIENTE-*`).
   - `check_trunk_gates`: verifica que en `validacion.md` (`checks:`) ninguno de `F2_DOC_GATE`, `F3_TECH_GATE`, `F4_RBAC_GATE`, `ROOT_TOPOLOGY_CLEAN` o `DOC_HONESTY_*` figure en `NO_APTO`.
   - Tests: `ka_ac_partial_1_matrix_rejects_apto`, `ka_ac_partial_1_passing_matrix_with_notes_passes`, `ka_gate_trunk_2_no_apto_rejects_apto`, `ka_close_4_man50_counterfactual`.
2. **`SddIA/engine/execute-process/src/engine/qa_attestation.rs`:**
   - `ROOT_EPHEMERAL_DIRS` y `root_ephemeral_dir_violations`: detecta presencia física en raíz (incluso vacíos) de `.events`, `orchestration`, `telemetry`, `pending`, `domain`, `dead-letter`, `progress`, `_lab`.
   - `try_write_from_ppr`: ejecuta `apto_coherence::validate_apto_coherence` antes de emitir atestación.
   - Tests: `ka_root_phys_3_empty_ephemeral_dirs_detected`, `try_write_from_ppr_blocks_on_trunk_gate_no_apto`.
3. **`SddIA/engine/execute-process/src/engine/pull_request_review.rs` & `residual_runner.rs`:**
   - `handoff_accept_pr` y cierre de `pull-request-review` en `residual_runner` invocan `validate_apto_coherence` cuando existe `persist_ref`, bloqueando aprobación/handoff si hay incoherencia.
   - Test: `ka_gate_trunk_2_handoff_blocks_on_f3_no_apto`.
4. **`SddIA/engine/execute-process/src/engine/delivery_close.rs` & `accept_pr.rs`:**
   - `capsule_gate_pre_pr_qa` (`delivery-close-cycle`) y `Auditoría Genómica` (`accept-pr`) verifican `root_ephemeral_dir_violations(repo)` bloqueando con `root_topology_dirty` sin emitir `System_Fracture_Detected`.
   - Tests: `ka_root_phys_3_dcc_gate_blocks_empty_ephemeral_dir_without_fracture`, `ka_root_phys_3_genomic_audit_aborts_on_empty_ephemeral_dir`.
