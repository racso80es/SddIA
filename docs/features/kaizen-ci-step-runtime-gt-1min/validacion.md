---
feature_name: kaizen-ci-step-runtime-gt-1min
created: "2026-09-01"
process: feature
phase: validate
agents: argos
branch: feat/kaizen-ci-step-runtime-gt-1min
branch_name: feat/kaizen-ci-step-runtime-gt-1min
persist_ref: docs/features/kaizen-ci-step-runtime-gt-1min
pbi_ref: docs/todos/pending/[KAIZEN] CI — optimizar steps >1 min (verify-compiled-capsules y LanceDB).md
document_id: PBI-KAIZEN-CI-STEP-RUNTIME-GT-1MIN
uuid: "530039c9-100b-413a-b3d5-ca632d83acc6"
global: PENDIENTE-CI
pbi_archived: false
pr_url: https://github.com/racso80es/SddIA/pull/246
checks:
  CA1: PENDIENTE-CI
  CA2: APTO
  CA3: APTO
  CA4: PENDIENTE-CI
  CA5: PENDIENTE-CI
  CA6: APTO
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
  - docs/todos/pending/[KAIZEN] CI — optimizar steps >1 min (verify-compiled-capsules y LanceDB).md
---

# Validacion — kaizen-ci-step-runtime-gt-1min

PR https://github.com/racso80es/SddIA/pull/246. Ola A3.2 SHA-1 (calentamiento). `global: PENDIENTE-CI`. Prohibido APTO de CA1/CA5 en este SHA. Prohibido polling (DA-6).

## CA2 / CA3 / CA6 / CA7

Gate 29 bins. 16 tests locales. A0 = 33477170741. Hermanos verdes en runs previos del mismo PR. Evolution `530039c9-100b-413a-b3d5-ca632d83acc6`. Sin mutar `SddIA/tools/`.

## CA4 / CA1 / CA5

SHA-1 debe mostrar `sccache --show-stats` con Compile requests > 0 y cache location GHA (no `Local disk`). SHA-2 sella umbrales. A3.1 (`867bf5d`) no cuenta: 0 requests.
