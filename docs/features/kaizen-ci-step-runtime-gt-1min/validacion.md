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
global: APTO
pbi_archived: true
pr_url: https://github.com/racso80es/SddIA/pull/246
ci_run_id: "33506016317"
ci_run_event: pull_request
ci_job_integrity: "99850119868"
ci_run_id_sha1: "33503202496"
ci_job_integrity_sha1: "99841076343"
checks:
  CA1: APTO
  CA2: APTO
  CA3: APTO
  CA4: APTO
  CA5: APTO
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
  - docs/todos/done/[KAIZEN] CI — optimizar steps >1 min (verify-compiled-capsules y LanceDB).md
---

# Validacion — kaizen-ci-step-runtime-gt-1min

PR https://github.com/racso80es/SddIA/pull/246. `global: APTO`. Medicion SHA-2 `25e19c7` run [33506016317](https://github.com/racso80es/SddIA/actions/runs/33506016317) job [99850119868](https://github.com/racso80es/SddIA/actions/runs/33506016317/job/99850119868).

## SHA-1 calentamiento (`43dd0bb`)

Run [33503202496](https://github.com/racso80es/SddIA/actions/runs/33503202496) job [99841076343](https://github.com/racso80es/SddIA/actions/runs/33503202496/job/99841076343). sccache: 1029 requests, 221 hits (26 %), location ghac. Build 537 s, LanceDB 550 s, job 18 m 30 s. No sella CA1/CA5.

## SHA-2 medicion (`25e19c7`)

| Presupuesto | Baseline | SHA-2 | Umbral | Veredicto |
| :--- | ---: | ---: | :--- | :--- |
| Build + verify | 340 s | **160 s** | techo 170 s | CA1 APTO |
| LanceDB | 361 s | **92 s** | techo 180 s | CA1 APTO |
| Job | 816 s | **277 s (4 m 37 s)** | < 8 min | CA5 APTO |

sccache SHA-2: 1029 requests, 745 hits (87,75 %, Rust 85 %), 104 misses, location ghac.

## CA3

A0 = 33477170741. Tabla = este run. **Diferir** `wasi-runtime-smoke` cache restore **94 s** (baseline A0 43 s; keys `wasi-*` fuera de A3.2). IOTA simulate Build 114 s (hits 100 %) residual de link; diferir.

## CA2 / CA4 / CA6 / CA7

Gate 29 bins. 16 tests. Wrapper + GHA. Hermanos verdes. iota-physical SUCCESS; smoke 0 s (secret). Sin mutar `SddIA/tools/`. Evolution `530039c9-100b-413a-b3d5-ca632d83acc6`.
