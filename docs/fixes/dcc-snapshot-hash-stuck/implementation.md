---
feature_name: dcc-snapshot-hash-stuck
created: "2026-10-31"
process: bug-fix
branch_name: Fix_Events_Multiple
persist_ref: docs/fixes/dcc-snapshot-hash-stuck
tracker_ref: AUT-22
items_applied:
  - snapshot-branch-mismatch-guard
  - snapshot-hash-advanced-head-and-branch
  - fracture-signature-dcc-snapshot-hash-stuck
  - enrich-fracture-corpus-51dc7bdfd2fe
---

# Implementación — AUT-22

| Artefacto | Cambio |
|-----------|--------|
| `phase_capsules.rs` | `parse_star_branch_from_branch_list`, `snapshot_branch_aligned`, `snapshot_hash_advanced`; guardia pre-commit; comparación HEAD + ref; log de commit en fallo |
| `fracture-signatures.json` | `F-DCC-SNAPSHOT-HASH-STUCK`; exclusión en `F-DCC-HOOK-RECURSION` |
| `enrich_fracture_pbi_kaizen.rs` | Corpus `51dc7bdfd2fe` |
| `SddIA/evolution/c8f3a1b2-4d5e-6f78-9abc-def012345678.md` | Hito evolutivo |

Tests: `parse_star_branch_from_branch_list_local`, `snapshot_branch_aligned_requires_exact_name`, `snapshot_hash_advanced_requires_both_refs_move`, `snapshot_branch_mismatch_sets_error_code`, `fracture_corpus_regression`.
