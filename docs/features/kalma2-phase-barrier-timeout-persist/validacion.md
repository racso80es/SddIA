---
feature_name: kalma2-phase-barrier-timeout-persist
created: "2026-08-14"
process: refactorization
branch: refactor/kalma2-phase-barrier-timeout-persist
persist_ref: docs/features/kalma2-phase-barrier-timeout-persist
pbi_ref: docs/todos/done/[REFACTOR] Kalma2 — serialización de fases, timeout y rama refactor (KALMA2-AUD-4b9de6).md
document_id: 1de0bdd1-6144-4e45-8efa-92db0f399147
source_correlation_id: 4b9de6b3-c400-49c8-86f2-55f08ec64ce4
execution_id: d630a6cf-1767-4751-a2b9-b1f4210a01fb
pr_url: https://github.com/racso80es/SddIA/pull/174
pr_presented_event_id: 2b466b03-9125-414e-9893-8ea6d8ef7f93
snapshot_commit: 2f7fc11238141296cc050e2769e80f764ebeef14
phase: verification
agents: argos
global: APTO
pbi_archived: true
approval_status: aprobado
checks:
  AC-TIMEOUT: APTO
  AC-SKIP: APTO
  AC-BRANCH: APTO
  AC-PERSIST: APTO
  AC-TESTS: APTO
  AC-PR: APTO
  L-TIMEOUT-ENV: APTO
  L-SKIP-LAB: APTO
  L-GENOME: APTO
  DOC_OBJECTIVES: APTO
  DOC_CLARIFY: APTO
  DOC_SPEC: APTO
  DOC_PLAN: APTO
  DOC_IMPLEMENTATION: APTO
  DOC_EXECUTION: APTO
  DOC_FRONTMATTER_YAML: APTO
  MIX_EVOLUTION: APTO
git_changes:
  - SddIA/engine/execute-process/src/engine/executor.rs
  - SddIA/engine/execute-process/src/engine/workspace_init.rs
  - SddIA/engine/execute-process/src/engine/handlers/task_queue_manager.rs
  - SddIA/engine/execute-process/src/engine/agent_runtime.rs
  - SddIA/engine/execute-process/src/engine/eda_bus_topology.rs
  - SddIA/scripts/tools/kalma2-agent-runtime-cursor.py
  - SddIA/scripts/tools/test_kalma2_runtime_timeout.py
  - docs/features/kalma2-phase-barrier-timeout-persist/
  - docs/todos/done/[REFACTOR] Kalma2 — serialización de fases, timeout y rama refactor (KALMA2-AUD-4b9de6).md
  - SddIA/evolution/d630a6cf-1767-4751-a2b9-b1f4210a01fb.md
  - SddIA/evolution/Evolution_log.md
---

# Validación — kalma2-phase-barrier-timeout-persist

Argos. Evidencia determinista: unitarios Rust/Python + inspección de contratos en el diff. Sin e2e Kalma2 (el runtime es el objeto; circular).

## Evidencia física

| Comando | Resultado |
|---------|-----------|
| `cargo test -p execute-process --lib -- workspace_init agent_runtime::tests handlers::task_queue_manager::tests executor::tests` | **24 passed** |
| `python3 SddIA/scripts/tools/test_kalma2_runtime_timeout.py` | **4 passed** |

## Criterios

| ID | Veredicto | Evidencia |
|----|-----------|-----------|
| **AC-TIMEOUT** | APTO | `SOFT_CONFIG_MARKERS` sin `"timeout"`. `is_soft_config_error("timeout 600s")` → False. `TimeoutExpired` emite `timeout {n}s` → `failed`. |
| **AC-SKIP** | APTO | Loop `executor::run_generic` arma barrera si agente ∈ {failed, blocked, awaiting_agents, awaiting}. `barrier_sequence_skips_verification_after_failed_execution`: Verificación y Cierre → `skipped`. `simulated` no dispara. |
| **AC-BRANCH** | APTO | `refactor_prefix_is_not_rewritten_to_feat`; TQM `refactor_child_keeps_suggested_branch_and_persist_ref`. HEAD de trabajo = `refactor/kalma2-phase-barrier-timeout-persist`. |
| **AC-PERSIST** | APTO | TQM inyecta `persist_ref` (FM o Cúmulo). `resolve_persist_ref_value` fallback workspace. Prompt Python `resolve_persist_ref`. |
| **AC-TESTS** | APTO | 24 Rust + 4 Python cubren TQM, workspace-init, timeout, barrera. |
| **AC-PR** | APTO | PR https://github.com/racso80es/SddIA/pull/174 ; PBI en `docs/todos/done/`; `validacion.md` APTO en el mismo PR. |
| **L-TIMEOUT-ENV** | APTO | Default 600. Override `SDDIA_AGENT_RUNTIME_TIMEOUT_SECS_EJECUCION` solo fase Ejecución. Default no subido. |
| **L-SKIP-LAB** | APTO | Skip PBI archive / delivery-close también si `process_name == refactorization`. |
| **L-GENOME** | APTO | `refactorization.md` y `git-operations.md` intocados. |
| **MIX_EVOLUTION** | APTO | Diff sin `docs/features/evolution-history-normalization/`. |

## Límites de evidencia

- No se disparó hijo Kalma2 real con `cursor-agent` timeout 600 s (circular). La cadena timeout→failed→barrera está cubierta en unitario.
- `phase_terminal` mantiene `awaiting_agents` neutral; el corte de Argos es la barrera.

## Veredicto

**APTO** para cierre documental en rama y `delivery-close-cycle`.
