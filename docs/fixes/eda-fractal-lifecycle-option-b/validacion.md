---
feature_name: eda-fractal-lifecycle-option-b
created: "2026-07-16"
process: bug-fix
branch: fix/eda-fractal-lifecycle-option-b
global: APTO
pbi_archived: true
pbi_ref: docs/todos/done/[FIX] EDA fractal — lifecycle opción B.md
residual_ref: docs/fixes/eda-fractal-lifecycle-option-b/continuation.md
residual_pbi: docs/todos/done/[FIX] EDA domain — residual IOTA failed bloquea purga.md
checks:
  CA1-domain-purge-after: pass
  CA2-stamp-and-skip: pass
  CA3-telegram-ack-first: pass
  CA4-sweeper-fractal: pass
  CA5-empirical-backlog-empty: fail
git_changes:
  - SddIA/engine/execute-process/src/engine/route_fractal_core.rs
  - SddIA/daemons/telegram-watcher/src/main.rs
  - SddIA/daemons/event-watcher/src/main.rs
  - SddIA/sddia-daemon-runtime/src/eda_sweep.rs
  - docs/fixes/eda-fractal-lifecycle-option-b/
  - docs/todos/done/[FIX] EDA fractal — lifecycle opción B.md
---

# Validación — opción B

**APTO (lab happy-path).** Residual empírico: backlog domain **no vacío** — ver `continuation.md`.

| CA | Evidencia |
|----|-----------|
| CA1 | `route-domain` lab + `SDDIA_LAB_SIMULATE_IOTA=1` → `purged:true` |
| CA2 | stamp + `skipped-already-delivered` en binario debug |
| CA3 | ACK-first Telegram en código mergeado |
| CA4 | sweeper fractal unlink fixture terminal |
| CA5 | **FAIL empírico:** 145 domain con `iota-immutable-publisher: failed` → política B no purga |

## Residual

Los archivos permanecen **por diseño de opción B** ante stamps `failed` (IOTA). Continuación: PBI `PBI-EDA-DOMAIN-RESIDUAL-FAILED-IOTA`.
