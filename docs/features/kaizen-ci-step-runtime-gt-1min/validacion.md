---
feature_name: kaizen-ci-step-runtime-gt-1min
created: "2026-09-01"
process: feature
phase: validate
agents: argos
branch: feat/kaizen-ci-step-runtime-gt-1min
branch_name: feat/kaizen-ci-step-runtime-gt-1min
persist_ref: docs/features/kaizen-ci-step-runtime-gt-1min
pbi_ref: docs/todos/done/[KAIZEN] CI — optimizar steps >1 min (verify-compiled-capsules y LanceDB).md
document_id: PBI-KAIZEN-CI-STEP-RUNTIME-GT-1MIN
uuid: "530039c9-100b-413a-b3d5-ca632d83acc6"
global: PENDIENTE-CI
pbi_archived: true
checks:
  CA1: PENDIENTE-CI
  CA2: APTO
  CA3: APTO
  CA4: APTO
  CA5: PENDIENTE-CI
  CA6: PENDIENTE-CI
  CA7: APTO
git_changes:
  - .github/workflows/sddia-index-qa.yml
  - SddIA/engine/execute-process/tests/memory_evolution_ingest.rs
  - SddIA/engine/execute-process/src/engine/memory_evolution_ingest_core.rs
  - SddIA/evolution/530039c9-100b-413a-b3d5-ca632d83acc6.md
  - SddIA/evolution/Evolution_log.md
  - docs/features/kaizen-ci-step-runtime-gt-1min/clarify.md
  - docs/features/kaizen-ci-step-runtime-gt-1min/objectives.md
  - docs/features/kaizen-ci-step-runtime-gt-1min/spec.md
  - docs/features/kaizen-ci-step-runtime-gt-1min/plan.md
  - docs/features/kaizen-ci-step-runtime-gt-1min/implementation.md
  - docs/features/kaizen-ci-step-runtime-gt-1min/execution.md
  - docs/features/kaizen-ci-step-runtime-gt-1min/validacion.md
  - docs/todos/done/[KAIZEN] CI — optimizar steps >1 min (verify-compiled-capsules y LanceDB).md
---

# Validación — kaizen-ci-step-runtime-gt-1min

## CA2

16 tests locales: 3 memory + 5 thought + 5 evolution + 3 ingest integración. Gate no mutado (29 bins).

## CA3

A0 SSOT en PBI v1.1.0 (run 33477170741). Tabla de cierre = este PR (steps del workflow).

## CA4

Decisión: save `target/` solo en integrity (`native-integrity-*`); IOTA `lookup-only`. Números de restore en el `run_id` de CA1.

## CA7

Sin mutar `SddIA/tools/`. Evolution `530039c9-100b-413a-b3d5-ca632d83acc6`.

## CA1 / CA5 / CA6

Gates de GitHub Actions: `PENDIENTE-CI` hasta `run_id` de `pull_request` verde (norma features-documentation-pattern § CA de CI). Prohibido polling (DA-6).
