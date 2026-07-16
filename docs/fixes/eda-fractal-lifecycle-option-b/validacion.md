---
feature_name: eda-fractal-lifecycle-option-b
created: "2026-07-16"
process: bug-fix
branch: fix/eda-fractal-lifecycle-option-b
global: APTO
pbi_archived: true
pbi_ref: docs/todos/done/[FIX] EDA fractal — lifecycle opción B.md
checks:
  CA1-domain-purge-after: pass
  CA2-stamp-and-skip: pass
  CA3-telegram-ack-first: pass
  CA4-sweeper-fractal: pass
git_changes:
  - SddIA/engine/execute-process/src/engine/route_fractal_core.rs
  - SddIA/daemons/telegram-watcher/src/main.rs
  - SddIA/daemons/event-watcher/src/main.rs
  - SddIA/sddia-daemon-runtime/src/eda_sweep.rs
  - docs/fixes/eda-fractal-lifecycle-option-b/
  - docs/todos/done/[FIX] EDA fractal — lifecycle opción B.md
---

# Validación — opción B

**APTO**

| CA | Evidencia |
|----|-----------|
| CA1 | `route-domain` lab → `purged:true`, archivo domain ausente |
| CA2 | Código: stamp + `skipped-already-delivered` en `route_fractal_core`; binario contiene string |
| CA3 | `telegram-watcher`: ACK offset antes de `invoke_gateway`; state contrato + seen |
| CA4 | `event-sweeper --once` purgó fixture domain con `delivery_state` terminal |
