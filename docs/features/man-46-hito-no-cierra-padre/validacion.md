---
feature_name: man-46-hito-no-cierra-padre
created: 2026-11-21
updated: 2026-11-21
process: feature
branch_name: feat/man-46-hito-no-cierra-padre
persist_ref: docs/features/man-46-hito-no-cierra-padre
pbi_ref: MAN-46
parent_ref: MAN-43
document_id: PBI-74cd5db0
phase: verificacion
agent: argos
global: APTO
verdict: aprobado
approval_status: aprobado
norm_ref: features-documentation-pattern@1.3.2
execution_id: cbf032fc-8bbf-452e-b124-a5266b7d3fc4
validated_base_sha: f102e89b6d8339a938d50e6c4e35a9783be9b005
validated_head_sha: 8db4b01984472470286a6720a9d95b16b63c68da
validated_tree_sha: 5efce7b40a3bfb7ce9d0c5d55ad177aa4f7a6569
diff_digest: sha256:e63c26738925cf77493cade8df62b6d9f72753141549e3c2c74b95a5cefb0316
---

# Validación — man-46-hito-no-cierra-padre

## global

**`APTO`** — Aduana `parental_close_gate` implementada e integrada en roll-up (`tracker_stamp.rs`), pre-sello (`delivery_close.rs`) y pre-fusión (`diff_seal.rs`), con nota normativa en `feature.md`. Todos los tests, `clippy -D warnings` y `verify-process-integrity` pasan en verde.

## checks

| Check / AC | Resultado | Evidencia |
|------------|-----------|-----------|
| **AC-1 (KA-MILE-1)** | **APTO** | `ka_mile_1_rollup_denied_when_parent_acs_open` + `tracker_stamp::maybe_complete_hu_when_children_done` impiden roll-up a `done` con AC abiertos sin refine previo. |
| **AC-2 (KA-MILE-2)** | **APTO** | `ka_mile_2_narrative_denied_with_milestone_tokens` rechaza `delivery_state: success` y narración «ciclo cerrado» con `H*-only` / `*_deferred` / `PASS_*_MILESTONE`. |
| **AC-3 (KA-MILE-3)** | **APTO** | `ka_mile_3_prior_refine_allows_success` vs `ka_mile_3_same_pass_rewrite_denied` distinguen refine previo sellado de rewrite en la misma pasada. |
| **AC-4 (KA-MILE-4)** | **APTO** | Contrafactual documentado en `execution.md` citando auditoría KZ-3. |
| **Tests & Clippy** | **APTO** | `parental_close_gate` (6/6), `tracker_stamp` (26/26), `diff_seal` (14/14), `delivery_close::tests` (38/38), `clippy -D warnings` (0 warnings), `verify-process-integrity: OK`. |

## branch

- **Rama:** `feat/man-46-hito-no-cierra-padre`
- **Persist:** `docs/features/man-46-hito-no-cierra-padre`
- **execution_id:** `cbf032fc-8bbf-452e-b124-a5266b7d3fc4`
- **pbi_ref:** `MAN-46`