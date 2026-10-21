---
feature_name: kaizen-ci-chronic-eda-bus-e2e-smoke
created: "2026-10-21"
process: feature
phases:
  - design
  - exclude
  - local-wasi-build
  - delivery-pr
  - ci-then-close
branch_name: feat/kaizen-ci-chronic-eda-bus-e2e-smoke
persist_ref: docs/features/kaizen-ci-chronic-eda-bus-e2e-smoke
pbi_ref: docs/todos/pending/[KAIZEN] CI crónica — eda-bus-e2e-smoke.md
document_id: PBI-KAIZEN-CI-CHRONIC-EDA-BUS-E2E-SMOKE
pbi_uuid: "f2bbed15-2cca-49ae-a439-aff1a935e43b"
pbi_version: "1.1.0"
execution_id: "0db56ca9-0fe4-4e81-96cd-88f5c52fcdd4"
---

# Plan — kaizen-ci-chronic-eda-bus-e2e-smoke

## L0 — Diseño

clarify, objectives, spec y plan. Sin mutar el pin de `redb`.

## L1 — Exclusión

Una entrada `event-bus-audit` en `EXCLUDE` de `build-wasi-capsules.sh`.

## L2 — Gate local

Ejecutar el script. Exigir exit 0 y ausencia de `could not compile redb`.

## L3 — Entrega

`implementation.md` + `execution.md`. `validacion.md` con CA3 y CA4 en `PENDIENTE-CI` y `global` distinto de `APTO`. `delivery-close-cycle` abre el PR. `pbi_archived` permanece false mientras el PBI siga en `pending/`.

## L4 — CI y cierre

Un run `pull_request` verde de `eda-bus-e2e-smoke` y `wasi-runtime-smoke` habilita `global: APTO` y el archivo del PBI en la misma rama. Un check rojo del mismo `headSha` no se rerunea.
