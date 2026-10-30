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
validated_head_sha: 8e440e080f94c751236c3f04a7269b9e66601e75
validated_tree_sha: 3811f4e827e7d03b3fc930f6127ecc61bbfef35d
diff_digest: sha256:866164587c10ac0cb94eedaf0f691bc963f23c4a62579bb6e3b58e6c44c80e44
---

# Validación — AUT-6

| Check | Resultado |
|-------|-----------|
| AC-1 | `bind_core_tolerates_absolute_refine_lab_in_index` OK |
| AC-2 | `bind_core_still_rejects_absolute_nested_real_project` OK |
| `cargo test -p execute-process project_binding:: --lib` | 17/17 OK |
| Evolution | `d5fbe8a2-0b49-458d-bc5f-5d3f1e2025ef` |
| `gate-evolution --range` | EVOL_OK |
| PPR lab (`60a6e3a7`) | Triaje técnico APTO; QA attestation en proofs (relevo IDE) |
| PR | https://github.com/racso80es/SddIA/pull/423 |

Cierre Linear vía merge de PR (`done_gate: linear`).