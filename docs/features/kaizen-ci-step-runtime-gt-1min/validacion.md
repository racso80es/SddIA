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
ci_run_id: "33493563587"
ci_run_event: pull_request
ci_job_integrity: "99810443522"
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

Run de cierre (primer `pull_request` post-parche, cache-miss `native-integrity-*`): [33493563587](https://github.com/racso80es/SddIA/actions/runs/33493563587) job [99810443522](https://github.com/racso80es/SddIA/actions/runs/33493563587/job/99810443522) `headSha` `587e40a7fe141ec2cbbf0a2ea6491b4ff69afd1e`. PR https://github.com/racso80es/SddIA/pull/246.

## CA2

16 tests locales: 3 memory + 5 thought + 5 evolution + 3 ingest integracion. Gate no mutado (29 bins). CI del run anterior: conclusion SUCCESS.

## CA3

A0 SSOT en PBI v1.1.0 (run 33477170741). Tabla de cierre = este PR (steps del workflow).

### Integrity vs baseline (mismo evento `pull_request`)

| Step | Baseline 99758755444 | Cierre 99810443522 | Decision |
| :--- | ---: | ---: | :--- |
| actions/cache@v4 (restore) | 62 s | 35 s | bajo umbral |
| Build native workspace | (dentro de verify, 340 s) | 430 s | calor relocado; presupuesto CA1; suelo frio 29 bins (spec §6) |
| verify-compiled-capsules | 340 s (build+gate) | 0 s (solo I/O) | anti-maquillaje: suma con Build |
| LanceDB memory integration tests | 361 s | 419 s | frio del perfil `test`; A2 no medible hasta hit |
| Post cache (save) | 0 s | 72 s | step nuevo >60 s; diferir (save de `target/` completo, A3) |
| Job wall-clock | 816 s (13 m 36 s) | 976 s (16 m 16 s) | CA5; spec §6: no techo 50 % |

### Hermanos — steps >60 s (CA3)

| Job | Wall-clock | Steps >60 s | Decision |
| :--- | ---: | :--- | :--- |
| sddia-index-integrity | 976 s | Build 430 s, LanceDB 419 s, Post cache 72 s | objeto PBI; miss de key nueva |
| eda-iota-smoke-simulate | 531 s | Build centinelas 485 s | lookup-only + miss paralelo (integrity aun no ha save); diferir; verificar en hit |
| eda-iota-physical | 521 s | Build nativos 476 s | igual; smoke physical 0 s (secret) |
| wasi-runtime-smoke | 100 s | ninguno (cache 42 s, WASI 10 s, nativos 24 s) | sin regresion de umbral |
| eda-bus-e2e-smoke | 108 s | ninguno (cache 44 s, WASI 13 s, nativos 25 s) | sin regresion de umbral |

## CA4

Decisión confirmada con cronometro:

- Restore integrity: 35 s (era 62 s). Hit parcial legado `native-*` + key nueva miss exacto.
- Save solo integrity: Post cache 72 s (blob `target/` completo). IOTA `lookup-only` no sella.
- IOTA restore 0 s en este run (key `native-integrity-*` inexistente al arranque paralelo). El save de este job habilita el siguiente SHA.

## CA7

Sin mutar `SddIA/tools/`. Evolution `530039c9-100b-413a-b3d5-ca632d83acc6`.

## CA6

Verdes en run 33493563587: `wasi-runtime-smoke`, `eda-bus-e2e-smoke`, `eda-iota-smoke-simulate`. `eda-iota-physical` conclusion SUCCESS; secret ausente (smoke 0 s) fuera de alcance.

## CA1 / CA5

Presupuesto CA1 (anti-maquillaje): Build + verify = **430 s** (techo 50 % de 340 s = 170 s; umbral 60 s). LanceDB **419 s** (techo 180 s). Job **16 m 16 s** (CA5 < 8 min, sin fallback).

Suelo de 29 bins en frio: spec.md §6. CA1/CA5 permanecen `PENDIENTE-CI` hasta un `pull_request` con hit `native-integrity-*`. Prohibido `global: APTO` y merge hasta ese `run_id`. Prohibido polling (DA-6).
