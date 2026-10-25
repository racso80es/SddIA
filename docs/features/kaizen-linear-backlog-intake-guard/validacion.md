---
feature_name: kaizen-linear-backlog-intake-guard
branch: feat/kaizen-linear-backlog-intake-guard
global: NO_APTO
tracker_ref: AUT-14
checks:
  AC-1: APTO
  AC-2: APTO
  AC-3: APTO
  AC-4: APTO
  CI: PENDIENTE-CI
git_changes:
  - .cursor/hooks.json
  - .cursor/rules/task-closure-documental.mdc
  - .cursorrules
  - .gitignore
  - SddIA/scripts/qa/cursor-guard-todos.sh
  - SddIA/scripts/qa/git-hooks/hook_common.sh
  - SddIA/scripts/qa/git-hooks/pre_commit_gate.sh
  - SddIA/scripts/qa/test-linear-todos-intake-gate.sh
  - docs/features/kaizen-linear-backlog-intake-guard/
pbi_archived: false
---

# Validación — AUT-14

Criterios locales APTO. `global: NO_APTO` hasta PR y CI verde. Cierre Linear: issue en `Done` en el mismo PR.
