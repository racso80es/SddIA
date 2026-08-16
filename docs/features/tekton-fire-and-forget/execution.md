---
feature_name: tekton-fire-and-forget
created: "2026-08-16"
process: feature
items_applied:
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
status: executed
pattern_id: b6a9ed14-3a0d-4f5b-8444-d1b867335daf
---

# Execution — tekton-fire-and-forget

## Unit

| Suite | Resultado |
|-------|-----------|
| `cargo test -p execute-process --lib cli_detach` (5) | OK — allowlist, foreground, PEC path, spawn &lt;500 ms vs `sleep 2` |

## Smoke AC

| ID | Caso | Resultado |
|----|------|-----------|
| **AC2** | `./sddia-run.sh --process event-bus-audit --detach --inputs '{}'` | **APTO** · 28.3 ms · `detached:true` · `status:accepted` · `exitCode:0` · PID 112611 |
| **AC3** | PEC `.events/orchestration/62586142-939b-4d18-801f-1ed80fa0ae25.json` | **APTO** · `Process_Execution_Completed` · `detach:true` · `cycle_phase:awaiting_agents` · no `.SddIA/events/` |
| **AC4** | log hijo `.SddIA/workspaces/event-bus-audit/ccb9a335-…/detached.log` | **APTO** · size 1800 |
| **AC5** | mismo process **sin** `--detach` (foreground) | **APTO** · 190 ms join · `detached` ausente |
| **AC1** | DA-5 + tekton §5 + `.cursorrules` / mdc | **APTO** (diff) |

PPR real (GitHub/agentes) no exigido en CI; allowlist cubre `pull-request-review`. Smoke usó `--detach` sobre process corto para medir acuse.

## Comandos

```bash
cd SddIA && cargo test -p execute-process --lib cli_detach -- --test-threads=1
SDDIA_AGENT_RUNTIME_COMMAND= ./sddia-run.sh --process event-bus-audit --detach --inputs '{}'
```

## Fricción EM

`entity-manager` update `tekton` no ejecutado: `run_agent_forge` regenera UUID. Tekton.md v1.2.0 quirúrgico.
