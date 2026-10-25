---
feature_name: kaizen-linear-backlog-intake-guard
branch: feat/kaizen-linear-backlog-intake-guard
global: APTO
tracker_ref: AUT-14
pr_url: https://github.com/racso80es/SddIA/pull/389
merged_pr: https://github.com/racso80es/SddIA/pull/389
merge_commit: eb7692ddd84b724d482a24b90dd28e4732eae01b
ci_run: https://github.com/racso80es/SddIA/actions/runs/40220716586
ci_installer_run: https://github.com/racso80es/SddIA/actions/runs/40220716606
ci_head: 0de2897f5aa23b80a30a2b8a285750f98a3af3a8
closed: "2026-10-25"
checks:
  AC-1: APTO
  AC-2: APTO
  AC-3: APTO
  AC-4: APTO
  CI: APTO
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
  - SddIA/evolution/d48f4421-71c8-4dce-b0b7-4a814d047a9b.md
pbi_archived: false
---

# Validación — AUT-14

`done_gate` linear: issue **AUT-14** en Done; sin fichero en `docs/todos/pending/` ni `docs/todos/done/`.

## Local

- `SddIA/scripts/qa/test-linear-todos-intake-gate.sh` — OK.
- Pre-commit simulado: alta en `docs/todos/pending/` → BLOCKED con `provider: linear`.
- Hook Cursor: `Write` a cola todos → `permission: deny`.

## CI (rama del PR)

Run [40220716586](https://github.com/racso80es/SddIA/actions/runs/40220716586) (`sddia-index-qa`) y [40220716606](https://github.com/racso80es/SddIA/actions/runs/40220716606) (`sddia-installer-qa`), `headSha` `0de2897f` — **success**.

## Merge

PR [#389](https://github.com/racso80es/SddIA/pull/389) → `main` @ `eb7692ddd84b724d482a24b90dd28e4732eae01b` (2026-10-25).
