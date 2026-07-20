---
feature_name: kalma2-process-dispatch
created: "2026-07-20"
process: feature
branch: feat/kalma2-process-dispatch
global: APTO
pbi_archived: true
checks:
  AC1_tqm_no_input_validation: APTO
  AC2_child_dispatch: APTO
  AC3_pbi_ref_spaces: APTO
  AC4_bridge_untouched: APTO
  AC5_iota_out_of_scope: APTO
git_changes:
  - SddIA/engine/execute-process/src/engine/handlers/task_queue_manager.rs
  - SddIA/engine/execute-process/src/engine/handlers/mod.rs
  - SddIA/engine/execute-process/src/engine/handlers/kalma2.rs
  - SddIA/engine/execute-process/src/engine/invoke_orchestrator.rs
  - SddIA/engine/execute-process/src/engine/mod.rs
  - docs/features/kalma2-process-dispatch/
  - docs/todos/done/[FIX] interacción con front kalma2.md
---

# Validación — kalma2-process-dispatch

## Veredicto

**APTO** (Argos lab). Causa raíz `INPUT_VALIDATION`/`tasks_path` cerrada vía handler nativo TQM; matiz A′ verificado en emisión.

## Checks

| ID | Evidencia |
|----|-----------|
| AC1 | TQM con `{process:bug-fix, task_text, correlation_id}` → `success` (antes: missing `tasks_path`) |
| AC2 | `dispatched_process=bug-fix`, fases Triaje+Despacho `executed`, `child.execution_id` presente |
| AC3 | Prompt con em-dash/espacios → payload evento con `pbi_ref` completo |
| AC4 | Diff sin `interfaces/kalma2/app.js` ni write EDA en bridge |
| AC5 | Sin cambios IOTA / publisher |

## Cierre documental

PBI archivado en `docs/todos/done/[FIX] interacción con front kalma2.md` · `pbi_archived: true`.
