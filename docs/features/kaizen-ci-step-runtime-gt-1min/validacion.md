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
global: NO_APTO
pbi_archived: true
pr_url: https://github.com/racso80es/SddIA/pull/246
ci_run_id: "33498554132"
ci_run_event: pull_request
ci_job_integrity: "99827293302"
ci_run_id_hit_a30: "33495498463"
checks:
  CA1: NO_APTO
  CA2: APTO
  CA3: APTO
  CA4: APTO
  CA5: NO_APTO
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

PR https://github.com/racso80es/SddIA/pull/246. Checks funcionales SUCCESS. `global: NO_APTO`. Prohibido `accept-pr`.

## A3.1 medicion (`867bf5d`)

Run [33498554132](https://github.com/racso80es/SddIA/actions/runs/33498554132) job [99827293302](https://github.com/racso80es/SddIA/actions/runs/33498554132/job/99827293302).

| Step | Baseline | A3.0 hit | A3.1 |
| :--- | ---: | ---: | ---: |
| cache restore | 62 s | 43 s | 0 s (key nueva, solo registry) |
| Build native workspace | 340 s (en verify) | 429 s | **486 s** |
| verify-compiled-capsules | 340 s | 0 s | 0 s |
| LanceDB | 361 s | 419 s | **830 s** |
| Job | 816 s (13 m 36 s) | 911 s | **1350 s (22 m 30 s)** |

sccache stats (mismo job): Compile requests **0**, Cache hits 0, Cache location `Local disk` (no GHA). `RUSTC_WRAPPER` no estaba en el YAML; `sccache-action@v0.0.9` instala el binario y no envuelve `rustc`. Palanca A3.1 inoperante. `CARGO_INCREMENTAL=0` sin wrapper alarga LanceDB (830 s vs 419 s).

IOTA simulate: cache/restore 1 s, Build 482 s (verde; umbral de step >60 s). restore@v4 corrige `lookup-only`; sin sccache el compile sigue frio.

No hay segundo SHA de "hit sccache": no hay blob que calentar. CA1/CA5 no se posponen.

## CA1 / CA5

- CA1 verify = 486 s (umbral 60 s; techo 170 s) → **NO_APTO**
- CA1 LanceDB = 830 s (techo 180 s) → **NO_APTO**
- CA5 job = 22 m 30 s (umbral 8 min) → **NO_APTO**

## CA2 / CA3 / CA6 / CA7

APTO. Gate 29 bins. 16 tests. A0 = 33477170741. Hermanos verdes en 33498554132. Evolution `530039c9-100b-413a-b3d5-ca632d83acc6`.

## CA4

Decision empirica A3.1: sccache no se invocó; quitar `target/` + incremental=0 **regresiona** el job vs baseline. Registro para laudo (revertir calor A3.1 o cablear `RUSTC_WRAPPER`+`SCCACHE_GHA_ENABLED`).
