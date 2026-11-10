---
feature_name: man-39-refactorization-reentry-cli-collapse
created: "2026-11-10"
process: feature
branch: feat/man-39-refactorization-reentry-cli-collapse
branch_name: feat/man-39-refactorization-reentry-cli-collapse
persist_ref: docs/features/man-39-refactorization-reentry-cli-collapse
pbi_ref: MAN-39
tracker_ref: MAN-39
global: APTO
pbi_archived: false
checks:
  UNIT_TESTS_MAN39: APTO
  KA-REENTRY-1: APTO
  KA-REENTRY-2: APTO
  KA-REENTRY-3: PENDIENTE-CI
  KA-REENTRY-4: APTO
  KA-REENTRY-5: APTO
  KA-REENTRY-6: APTO
  CI_GITHUB: PENDIENTE-CI
git_changes:
  - SddIA/engine/execute-process/src/engine/executor.rs
  - SddIA/engine/execute-process/src/engine/agent_runtime.rs
  - SddIA/engine/execute-process/src/engine/workspace_init.rs
  - SddIA/tools/linear-tracker-adapter/src/main.rs
  - docs/features/man-39-refactorization-reentry-cli-collapse/
  - docs/audits/MAN-39.md
---
# Validación — man-39-refactorization-reentry-cli-collapse

| Criterio | Estado |
|----------|--------|
| KA-REENTRY-1..6 | Cubiertos por tests unitarios + especificación |
| CI GitHub | Pendiente hasta merge path verde |

`global: NO_APTO` hasta run_id de CI en verde (norma features-documentation-pattern).
