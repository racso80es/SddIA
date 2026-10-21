---
feature_name: kaizen-ci-chronic-eda-bus-e2e-smoke
created: "2026-10-21"
process: feature
branch_name: feat/kaizen-ci-chronic-eda-bus-e2e-smoke
persist_ref: docs/features/kaizen-ci-chronic-eda-bus-e2e-smoke
pbi_ref: docs/todos/pending/[KAIZEN] CI crónica — eda-bus-e2e-smoke.md
document_id: PBI-KAIZEN-CI-CHRONIC-EDA-BUS-E2E-SMOKE
uuid: "f2bbed15-2cca-49ae-a439-aff1a935e43b"
phase: mayeuta-stabilization
execution_id: "0db56ca9-0fe4-4e81-96cd-88f5c52fcdd4"
---

# Objetivos — kaizen-ci-chronic-eda-bus-e2e-smoke

## Objetivo

Sacar `event-bus-audit` del build `wasm32-wasip1` de `build-wasi-capsules.sh` para que `eda-bus-e2e-smoke` deje de morir en `E0554` de `redb` 2.6.4 (`feature(wasi_ext)` sobre canal stable).

## Alcance

1. Añadir `event-bus-audit` a `EXCLUDE`. El comando del script no lleva `-p event-bus-audit` y no compila `redb`.
2. `SddIA/scripts/qa/build-wasi-capsules.sh` con exit 0 en local antes del push.
3. Cascada en `docs/features/kaizen-ci-chronic-eda-bus-e2e-smoke/`. PBI v1.1.0.
4. CA3 y CA4: run `pull_request` de `sddia-index-qa`. Cierre documental después de ese verde. Hasta entonces `global` no es `APTO`.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| CA1 | `event-bus-audit` en `EXCLUDE`. Sin `-p event-bus-audit`. |
| CA2 | El script termina en 0 y la salida no contiene `could not compile redb`. |
| CA3 | Job `eda-bus-e2e-smoke` success en el PR. Sin run verde, no hay `global: APTO`. |
| CA4 | `wasi-runtime-smoke` success en el mismo run. El resto no pasa a failure por este diff. |
| CA5 | Diff limitado al `EXCLUDE`, a esta cascada y al PBI. Pin `redb`, toolchain, fuente de `event-bus-audit` y ledger intactos. |

## Fuera de alcance

Lab Linear del 2026-10-04. Dedup del ledger. Nightly. Bump de `redb`. Quitar el step WASI. Segundo PBI para `wasi-runtime-smoke`. Polling DA-6.

## Ley aplicada

- `features-documentation-pattern` v1.3.0.
- Proceso `feature` v1.3.2.
- `external-ai-constraints.md` v1.6.2: DA-5, DA-6. `SddIA/scripts/` queda fuera de DA-2.
