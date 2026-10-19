---
feature_name: kalma2-reincidence-stale-runtime-and-failed-projection
created: "2026-10-19"
process: bug-fix
branch: fix/kalma2-reincidence-stale-runtime-and-failed-projection
persist_ref: docs/fixes/kalma2-reincidence-stale-runtime-and-failed-projection
pbi_ref: docs/todos/done/[FIX] Kalma2 — reincidencia PR 347 runtime fósil e init failed en status.md
document_id: PBI-FIX-KALMA2-REINCIDENCE-STALE-RUNTIME-AND-FAILED-PROJECTION
uuid: "e3b27269-6736-4735-ba6f-fd94bb87d1c2"
global: APTO
pbi_archived: true
checks:
  R-CA1: APTO
  R-CA2: APTO
  R-CA3: APTO
  R-CA4: APTO
  R-CA5: APTO
  R-CA6: APTO
git_changes:
  - SddIA/interfaces/kalma2-bridge/src/main.rs
  - SddIA/engine/execute-process/src/engine/workspace_init.rs
  - SddIA/engine/execute-process/src/engine/invoke_orchestrator.rs
  - SddIA/engine/execute-process/src/engine/capsule_paths.rs
  - SddIA/engine/execute-process/src/engine/capsule_seal.rs
  - SddIA/sddia-daemon-runtime/src/lib.rs
  - SddIA/sddia-daemon-runtime/src/github_bridge.rs
  - SddIA/daemons/telegram-watcher/src/main.rs
  - SddIA/daemons/event-watcher/src/main.rs
  - SddIA/daemons/event-watcher.md
  - SddIA/daemons/event-sweeper/src/main.rs
  - SddIA/daemons/event-sweeper.md
  - SddIA/tools/sddia-qa/src/resolve.rs
  - start-sddia.sh
  - docs/fixes/kalma2-reincidence-stale-runtime-and-failed-projection/
  - docs/todos/done/[FIX] Kalma2 — reincidencia PR 347 runtime fósil e init failed en status.md
  - docs/features/arquitecturalinearhu-b07manifiestocore-selfyactivacindone-gateli/_agent_handoff.md
  - docs/features/arquitecturalinearhu-b07manifiestocore-selfyactivacindone-gateli/validacion.md
---

# Validación — reincidencia PR #347

**Veredicto global: APTO.**

| ID | Criterio | Estado | Evidencia |
|----|----------|--------|-----------|
| R-CA1 | `phase_status: failed` pisa `awaiting_agents` | APTO | `project_status_failed_phase_overrides_awaiting_agents_pec` |
| R-CA2 | Rollback de rama nueva si `dirty-worktree` aborta | APTO | `run_dirty_outside_scope_aborts_without_system_fracture` |
| R-CA3 | Debug solo si es más nuevo que release; testigos en `start-sddia.sh` | APTO | Tests de mtime en orquestador, puente y daemon-runtime. Sellado release de `execute-process` y `kalma2-bridge` |
| R-CA4 | Replay SSE en la primera lectura | APTO | `progress_stream_replay_arrives_on_first_read` |
| R-CA5 | Binarios recompilados y puente reciclado | APTO | PID 26035 → 38851; `exe` = `SddIA/target/release/kalma2-bridge` |
| R-CA6 | Tests unitarios de los dos huecos | APTO | Los siete tests del ciclo, 0 fallos |

## Cierre documental

PBI en `docs/todos/done/` con `status: done`. `pbi_archived: true`.
