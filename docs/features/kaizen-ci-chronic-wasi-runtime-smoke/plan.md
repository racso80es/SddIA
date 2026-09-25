---
feature_name: kaizen-ci-chronic-wasi-runtime-smoke
created: "2026-09-25"
process: feature
phases:
  - design-commit
  - evolution-rehash
  - local-universe-gate
  - delivery-pr
  - ci-then-close
branch_name: feat/kaizen-ci-chronic-wasi-runtime-smoke
persist_ref: docs/features/kaizen-ci-chronic-wasi-runtime-smoke
pbi_ref: docs/todos/pending/[KAIZEN] CI crónica — wasi-runtime-smoke.md
document_id: PBI-KAIZEN-CI-CHRONIC-WASI-RUNTIME-SMOKE
pbi_uuid: "30a026cf-8227-4315-a521-53ec34a8cc0a"
pbi_version: "1.1.0"
execution_id: "915ff612-6aad-4d82-b92a-6d455d2cfe3d"
---

# Plan — kaizen-ci-chronic-wasi-runtime-smoke

Corte diseño: PBI v1.1.0 + clarify + objectives + spec + plan. Commit de planificación antes del rehash.

## L0 — Diseño

Este commit. Sin mutar `SddIA/evolution/`.

## L1 — Rehash

`sddia-qa evolution-rehash --id 83d6eb73-0936-4acb-9f1e-5d519987f1ab --json`. Exigir `success: true` y `hash_integrity` de 64 hex. Si `idempotent: true` con el placeholder aún presente, parar: el binario no aplicó el contrato.

## L2 — Gates locales

`gate-evolution --json --all` y `--range --require-synced-base`. Ambos `exitCode: 0` antes del push. Si el rango toca solo evolution + docs, el delta esperado es `L-SELF / sin material`.

## L3 — Entrega

`implementation.md` + `execution.md`. `validacion.md` con CA3 en `PENDIENTE-CI` y `global` distinto de `APTO` hasta el run verde. `delivery-close-cycle` abre el PR. Prohibido `pbi_archived: true` mientras el PBI siga en `pending/`.

## L4 — CI y cierre

Un vistazo al run del PR. Verde en `wasi-runtime-smoke` y hermanos no rotos → entonces `global: APTO`, mover el PBI a `docs/todos/done/`, push de cierre, y `accept-pr` solo con ese head verde. Un failure del mismo `headSha`: parche local, un push, sin `gh run rerun`.
