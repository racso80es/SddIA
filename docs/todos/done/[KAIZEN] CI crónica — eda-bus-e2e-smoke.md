---
document_id: PBI-KAIZEN-CI-CHRONIC-EDA-BUS-E2E-SMOKE
uuid: f2bbed15-2cca-49ae-a439-aff1a935e43b
title: '[KAIZEN] CI crónica — eda-bus-e2e-smoke'
format: markdown
version: 1.1.0
created: 2026-10-21
updated: 2026-10-21
status: done
refinement_status: implemented
priority: alta
process: feature
executor_vehicle: feature
type: kaizen
suggested_branch: feat/kaizen-ci-chronic-eda-bus-e2e-smoke
persist_ref_suggested: docs/features/kaizen-ci-chronic-eda-bus-e2e-smoke
refined: 2026-10-21
---

# [KAIZEN] CI crónica — eda-bus-e2e-smoke

Materializado por Cúmulo ante `CI_Chronic_Failure_Detected` (`materialize-ci-chronic-failure-pbi`). No es Kintsugi. No es DIA. `job_entity_map` está vacío: no hubo `Domain_Entity_Degraded`.

El nombre del job no es el subsistema que falla. La muestra que cruzó la cuota, y el rojo vigente de `main`, mueren en el step `Build WASI capsules` antes de `EDA E2E lab (simulate)`.

## 0. Hechos del stub (verificados, no reescritos)

| Campo | Valor | Verificación |
|-------|-------|----------------|
| `job_name` | `eda-bus-e2e-smoke` | Ledger `.SddIA/radamanto/ci_failures.json` y API del check |
| `workflow_name` | `sddia-index-qa` | Igual |
| `failure_count` | `3` | Cardinalidad de `check_run_id` del job en el ledger. Son 3 `head_sha` distintos |
| `quota_limit` | `3` | `radamanto.thresholds.json` `ci_failures.per_job_limit` |
| `head_sha` | `218831912cfdafb376ddaee53857df18e9792b42` | SHA de la muestra que disparó la alerta (`count_at_alert: 3`, `emitted_at: 2026-10-21T07:58:37Z`) |
| `html_url` | `https://github.com/racso80es/SddIA/actions/runs/39565647758/job/119107147785` | Job de la muestra. PR [#358](https://github.com/racso80es/SddIA/pull/358), ya mergeado |
| `sample_check_run_id` | `119107147785` | Igual |
| `repository` | `racso80es/SddIA` | Igual |

`priority: media` del stub v1.0.0 era el default de la plantilla. Pasa a `alta` porque `main` sigue rojo en este job.

## 1. Las tres filas del ledger (dos causas)

| # | check_run_id | run | evento | head_sha | step que falla | causa |
|---|--------------|-----|--------|----------|----------------|-------|
| 1 | `111402814783` | [37190795064](https://github.com/racso80es/SddIA/actions/runs/37190795064) | `pull_request` `feat/linear-hu-b-01-outbound` | `e1066a8e9d5885e991c34d01bdb42a0da6f1c13f` | `Linear direct-cycle E2E lab (mock)` | Anotación GitHub: `Process completed with exit code 1`. El cuerpo del log no se recuperó en este refinamiento. |
| 2 | `111409283409` | [37193112231](https://github.com/racso80es/SddIA/actions/runs/37193112231) | `pull_request` de la misma rama | `24d15a8b67c9f8060e298e0d404dbdafd71edf3f` | el mismo step | La misma anotación, exit code 1. SHA distinto: no es gemelo push+PR. |
| 3 | `119107147785` | [39565647758](https://github.com/racso80es/SddIA/actions/runs/39565647758) | `pull_request` #358 `feat/eda-redb-06-semantic-layer` | `218831912cfdafb376ddaee53857df18e9792b42` | `Build WASI capsules (skills/tools; exclude native daemons)` exit 101 | `error[E0554]` en `redb-2.6.4/src/lib.rs:26`: `#![cfg_attr(target_os = "wasi", feature(wasi_ext))]`. `could not compile redb (lib)`. |

Incoherencias que el stub no dice:

- `failure_count: 3` mezcla la pareja Linear del 2026-10-04 con el fallo WASI del 2026-10-21. La cuota cuenta check runs, no una sola regresión.
- El rojo de la muestra no llega al lab EDA ni al lab Linear. El step de cápsulas WASI aborta antes.
- El ledger leído en este refinamiento no contiene los check runs de los `push` a `main` de los merges #356, #357, #358 y #359. Esos jobs también están en failure.

## 2. Rojo vigente (objeto de este ciclo)

Último `eda-bus-e2e-smoke` en success sobre `main`: run [39554780138](https://github.com/racso80es/SddIA/actions/runs/39554780138), merge #355, `cb8730a119cb110057c656fd089d7828afbcbfcc`. Los seis jobs del workflow en success.

A partir del merge #356 el step `Build WASI capsules` falla en `eda-bus-e2e-smoke` y en `wasi-runtime-smoke` con el mismo `E0554` de `redb` 2.6.4. Log verificado en el push de #356, run [39562132513](https://github.com/racso80es/SddIA/actions/runs/39562132513), `7113d6f805caf424bbc21cf40e968d0d61c1f078`. Misma conclusión de job en los pushes de #357, #358 y #359.

| Run | Merge | head_sha | Job `eda-bus-e2e-smoke` |
|-----|-------|----------|-------------------------|
| [39562132513](https://github.com/racso80es/SddIA/actions/runs/39562132513) | #356 | `7113d6f805caf424bbc21cf40e968d0d61c1f078` | failure, step WASI |
| [39564464904](https://github.com/racso80es/SddIA/actions/runs/39564464904) | #357 | `61f17bf04d501f8c987c565a76dcf99baaae823e` | failure, step WASI |
| [39565980226](https://github.com/racso80es/SddIA/actions/runs/39565980226) | #358 | `4f0ed2155788a6913f5bdff22aa64e277bfb4495` | failure, step WASI |
| [39568902126](https://github.com/racso80es/SddIA/actions/runs/39568902126) | #359 | `37d499f5d40084580586d842bafaa8977df74661` | failure, step WASI |

Causa local, alineada con ese log:

- `SddIA/scripts/qa/build-wasi-capsules.sh` compila con `cargo build --target wasm32-wasip1` todo `tools/*/src/main.rs` que no esté en `EXCLUDE`. El workflow usa `dtolnay/rust-toolchain@stable`.
- `event-bus-audit` tiene `src/main.rs`, no está en `EXCLUDE` y depende de `sddia-event-store`, que fija `redb =2.6.4`.
- En `wasm32-wasip1`, `target_os = "wasi"`. `redb` 2.6.4 activa `feature(wasi_ext)`, y el canal stable lo rechaza.
- El commit `5e78d781d2ae2dbbd74cf9b0936fef43490dbee6` introduce ese CLI. Es ancestro del merge #356 y no lo es del merge #355.
- De los paquetes que `build-wasi-capsules.sh` descubre, solo `event-bus-audit` depende de `sddia-event-store`. `execute-process`, `event-watcher` y `kalma2-bridge` ya están en `EXCLUDE`. `sddia-daemon-runtime` no vive bajo `skills/`, `tools/` ni `interfaces/`.

## 3. Mandato

Añadir `event-bus-audit` a `EXCLUDE` de `SddIA/scripts/qa/build-wasi-capsules.sh`, con el mismo criterio que `thought-graph-access`: órgano nativo de host, fuera de `wasm32-wasip1`. El script debe dejar de compilar `redb` en ese target. El canal del job sigue siendo stable. El pin `redb =2.6.4` no se mueve.

`SddIA/scripts/` no está en los directorios de genoma de DA-2. El cambio del script es edición de ciclo, no `entity-manager`.

## 4. Criterios de aceptación

| ID | Criterio |
|----|----------|
| CA1 | `event-bus-audit` figura en `EXCLUDE` de `build-wasi-capsules.sh`. El comando de ese script no incluye `-p event-bus-audit`. |
| CA2 | `SddIA/scripts/qa/build-wasi-capsules.sh` termina con exit 0 en local antes del push. La salida no contiene `could not compile redb`. |
| CA3 | Run `pull_request` de `sddia-index-qa` sobre el head del PR: job `eda-bus-e2e-smoke` en success. Sin `run_id` verde, `validacion.md` queda `PENDIENTE-CI` y `global` no es `APTO`. |
| CA4 | En ese mismo run, `wasi-runtime-smoke` queda en success. Los otros jobs del workflow no pasan a failure por este diff. |
| CA5 | Diff limitado al `EXCLUDE` del script, al registro evolution `4a6499e4-1a65-4aed-bd9f-e1be6b51b196` que lo correlaciona, a la cascada `docs/features/kaizen-ci-chronic-eda-bus-e2e-smoke/` y a este PBI. Sin cambiar el pin de `redb`, el toolchain del workflow, el fuente de `event-bus-audit` ni el ledger Radamanto. |

## 5. Fuera de alcance

- Reabrir el lab Linear de las filas 1 y 2. En el run verde de #355 ese step ya pasó.
- Deduplicar el ledger por `head_sha` o ingerir los push de #356–#359 que el ledger aún no lista.
- Pasar el job a nightly para aceptar `feature(wasi_ext)`.
- Subir o parchear `redb`.
- Sacar el step WASI de `eda-bus-e2e-smoke`.
- Abrir otro PBI crónico para `wasi-runtime-smoke`: el rojo actual de ese job es el mismo `E0554` y queda cubierto por CA4.
- Polling DA-6: un check rojo del mismo `headSha` no se rerunea; se parchea y se empuja una vez.

## 6. Ley aplicada

- `features-documentation-pattern` v1.3.0.
- Proceso `feature` v1.3.2. Cierre documental en la rama del PR, después de CA3 verde.
- `external-ai-constraints.md` v1.6.2: DA-5 y DA-6. DA-2 no cubre `SddIA/scripts/`.

<!-- refined:mayeuta -->
