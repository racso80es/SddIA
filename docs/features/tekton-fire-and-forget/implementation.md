---
feature_name: tekton-fire-and-forget
created: "2026-08-16"
process: feature
items:
  - T1-cli-detach
  - T2-da5-norm
  - T3-tekton-contract
  - T4-touchpoints
  - T5-smokes
branch_name: feat/tekton-fire-and-forget
persist_ref: docs/features/tekton-fire-and-forget
document_id: PBI-TEKTON-FIRE-AND-FORGET
uuid: 3ad2901a-aaf4-4631-b5df-11386b3ea997
agents: tekton
pattern_id: b6a9ed14-3a0d-4f5b-8444-d1b867335daf
---

# Implementation — tekton-fire-and-forget

## Touchpoints

| ID | Path | Acción | Patrón |
|----|------|--------|--------|
| T1 | `SddIA/engine/execute-process/src/engine/cli_detach.rs` | Crear módulo allowlist + PEC ack + spawn huérfano | `b6a9ed14-3a0d-4f5b-8444-d1b867335daf` |
| T1 | `SddIA/engine/execute-process/src/engine/mod.rs` | Hook `maybe_detach` post domain-authority | idem |
| T1 | `SddIA/engine/execute-process/src/main.rs` | Flags `--detach` / `--foreground` | — |
| T1 | `SddIA/engine/execute-process/src/engine/invoke_orchestrator.rs` | `SDDIA_CLI_FOREGROUND=1` en subprocesos | — |
| T1 | `SddIA/engine/execute-process/src/engine/workspace.rs` | Honrar `SDDIA_DETACHED_EXECUTION_ID` | — |
| T1 | `SddIA/daemons/event-watcher/src/main.rs` | Foreground en invocaciones CLI | L6 |
| T2 | `SddIA/norms/external-ai-constraints.md` | DA-5 v1.5.0 | Core norms; sin `norm-creator` |
| T3 | `SddIA/agents/tekton.md` + `index.md` | §5 Mandato de Latencia; v1.2.0 | EM update abortado: forge regenera UUID |
| T4 | `.cursorrules` + `.cursor/rules/tekton-fire-and-forget.mdc` | Difusión DA-5 | touchpoint |
| T5 | tests `cli_detach` + smoke `--detach` | AC2–AC5 | — |

## Notas de fricción

- `agent-creator`/`run_agent_forge` en update mintea UUID nuevo y reescribe plantilla. **Prohibido** EM update de `tekton` hasta que el forge sea patch-safe. Mutación quirúrgica + evolution.
- Allowlist default **solo** `pull-request-review`. `radamanto-batch` excluido (join del watcher).
