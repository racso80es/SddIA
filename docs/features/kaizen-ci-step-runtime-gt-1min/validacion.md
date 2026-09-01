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
pr_url: https://github.com/racso80es/SddIA/pull/246
ci_run_id_miss: "33493563587"
ci_run_id_hit_a30: "33495498463"
ci_job_integrity_a30: "99816608487"
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
  - docs/todos/done/[KAIZEN] CI — optimizar steps >1 min (verify-compiled-capsules y LanceDB).md
---

# Validacion — kaizen-ci-step-runtime-gt-1min

PR https://github.com/racso80es/SddIA/pull/246. Iteracion A3.1 en vuelo. `global: PENDIENTE-CI` hasta `run_id` de hit sccache.

## A3.0 (cerrada)

Hit key [33495498463](https://github.com/racso80es/SddIA/actions/runs/33495498463) job [99816608487](https://github.com/racso80es/SddIA/actions/runs/33495498463/job/99816608487) `754c575`: Build 429 s, LanceDB 419 s, job 15 m 11 s. CA1/CA5 **NO_APTO** en esa ola. `lookup-only` no restaura (IOTA ~8 min vs baseline 86 s).

## A3.1 (esta iteracion)

Palancas: sccache GHA; no cachear `SddIA/target`; IOTA `actions/cache/restore@v4`; key + rustc hash; `CARGO_INCREMENTAL=0`. spec v1.1.0.

Primer SHA A3.1 = miss sccache. CA1/CA5 se sellan en el segundo `pull_request` con hit. Prohibido polling (DA-6).

## CA2 / CA3 / CA6 / CA7

Sin cambio de cobertura ni gate. A0 = 33477170741. Hermanos verdes en A3.0. Evolution `530039c9-100b-413a-b3d5-ca632d83acc6`. Sin mutar `SddIA/tools/`.

## CA4

Decision A3.1 documentada: `target/` no paga; compile = sccache; save registry/git solo en integrity.
