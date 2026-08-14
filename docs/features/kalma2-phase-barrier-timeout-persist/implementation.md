---
feature_name: kalma2-phase-barrier-timeout-persist
created: "2026-08-14"
process: refactorization
items:
  - executor-phase-barrier
  - workspace-init-refactor-prefix
  - tqm-persist-ref
  - runtime-timeout-terminal
branch_name: refactor/kalma2-phase-barrier-timeout-persist
persist_ref: docs/features/kalma2-phase-barrier-timeout-persist
document_id: 1de0bdd1-6144-4e45-8efa-92db0f399147
agents: tekton
execution_id: d630a6cf-1767-4751-a2b9-b1f4210a01fb
---

# Implementation — kalma2-phase-barrier-timeout-persist

## Touchpoints

| Path | Propuesta |
|------|-----------|
| `SddIA/engine/execute-process/src/engine/executor.rs` | Barrera: fase agente `failed`/`awaiting_agents`/`blocked`/`awaiting` → skip Verificación/cierre y agentes posteriores. Skip lab cierre también en `refactorization`. |
| `SddIA/engine/execute-process/src/engine/workspace_init.rs` | Conserva `feat/`/`fix/`/`refactor/`. Default `refactorization` → `refactor/{slug}`. |
| `SddIA/engine/execute-process/src/engine/handlers/task_queue_manager.rs` | Inyecta `persist_ref` (FM PBI o Cúmulo). Default rama `refactor/{slug}`. |
| `SddIA/engine/execute-process/src/engine/agent_runtime.rs` | `persist_ref` desde inputs o `state.workspace`. |
| `SddIA/engine/execute-process/src/engine/eda_bus_topology.rs` | `infer_persist_ref_from_branch`: `refactor/` → `featurePath`. |
| `SddIA/scripts/tools/kalma2-agent-runtime-cursor.py` | Timeout ≠ soft. Override `SDDIA_AGENT_RUNTIME_TIMEOUT_SECS_EJECUCION`. Prompt: persist desde `inputs`. |
| `SddIA/scripts/tools/test_kalma2_runtime_timeout.py` | Unit timeout + persist fallback. |

## No tocado (genoma)

`git-operations.md` — ejemplo `feat/`/`fix/` no es lista cerrada. Alineación de texto vía `entity-manager` (fuera de este bisturí). Process `refactorization.md` sin cambio (L-GENOME).
