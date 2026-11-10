---

feature_name: man-39-refactorization-reentry-cli-collapse
created: "2026-11-10"
process: feature
items:
  - T0-forge-reentry-skip
  - T1-timeout-floor
  - T3-dirty-peaje
  - T5-lab-kaizen-label
branch_name: feat/man-39-refactorization-reentry-cli-collapse
persist_ref: docs/features/man-39-refactorization-reentry-cli-collapse
pbi_ref: MAN-39
execution_id: "454bcd99-8bbb-4cb3-8038-31cbe046ac55"
---
# Implementación — man-39-refactorization-reentry-cli-collapse

## Touchpoints

| Tajo | Archivo | Cambio |
|------|---------|--------|
| T0, T4 | `engine/execute-process/src/engine/executor.rs` | `try_skip_forge_agent_phase`; barrera no bloquea con `forge_already_present` |
| T1 | `engine/execute-process/src/engine/agent_runtime.rs` | Suelo 2400 s en Ejecución; env vacío ignorado |
| T3 | `engine/execute-process/src/engine/workspace_init.rs` | `path_in_scope` admite `docs/audits/{tracker_ref}.md` |
| T5 | `tools/linear-tracker-adapter/src/main.rs` | Catálogo lab MAN incluye `Kaizen` |
| T2 | — | Abort soberano con limpieza de hijos: pendiente (solo tests de timeout/peaje en este PR) |

## Propuesta aplicada

Reentrada detectada por `implementation.md` + `execution.md` en `persist_ref`. Peaje termodinámico del mismo `tracker_ref` deja de disparar `dirty-worktree`.
