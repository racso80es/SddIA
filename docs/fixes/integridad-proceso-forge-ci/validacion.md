---
feature_name: integridad-proceso-forge-ci
created: "2026-08-29"
process: bug-fix
branch_name: fix/integridad-proceso-forge-ci
persist_ref: docs/fixes/integridad-proceso-forge-ci
pbi_ref: docs/todos/done/[FIX] Integridad de proceso — parse_frontmatter ciego, hash forge divergente y aduana CI opaca.md
document_id: PBI-FIX-INTEGRIDAD-PROCESO-FORGE-CI
uuid: 5a049a19-29ae-4c3b-adb0-a8b4e8d042fb
global: APTO
pbi_archived: true
branch: fix/integridad-proceso-forge-ci
checks:
  CA1_parse_frontmatter_test: APTO
  CA2_forge_hash_phases_test: APTO
  CA3_ci_job_rename: APTO
  CA4_dcc_aduana_indices: APTO
  CA5_workspace_template: APTO
  verify_process_integrity: APTO
  verify_tools_index: APTO
  index_integrity_gate_unit: APTO
git_changes:
  - SddIA/engine/execute-process/src/forges/common.rs
  - SddIA/engine/execute-process/src/forges/factory.rs
  - SddIA/engine/execute-process/src/engine/phase_capsules.rs
  - SddIA/engine/execute-process/src/engine/delivery_close.rs
  - SddIA/engine/execute-process/src/engine/residual_runner.rs
  - SddIA/library/codexes/codex-software-engineering/process/delivery-close-cycle.md
  - SddIA/library/codexes/codex-software-engineering/process/index.md
  - SddIA/core/eda-coverage.json
  - .github/workflows/sddia-index-qa.yml
  - docs/fixes/integridad-proceso-forge-ci/
  - SddIA/evolution/3347c0b0-99a2-45ca-a1dd-97c6ebc8298a.md
  - SddIA/evolution/Evolution_log.md
---

# Validación — integridad proceso forge/CI/DCC

**Veredicto global: APTO**

## Criterios de aceptación

| CA | Criterio | Estado | Evidencia |
|----|----------|--------|-----------|
| CA1 | Test `parse_frontmatter` con `workspace_template` …/--- | **APTO** | `parse_frontmatter_reads_uuid_when_workspace_template_ends_with_delimiter` |
| CA2 | Body replacement sella hash de `phases` | **APTO** | `process_forge_body_replacement_seals_phases_hash_not_artifact_hash` |
| CA3 | Job CI renombrado | **APTO** | `sddia-index-integrity` en `sddia-index-qa.yml` |
| CA4 | DCC fase «Aduana integridad índices» | **APTO** | DCC v1.3.0 + `capsule_index_integrity_audit_gate`; test hash corrupto → `blocked` |
| CA5 | `workspace_template` sin `---` terminal | **APTO** | Forja `entity-manager` |

## Verificación runtime

```text
cargo test -p execute-process parse_frontmatter_reads     OK
cargo test -p execute-process body_replacement            OK
cargo test -p execute-process index_integrity_gate        OK (2 tests)
sddia-qa verify-process-integrity                         OK
sddia-qa verify-tools-index                             OK
```

## Notas

- **Branch protection:** antes del merge, sustituir required check `verify-tools-index` por `sddia-index-integrity`.
- Smoke CA4 manual (DCC bloquea con hash corrupto antes de push): cubierto por test unitario `index_integrity_gate_blocks_corrupt_process_hash`; smoke E2E DCC queda al cierre vía `delivery-close-cycle`.
