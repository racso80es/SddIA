---
feature_name: aut-97-project-git-invalid-lab-prefix
created: 2026-11-22
updated: 2026-11-22T14:15:30Z
process: pull-request-review
phase: Veredicto y bloqueo
agent: argos
agents: argos
branch: feat/aut-97-project-git-invalid-lab-prefix
branch_name: feat/aut-97-project-git-invalid-lab-prefix
persist_ref: docs/features/aut-97-project-git-invalid-lab-prefix
pbi_ref: AUT-97
document_id: PBI-FIX-FRACTURE-e76035db4846
tracker_provider: linear
tracker_ref: AUT-97
correlation_id: 2232c895-15ab-4340-83e7-015a30da61a6
pr_presented_event_id: 2232c895-15ab-4340-83e7-015a30da61a6
execution_id: cb6b60ee-cc70-49ea-a4e2-55d7d0ca15c7
norm_ref: features-documentation-pattern@1.3.1
global: APTO
pbi_archived: false
approval_status: aprobado
verdict: aprobado
delivery_state: success
resolution: PASS_F5_VERDICT
checks:
  CA-1: APTO
  CA-2: APTO
  CA-3: APTO
  CA-4: APTO
  global: APTO
  branch: APTO
  git_changes: APTO
  F2_DOC_GATE: APTO
  F3_TECH_GATE: APTO
  F4_RBAC_GATE: APTO
  F5_VERDICT: APTO
  DOC_EVOLUTION: APTO
git_changes:
  branch: feat/aut-97-project-git-invalid-lab-prefix
  base: main
  paths:
  - .gitignore
  - SddIA/engine/execute-process/src/engine/handlers/forge_hu.rs
  - SddIA/engine/execute-process/src/engine/project_binding.rs
  - SddIA/engine/execute-process/tests/fixtures/fracture-corpus/e76035db-project-git-invalid-lab-prefix.json
  - SddIA/evolution/Evolution_log.md
  - SddIA/evolution/ab3365ad-8352-4cef-9e76-7f39b1c41907.md
  - docs/audits/AUT-97.md
  - docs/features/aut-97-project-git-invalid-lab-prefix/
blocking_findings: []
validated_base_sha: b288999719a805414810e74e5ef36df512bc5967
validated_head_sha: ba52975c415223913f85be2749554d6451f83ea1
validated_tree_sha: d38f85b73f5b892d8f06f5bbe6dbda7b14f1170a
diff_digest: sha256:bf64f4a18557d5f7d1da588920606481b70655ff8c90f62d147dc5e0c9cc08f3
---

# Validación — aut-97-project-git-invalid-lab-prefix (Argos · pull-request-review)

## Dictamen

**APTO** — `resolution: PASS_F5_VERDICT` · `verdict: aprobado` · `approval_status: aprobado` · `delivery_state: success` · `blocking_findings: []`.

| Criterio | Estado | Evidencia |
| :--- | :--- | :--- |
| **CA-1** | **APTO** | `core_nested_index_exempt` en `project_binding.rs` exime `_refine_lab_`, `_forge_lab_`, `_forge_hu_` y `_linear_e2e_`. |
| **CA-2** | **APTO** | `LabProjectGuard` (`Drop`) en `forge_hu.rs` + `/_forge_hu_*/` y `/_linear_e2e_*/` en `.gitignore`. |
| **CA-3** | **APTO** | Test `bind_core_tolerates_absolute_forge_hu_and_linear_e2e_labs_in_index` (`19 passed`) + fixture `e76035db-project-git-invalid-lab-prefix.json`. |
| **CA-4** | **APTO** | `RUSTFLAGS="-D warnings" cargo check --all-targets` (`exit 0`) y `cargo build --release -p execute-process` (`exit 0`). |
| **DOC_EVOLUTION** | **APTO** | `SddIA/evolution/ab3365ad-8352-4cef-9e76-7f39b1c41907.md` + `Evolution_log.md` (`EVOL_OK`). |
| **F3_TECH_GATE** | **APTO** | `ppr-tech-triage` (`verify-process-integrity: OK`, `TECH_FORMAL_EXECUTE_PROCESS: APTO`). |

## approval_status

```text
aprobado
```