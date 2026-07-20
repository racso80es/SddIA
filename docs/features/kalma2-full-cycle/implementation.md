---
feature_name: kalma2-full-cycle
created: "2026-07-20"
process: feature
items:
  - thermodynamic.rs cycle_phase
  - kalma2-bridge project_status
  - interfaces/kalma2 app.js + style.css
  - agent_runtime.rs SDDIA_AGENT_RUNTIME_COMMAND
  - executor.rs + residual_runner.rs wire
---

# Implementation — kalma2-full-cycle

## Touchpoints Slice A

| # | Path | Cambio |
|---|------|--------|
| 1 | `thermodynamic.rs` | `derive_cycle_phase` + PEC |
| 2 | `kalma2-bridge` | `project_status` + orch.cycle_phase |
| 3–4 | `app.js` / `style.css` | Terminales UI |

## Touchpoints Slice B

| # | Path | Cambio |
|---|------|--------|
| 5 | `engine/agent_runtime.rs` | CLI `SDDIA_AGENT_RUNTIME_COMMAND` · op `AGENT_PHASE` |
| 6 | `executor.rs` | Fases solo-agent → runtime si configurado; fallo de fase → envelope fail |
| 7 | `residual_runner.rs` | Misma aduana agent-runtime |
| 8 | `.dev/.env.example` | Documenta env |

## Pendiente

| Slice | Estado |
|-------|--------|
| B wrapper Cursor/Agent SDK real | Fuera: contrato + hook nativo listos; comando instancia |
| C `pbi_body` | Diferido |
