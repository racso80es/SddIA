---
feature_name: kalma2-full-cycle
created: "2026-07-20"
process: feature
items:
  - thermodynamic.rs cycle_phase
  - kalma2-bridge project_status
  - interfaces/kalma2 app.js + style.css
  - agent_runtime.rs SDDIA_AGENT_RUNTIME_COMMAND
  - kalma2-agent-runtime-lab.sh
  - task_queue_manager pbi_body
  - workspace_init pbi_body / pbi_ref
---

# Implementation — kalma2-full-cycle

## Slice A

| Path | Cambio |
|------|--------|
| `thermodynamic.rs` | `cycle_phase` en PEC |
| `kalma2-bridge` + UI | `initialized` / `awaiting_agents` |

## Slice B

| Path | Cambio |
|------|--------|
| `agent_runtime.rs` | Hook CLI `AGENT_PHASE` |
| `executor.rs` / `residual_runner.rs` | Wire + fail envelope |
| `scripts/tools/kalma2-agent-runtime-lab.sh` | Wrapper lab (handoff + awaiting_agents) |

## Slice C

| Path | Cambio |
|------|--------|
| `handlers/task_queue_manager.rs` | `load_pbi_body` + semilla combinada + `pbi_body` |
| `workspace_init.rs` | Misión desde `pbi_body`; frontmatter `pbi_ref` |

## Pendiente

| Ítem | Nota |
|------|------|
| Wrapper Cursor/SDK producción | Config bóveda; lab script no sustituye agentes V5 |
| Evento handoff B2 | Opcional |
| Argos `validacion.md` APTO | Cierre documental feature |
