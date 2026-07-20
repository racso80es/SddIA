---
feature_name: kalma2-full-cycle
created: "2026-07-20"
process: feature
branch: feat/kalma2-full-cycle
global: APTO
pbi_archived: true
pr_url: https://github.com/racso80es/SddIA/pull/122
checks:
  AC-A1_cycle_phase_initialized: APTO
  AC-A2_status_initialized: APTO
  AC-A3_ui_terminal: APTO
  AC-A4_legacy_completed: APTO
  AC-A5_unit_tests: APTO
  AC-B1_agent_runtime_hook: APTO
  AC-B2_mock_statuses: APTO
  AC-B3_simulated_compat: APTO
  AC-B4_phase_fail_envelope: APTO
  AC-B5_lab_wrapper: APTO
  B_prod_cursor_sdk: APTO
  AC-C1_pbi_body_inject: APTO
  AC-C2_objectives_pbi: APTO
git_changes:
  - SddIA/engine/execute-process/src/engine/thermodynamic.rs
  - SddIA/engine/execute-process/src/engine/agent_runtime.rs
  - SddIA/engine/execute-process/src/engine/executor.rs
  - SddIA/engine/execute-process/src/engine/residual_runner.rs
  - SddIA/engine/execute-process/src/engine/handlers/task_queue_manager.rs
  - SddIA/engine/execute-process/src/engine/workspace_init.rs
  - SddIA/interfaces/kalma2-bridge/src/main.rs
  - SddIA/scripts/tools/kalma2-agent-runtime-lab.sh
  - SddIA/scripts/tools/kalma2-agent-runtime-cursor.sh
  - SddIA/scripts/tools/kalma2-agent-runtime-cursor.py
  - interfaces/kalma2/app.js
  - interfaces/kalma2/style.css
  - .dev/.env.example
  - docs/features/kalma2-full-cycle/
  - docs/todos/done/[FEATURE] kalma2-full-cycle — runtime de agentes y semántica de cierre (527007fa).md
---

# Validación — kalma2-full-cycle

## Veredicto

**APTO** (Argos lab) para slices A+B+C incluido wrapper producción Cursor (`cli`/`sdk`/`mock`). La ejecución live depende de bóveda (`cursor-agent` o `CURSOR_API_KEY`+`cursor-sdk`).

## Checks

| ID | Criterio | Estado | Evidencia |
|----|----------|--------|-----------|
| AC-A1 | PEC lab con `simulated` → `cycle_phase=initialized` | ✅ | `derive_initialized_when_simulated` |
| AC-A2 | `/api/status` proyecta `initialized` | ✅ | `project_status_initialized_from_cycle_phase` |
| AC-A3 | UI poll terminal en `initialized`/`awaiting_agents` | ✅ | `interfaces/kalma2/app.js` |
| AC-A4 | PEC legacy sin `cycle_phase` → `completed` | ✅ | `project_status_completed_from_pec` |
| AC-A5 | Tests nativos | ✅ | derive_* · project_status · build |
| AC-B1 | Con env, fase agent → `agent-runtime` | ✅ | `agent_runtime` + wire executor |
| AC-B2 | Mock executed / awaiting | ✅ | tests CLI mock |
| AC-B3 | Sin env → `simulated` | ✅ | `not_configured_returns_simulated` |
| AC-B4 | Fase failed → envelope fail | ✅ | `executor.rs` phase_failed |
| AC-B5 | Lab wrapper handoff | ✅ | `kalma2-agent-runtime-lab.sh` |
| B-prod | Wrapper Cursor CLI/SDK | ✅ | `kalma2-agent-runtime-cursor.{sh,py}` · mock + soft awaiting; live = bóveda |
| AC-C1 | `pbi_body` inyectado | ✅ | `load_pbi_body_reads_file` |
| AC-C2 | objectives prioriza PBI | ✅ | `workspace_init.rs` |

## Seguimiento live

PBI pendiente: `docs/todos/pending/[FEATURE] kalma2-llm-live — ejecución real Cursor desde Kalma2 (f0f1b1ec).md` (H1–H9: CLI host, chat LLM, fases agent, SDK, timeout, daemons, full-cycle).

## Comandos (2026-07-20)

```bash
cd SddIA && CARGO_TARGET_DIR=target cargo test -p execute-process derive_
CARGO_TARGET_DIR=target cargo test -p execute-process agent_runtime
CARGO_TARGET_DIR=target cargo test -p execute-process load_pbi_body
CARGO_TARGET_DIR=target cargo test -p kalma2-bridge project_status
```

## Cierre documental

- PBI → `docs/todos/done/` · `pbi_archived: true`
- `finalize-process.md` en `persist_ref`
- PR: https://github.com/racso80es/SddIA/pull/122
