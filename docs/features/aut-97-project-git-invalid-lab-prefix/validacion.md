---
feature_name: aut-97-project-git-invalid-lab-prefix
created: 2026-11-22
updated: 2026-11-22T14:12:30Z
process: feature
agent: argos
branch: feat/aut-97-project-git-invalid-lab-prefix
branch_name: feat/aut-97-project-git-invalid-lab-prefix
persist_ref: docs/features/aut-97-project-git-invalid-lab-prefix
pbi_ref: AUT-97
document_id: PBI-FIX-FRACTURE-e76035db4846
tracker_provider: linear
tracker_ref: AUT-97
execution_id: cb6b60ee-cc70-49ea-a4e2-55d7d0ca15c7
norm_ref: features-documentation-pattern@1.3.1
global: APTO
pbi_archived: false
checks:
  CA-1: APTO
  CA-2: APTO
  CA-3: APTO
  CA-4: APTO
  global: APTO
  branch: APTO
  git_changes: APTO
git_changes:
  branch: feat/aut-97-project-git-invalid-lab-prefix
  base: main
  paths:
  - .gitignore
  - SddIA/engine/execute-process/src/engine/handlers/forge_hu.rs
  - SddIA/engine/execute-process/src/engine/project_binding.rs
  - SddIA/engine/execute-process/tests/fixtures/fracture-corpus/e76035db-project-git-invalid-lab-prefix.json
  - docs/audits/AUT-97.md
  - docs/features/aut-97-project-git-invalid-lab-prefix/
validated_base_sha: b288999719a805414810e74e5ef36df512bc5967
validated_head_sha: 7d6520ffc6edccdc70be33fc4197376c1f7c609e
validated_tree_sha: a79735a7260faebe3a2ef2ce685d68ce3d286d9d
diff_digest: sha256:1f40b8940bacdbd96494904afc53b372bfb754b0ffe37c6768dcf0608a107336
---

# Validación — aut-97-project-git-invalid-lab-prefix

## Dictamen

**APTO** — Todos los criterios de aceptación (`CA-1`..`CA-4`) verificados.

| Criterio | Estado | Evidencia |
| :--- | :--- | :--- |
| **CA-1** | **APTO** | `core_nested_index_exempt` en `project_binding.rs` exime `_refine_lab_`, `_forge_lab_`, `_forge_hu_` y `_linear_e2e_`. |
| **CA-2** | **APTO** | `LabProjectGuard` (`Drop`) en `forge_hu.rs` + `/_forge_hu_*/` y `/_linear_e2e_*/` en `.gitignore`. |
| **CA-3** | **APTO** | Test `bind_core_tolerates_absolute_forge_hu_and_linear_e2e_labs_in_index` (`19 passed`) + fixture `e76035db-project-git-invalid-lab-prefix.json`. |
| **CA-4** | **APTO** | `RUSTFLAGS="-D warnings" cargo check --all-targets` (`exit 0`) y `cargo build --release -p execute-process` (`exit 0`). |