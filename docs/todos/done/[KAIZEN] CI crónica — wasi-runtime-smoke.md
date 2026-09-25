---
document_id: PBI-KAIZEN-CI-CHRONIC-WASI-RUNTIME-SMOKE
uuid: "30a026cf-8227-4315-a521-53ec34a8cc0a"
title: "[KAIZEN] CI crónica — wasi-runtime-smoke"
format: markdown
version: "1.1.0"
created: "2026-09-10"
updated: "2026-09-25"
status: done
refinement_status: implemented
priority: alta
process: feature
executor_vehicle: feature
type: kaizen
dispatch: false
suggested_branch: feat/kaizen-ci-chronic-wasi-runtime-smoke
persist_ref_suggested: docs/features/kaizen-ci-chronic-wasi-runtime-smoke
---

# [KAIZEN] CI crónica — wasi-runtime-smoke

Materializado por Cúmulo ante `CI_Chronic_Failure_Detected` (`materialize-ci-chronic-failure-pbi`). No es Kintsugi. No es DIA. `job_entity_map` está vacío: no hubo `Domain_Entity_Degraded`.

El nombre del job no es el subsistema que falla. En la muestra que cruzó la cuota, y en el rojo vigente de `main`, el step `WASI CI smoke` terminó en success. El rojo vive en los steps `evolution gate` embebidos en el mismo job.

## 0. Hechos del stub (verificados, no reescritos)

| Campo | Valor | Verificación |
|-------|-------|----------------|
| `job_name` | `wasi-runtime-smoke` | Ledger `.SddIA/radamanto/ci_failures.json` y API del check |
| `workflow_name` | `sddia-index-qa` | Igual |
| `failure_count` | `3` | Cardinalidad de `check_run_id` del job en el ledger. No son 3 `head_sha` distintos |
| `quota_limit` | `3` | `radamanto.thresholds.json` `ci_failures.per_job_limit` |
| `head_sha` | `8ea6be512c423cca28eacc05fb30647530751583` | SHA de la muestra que disparó la alerta (`count_at_alert: 3`, `emitted_at: 2026-09-10T17:23:36Z`) |
| `html_url` | `https://github.com/racso80es/SddIA/actions/runs/34507597695/job/102973508239` | Job de la muestra |
| `sample_check_run_id` | `102973508239` | Igual |
| `repository` | `racso80es/SddIA` | Igual |

`priority: media` del stub v1.0.0 era el default de la plantilla. Pasa a `alta` porque `main` sigue rojo en este job.

## 1. Las tres filas del ledger (causas distintas)

| # | check_run_id | run | evento | head_sha | step que falla | causa |
|---|--------------|-----|--------|----------|----------------|-------|
| 1 | `102049913900` | [34222880281](https://github.com/racso80es/SddIA/actions/runs/34222880281) | `push` `feat/nucleo-aiua-tormentosa-motor` | `be72f3da5f6bbf38c6d5bed058ff2f91fd04ffd6` | `Build WASI capsules` exit 101 | `zstd-sys` compilado a `wasm32-wasip1`: clang no encuentra `bits/libc-header-start.h`. El ciclo `nucleo-aiua-tormentosa-motor` excluyó después `thought-graph-access` en `build-wasi-capsules.sh`. Esa exclusión ya está en `main`. |
| 2 | `102973487853` | [34507591699](https://github.com/racso80es/SddIA/actions/runs/34507591699) | `push` | `8ea6be51…` | `evolution gate (delta)` exit 2 | `EVOL_MATERIAL_UNREGISTERED` en 9 paths (índices, `actions.rs`, `aiua_stimulus.rs`, `mod.rs`, `route_domain_core.rs`). Step `WASI CI smoke` en success. |
| 3 | `102973508239` | [34507597695](https://github.com/racso80es/SddIA/actions/runs/34507597695) | `pull_request` | **el mismo** `8ea6be51…` | el mismo gate, los mismos 9 paths | Gemelo push+PR del mismo commit. La cuota cuenta check runs, así que un SHA suma dos. |

Incoherencias que el stub no dice y que no hay que inventar al revés:

- La muestra no es un fallo de runtime WASI ni de wasmtime. Anotación GitHub: `Process completed with exit code 2` en el step del gate.
- `failure_count: 3` con **2** SHA únicos. El gemelo no es una tercera regresión.
- El commit `f3b45954` («Update fmt.Println message from 'Hello' to 'Goodbye'`) no toca Go ni `fmt.Println`. Edita el PBI de deuda Paciente 0. No introdujo el placeholder de hash.

## 2. Rojo vigente (objeto de este ciclo)

Run [35526123813](https://github.com/racso80es/SddIA/actions/runs/35526123813) sobre `main` / `f3b459543961dd316ee3e8e7927de88405f8b116` (2026-09-20). Jobs hermanos en success. `wasi-runtime-smoke` falla solo en `evolution gate (universe)`:

```text
EVOL_HASH_MISMATCH
SddIA/evolution/83d6eb73-0936-4acb-9f1e-5d519987f1ab.md
detail: placeholder/formato inválido
hash_integrity: sha256:pending-pr-292
```

El delta de ese run devolvió `EVOL_OK` (`L-SELF / sin material`). El placeholder entró en `6c9194f` (PR #292). El push de merge `b40d48a` quedó `cancelled` por concurrencia; el run siguiente (`f3b45954`) es el que ejecutó el universo.

## 3. Mandato

Sustituir el placeholder `sha256:pending-pr-292` del registro `83d6eb73-0936-4acb-9f1e-5d519987f1ab` mediante `sddia-qa evolution-rehash --id` (sin editar el hash a mano) hasta que `gate-evolution --all` salga 0, y dejar `wasi-runtime-smoke` en success en el PR de este ciclo.

## 4. Criterios de aceptación

| ID | Criterio |
|----|----------|
| CA1 | `hash_integrity` de `83d6eb73-0936-4acb-9f1e-5d519987f1ab.md` es `sha256:` + 64 hex. El literal `pending-pr-292` no permanece. |
| CA2 | `sddia-qa gate-evolution --json --all` con `exitCode: 0` en local antes del push. |
| CA3 | Run `pull_request` de `sddia-index-qa` sobre el head del PR: job `wasi-runtime-smoke` en success. Sin `run_id` verde, `validacion.md` queda `PENDIENTE-CI` y `global` no es `APTO`. |
| CA4 | Los otros cuatro jobs del mismo run no pasan a failure por este diff. |
| CA5 | Diff de código limitado al rehash del registro evolution y a la cascada `docs/features/` + este PBI. Sin mutar `build-wasi-capsules.sh` ni el ledger Radamanto. |

## 5. Fuera de alcance

- Reabrir la exclusión WASI de `thought-graph-access` (fila 1, ya en `main`).
- Reescribir la evolución ausente del SHA `8ea6be51` (filas 2–3, rama ya no es `HEAD`).
- Deduplicar el ledger por `head_sha` (el gemelo push+PR infla la cuota; no es el rojo de `main`).
- Sacar el evolution gate del job `wasi-runtime-smoke`.
- Revertir `f3b45954`.
- Polling DA-6: un check rojo del mismo `headSha` no se rerunea; se parchea y se empuja una vez.

## 6. Ley aplicada

- `features-documentation-pattern` v1.2.1.
- `external-ai-constraints.md` DA-2, DA-5, DA-6.
- Cierre documental en la rama del PR, después de CA3 verde.
