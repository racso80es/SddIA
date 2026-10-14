---
feature_name: kalma2-branch-before-mutate
created: "2026-10-14"
process: bug-fix
branch_name: fix/kalma2-branch-before-mutate
persist_ref: docs/fixes/kalma2-branch-before-mutate
items_applied:
  - workspace-init-branch-first
  - executor-init-failed-barrier
  - progress-trace-verdict
  - kalma2-project-status-phase-end
---

# Implementation — kalma2-branch-before-mutate

| Artefacto | Cambio |
|-----------|--------|
| `SddIA/engine/execute-process/src/engine/workspace_init.rs` | `checkout_work_branch` primero; gate dirty después; recreación de rama nueva tras pull de base |
| `SddIA/engine/execute-process/src/engine/executor.rs` | `barrier_from_phase_entry`; PTC fin con veredicto |
| `SddIA/engine/execute-process/src/engine/progress_trace.rs` | `short_verdict`, `build_message` con veredicto |
| `SddIA/interfaces/kalma2-bridge/src/main.rs` | Lectura PTC; `project_status` no restaura espera |
| `SddIA/evolution/a3c5accf-54fd-45ed-9b1e-fdf444e836e6.md` | Correlato evolution del motor |
