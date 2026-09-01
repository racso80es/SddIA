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
ci_run_id_sha1: "33503202496"
ci_job_integrity_sha1: "99841076343"
checks:
  CA1: PENDIENTE-CI
  CA2: APTO
  CA3: APTO
  CA4: APTO
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

PR https://github.com/racso80es/SddIA/pull/246. SHA-1 A3.2 calentamiento OK. Este commit es SHA-2 (medicion CA1/CA5). `global: PENDIENTE-CI` hasta `run_id` de este SHA. Prohibido polling (DA-6).

## SHA-1 calentamiento (`43dd0bb`)

Run [33503202496](https://github.com/racso80es/SddIA/actions/runs/33503202496) job [99841076343](https://github.com/racso80es/SddIA/actions/runs/33503202496/job/99841076343). Checks SUCCESS.

| Step | Duracion |
| :--- | ---: |
| Build native workspace | 537 s |
| verify-compiled-capsules | 0 s |
| LanceDB | 550 s |
| Job | 1110 s (18 m 30 s) |

sccache: Compile requests **1029**, executed 853, hits 221 (26 %, Rust 158), misses 628. Cache location **ghac** (no Local disk). `RUSTC_WRAPPER=sccache` presente en steps cargo. Gate de honestidad PBI § A3.2 **cumplido**. SHA-1 no sella CA1/CA5.

## CA2 / CA3 / CA6 / CA7

APTO. Gate 29 bins. 16 tests. A0 = 33477170741. Hermanos verdes en 33503202496. Evolution `530039c9-100b-413a-b3d5-ca632d83acc6`.

## CA4

Wrapper + GHA backend empiricos. `CARGO_INCREMENTAL=0` sostenido. `SddIA/target` fuera del blob.

## CA1 / CA5

Pendientes del `pull_request` de **este** SHA. Umbrales: suma Build+verify < 60 s o techo 170 s; LanceDB < 60 s o techo 180 s; job < 8 min. Hits sccache del SHA-2 deben ser > 0.
