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
ci_run_id: "33495498463"
ci_run_event: pull_request
ci_job_integrity: "99816608487"
ci_run_id_miss: "33493563587"
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

PR https://github.com/racso80es/SddIA/pull/246. Checks funcionales SUCCESS. `global: NO_APTO`: CA1 y CA5 no cumplen umbral (L-NO-SCCACHE / spec §6). Prohibido `accept-pr` con este veredicto.

## Runs

| Rol | Run | Evento | Job integrity | headSha |
| :--- | :--- | :--- | :--- | :--- |
| Miss key nueva | [33493563587](https://github.com/racso80es/SddIA/actions/runs/33493563587) | pull_request | [99810443522](https://github.com/racso80es/SddIA/actions/runs/33493563587/job/99810443522) | `587e40a` |
| Hit key (medicion CA1/CA5) | [33495498463](https://github.com/racso80es/SddIA/actions/runs/33495498463) | pull_request | [99816608487](https://github.com/racso80es/SddIA/actions/runs/33495498463/job/99816608487) | `754c575` |
| Hit key (push, colateral) | [33495496058](https://github.com/racso80es/SddIA/actions/runs/33495496058) | push | [99816601633](https://github.com/racso80es/SddIA/actions/runs/33495496058/job/99816601633) | `754c575` |

Evidencia de hit en `754c575`: restore integrity 43 s (blob); Post cache 1 s (key ya sellada, first-write-wins). No es miss de key.

## CA2

16 tests locales. Gate 29 bins. CI SUCCESS en ambos runs.

## CA3

A0 = run 33477170741. Tabla de cierre = run 33495498463 (hit).

### Integrity vs baseline (`pull_request`)

| Step | Baseline 99758755444 | Miss 99810443522 | Hit 99816608487 | Decision |
| :--- | ---: | ---: | ---: | :--- |
| cache restore | 62 s | 35 s | 43 s | bajo/umbral |
| Build native workspace | (en verify, 340 s) | 430 s | **429 s** | calor no cae con `target/` cacheado |
| verify-compiled-capsules | 340 s | 0 s | 0 s | I/O; CA1 = suma con Build |
| LanceDB | 361 s | 419 s | **419 s** | perfil `test` frio igual |
| Post cache | 0 s | 72 s | 1 s | save ya hecho |
| Job | 816 s (13 m 36 s) | 976 s | **911 s (15 m 11 s)** | CA5 |

Hermanos hit (33495498463): IOTA simulate 525 s (build 505 s, cache 1 s); IOTA physical 509 s (build 494 s, cache 0 s); wasi 91 s; e2e 91 s. IOTA: step >60 s permanente. Causa: `lookup-only: true` **no descarga** el blob (solo existencia). L-CACHE-IOTA-RO queda incumplido en el primitivo; palanca correcta = `actions/cache/restore@v4` (restore sin save). Fuera de este sello; no se itera sin laudo.

## CA4

Save solo integrity (`native-integrity-*`). Numeros: miss save 72 s; hit Post 1 s. Restore integrity en hit 43 s **no** calienta `cargo build --workspace` (429 s ≈ miss 430 s). IOTA `lookup-only` no restaura archivos (1 s / 0 s).

## CA7

Sin mutar `SddIA/tools/`. Evolution `530039c9-100b-413a-b3d5-ca632d83acc6`.

## CA6

Verdes en 33495498463: wasi, e2e, iota-simulate. iota-physical SUCCESS; secret ausente (smoke 0 s) fuera de alcance.

## CA1 / CA5

Hit `754c575` / job 99816608487:

- CA1 verify = 429 + 0 = **429 s** (umbral 60 s; techo 50 % = 170 s) → **NO_APTO**
- CA1 LanceDB = **419 s** (techo 180 s) → **NO_APTO**
- CA5 job = **15 m 11 s** (umbral 8 min; sin fallback) → **NO_APTO**

Tests en verde no equivalen a estos umbrales. `global: NO_APTO`.
