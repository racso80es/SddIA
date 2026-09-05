---
feature_name: dcc-snapshot-gitignore-1e62e8b851f8
created: "2026-09-05"
process: bug-fix
document_id: PBI-FIX-FRACTURE-1e62e8b851f8
items:
  - snapshot-filter
  - git-manager-skip-ignored
  - resolve-head
  - mayeuta-cube
---

# Implementación — 1e62e8b851f8

| Item | Path |
|------|------|
| FIX-1 | `phase_capsules.rs` `snapshot_path_is_ignored_vault` |
| FIX-2 | `git-manager/src/main.rs` `git_add_rejected_as_ignored` |
| FIX-3 | `delivery_close.rs` `resolve_symbolic_head_branch`; `capsule_delivery_gh_pr` |
| FIX-3b | `hook_common.sh` `ref_to_branch` |
| FIX-4 | `enrich_fracture_pbi_kaizen.rs` |
