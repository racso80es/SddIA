---
feature_name: dcc-pr-title-metachar-451dc8707819
created: "2026-09-01"
process: bug-fix
phases:
  - sanitize-title-preflight
  - classify-error-codes
  - stamp-friction-suppress-fracture
  - mayeuta-metachar-bucket
  - verify-unit
  - evolution-and-docs
branch_name: fix/dcc-pr-title-metachar-451dc8707819
persist_ref: docs/fixes/dcc-pr-title-metachar-451dc8707819
---

# Plan — fractura `451dc8707819`

Corte Diseño: **spec + plan + commit**. Ejecución (fases 1–6) en el mismo ciclo hasta PR.

## Fase 1 — Saneo + preflight (F1, CA1)

`phase_capsules.rs`: `sanitize_shell_argv_token`. En `capsule_delivery_gh_pr`, sanear título; no sanear head/base; preflight argv; título vacío/inseguro → `blocked` `PR_TITLE_METACHAR`.

Test: specimen `>` → token seguro; contiene `gt`.

## Fase 2 — Clasificador (F2, CA2/CA3)

Extender `classify_delivery_error` (índice 3 vs 9 vs resto). `delivery_phase_failed` añade `friction_id`. Conservar test `arguments[9]` → `PR_BODY_METACHAR`. Nuevo: `arguments[3]` → `PR_TITLE_METACHAR`.

## Fase 3 — Supresión F4 (CA6)

`delivery_close.rs`: predicado `dcc_title_metachar_block_suppresses_fracture` (Apertura + `failed`/`blocked` + traza `PR_TITLE_METACHAR` o friction `F-DCC-PR-TITLE-METACHAR`). `continue` en `emit_dcc_phase_fractures`. Tests: suppress title; emit `pr_url` opaco (regresión CA-4 de `d0cfd5b66ff1`).

## Fase 4 — Mayeuta (F3, CA4/CA5)

`enrich_fracture_pbi_kaizen.rs`: cubo metachar **antes** del catch-all `timeout|failed`. Tests: specimen hash-trace; `analyze_fracture_kaizen_recursion_verdict` intacto.

## Fase 5 — Tests (CA7)

```text
cargo test -p execute-process -- sanitize_shell classify_delivery PR_TITLE analyze_fracture_kaizen dcc_fracture
```

## Fase 6 — Evolution + cierre documental (CA8)

`sddia-qa evolution-register` (modificacion, motor `execute-process`). `implementation.md` / `execution.md` / `validacion.md`. PBI → `docs/todos/done/`. `delivery-close-cycle` con `pr_title` **sin** `>` (no dogfood el colapso con binario intermedio).
