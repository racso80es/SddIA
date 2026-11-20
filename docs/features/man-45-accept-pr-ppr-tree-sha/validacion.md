---
feature_name: man-45-accept-pr-ppr-tree-sha
created: 2026-11-20
updated: 2026-11-20
process: feature
branch_name: feat/man-45-accept-pr-ppr-tree-sha
persist_ref: docs/features/man-45-accept-pr-ppr-tree-sha
pbi_ref: MAN-45
parent_ref: MAN-43
document_id: PBI-75d12fd0
phase: verificacion
agent: argos
global: APTO
verdict: aprobado
approval_status: aprobado
norm_ref: features-documentation-pattern@1.3.2
execution_id: 73910788-b7e7-4a61-9ac2-72fedd024b2a
validated_base_sha: bc7255e83c7ab7adb34e3d1369c4d6d7bb59265c
validated_head_sha: 7d1af9b28338ee45fdbd09887b456497917e2623
validated_tree_sha: b2bd8dfcb9e6b5954ee68e992496e11650a602a1
diff_digest: sha256:86ed5861788dc7f2fc417874d66111641be0945fcf890fd8af309ec46ffae007
---

# Validación — man-45-accept-pr-ppr-tree-sha

## global

**`APTO`** — `accept-pr` exige atestación `qa-attestation/1.0` (`verdict: aprobado`, `issuer_process: pull-request-review`, `tree_sha` coincidente y `qa_profile: full` para delta `active`), rechazando `docs-only`, escrituras fuera de PPR y `skipped-ppr-dedupe`. Todos los tests, `clippy -D warnings` y `verify-process-integrity` en verde.

## checks

| Check / AC | Resultado | Evidencia |
|------------|-----------|-----------|
| **KA-PPR-1** | **APTO** | `ka_ppr_1_missing_attestation_aborts` y `ka_ppr_1_genomic_audit_aborts_without_attestation` abortan sin atestación. |
| **KA-PPR-2** | **APTO** | `ka_ppr_2_docs_only_or_forged_full_aborts` bloquea `docs-only` con delta `active` y `full` sin `issuer_process: pull-request-review`. |
| **KA-PPR-3** | **APTO** | `ka_ppr_3_tree_mismatch_aborts` bloquea si `tree_sha` diverge. |
| **KA-PPR-4** | **APTO** | `ka_ppr_4_dedupe_not_approval` rechaza `skipped-ppr-dedupe` como aprobación. |
| **KA-PPR-5** | **APTO** | Contrafactual en `ka-ppr-5-counterfactual.md`. |
| **Tests & Clippy** | **APTO** | `qa_attestation` (15/15), `accept_pr` (22/22), `clippy -D warnings` (0 warnings), `verify-process-integrity: OK`. |

## branch

- **Rama:** `feat/man-45-accept-pr-ppr-tree-sha`
- **Persist:** `docs/features/man-45-accept-pr-ppr-tree-sha`
- **execution_id:** `73910788-b7e7-4a61-9ac2-72fedd024b2a`
- **pbi_ref:** `MAN-45`