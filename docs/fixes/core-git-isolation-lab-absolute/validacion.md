---
feature_name: core-git-isolation-lab-absolute
created: 2026-10-30
process: bug-fix
branch_name: fix/core-git-isolation-lab-absolute
persist_ref: docs/fixes/core-git-isolation-lab-absolute
pbi_ref: AUT-6
tracker_ref: AUT-6
execution_id: 652511ad-2740-49f1-8ef5-ef9397fadefe
global: APTO
pbi_archived: false
branch: fix/core-git-isolation-lab-absolute
pr_url: https://github.com/racso80es/SddIA/pull/423
validated_base_sha: afc722d821c9f991511e527a99b3193864cc090c
validated_head_sha: 0bb9f308bd43efec2689280e9eb271617bab21c6
validated_tree_sha: 24aa338d5b985fdf35c8416a2af82da01b9369ca
diff_digest: sha256:9b9c567a5a50913c74452a4250651ff1a02e3613e0973a360d98b1112e68a053
---

# Validación — AUT-6

| Check | Resultado |
|-------|-----------|
| AC-1 | `bind_core_tolerates_absolute_refine_lab_in_index` OK |
| AC-2 | `bind_core_still_rejects_absolute_nested_real_project` OK |
| `cargo test -p execute-process project_binding:: --lib` | 17/17 OK |
| Evolution | `d5fbe8a2-0b49-458d-bc5f-5d3f1e2025ef` |
| `gate-evolution --range` | EVOL_OK |
| PR | https://github.com/racso80es/SddIA/pull/423 |

Cierre Linear vía merge de PR (`done_gate: linear`).