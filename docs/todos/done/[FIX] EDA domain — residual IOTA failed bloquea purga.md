---
document_id: PBI-EDA-DOMAIN-RESIDUAL-FAILED-IOTA
title: "[FIX] EDA domain — residual: archivos persisten por IOTA failed"
format: markdown
version: "1.1.0"
created: "2026-07-16"
status: done
priority: alta
process: bug-fix
persist_ref: docs/fixes/eda-fractal-dlq-c2/
branch: fix/eda-fractal-dlq-c2
validacion_ref: docs/fixes/eda-fractal-dlq-c2/validacion.md
closed: "2026-07-16"
related:
  - docs/fixes/eda-fractal-lifecycle-option-b/continuation.md
  - docs/todos/done/[FIX] EDA fractal — lifecycle opción B.md
  - https://github.com/racso80es/SddIA/pull/113
  - SddIA/evolution/3b42e74f-9b9e-4efa-84ad-c8431ba290b2.md
---

# PBI-EDA-DOMAIN-RESIDUAL-FAILED-IOTA

## Qué

Tras merge opción B (#113), `.events/domain/` retenía JSON con `delivery_state` conteniendo `*.iota-immutable-publisher: failed`. Purga opción B no unlea (correcto bajo all-ok).

## Laudo elegido

**C2** — política terminal-with-failure → DLQ `eda_fractal.dead_letter` (`./.events/dead-letter`).

## Criterio de cierre

- [ ] C1: IOTA remediado → re-route → domain purgable / vacío de terminales
- [x] C2: nueva política de archivo/purga con `failed` documentada + implementada
- [ ] C3: drenaje one-shot del backlog histórico + norma anti-acúmulo

## Persist

`docs/fixes/eda-fractal-dlq-c2/` — rama `fix/eda-fractal-dlq-c2`.

## Handoff origen

`docs/fixes/eda-fractal-lifecycle-option-b/continuation.md`
