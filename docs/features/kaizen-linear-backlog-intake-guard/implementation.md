---
feature_name: kaizen-linear-backlog-intake-guard
created: "2026-10-25"
process: feature
tracker_ref: AUT-14
---

# Implementación — AUT-14

| Touchpoint | Cambio |
|------------|--------|
| `.cursor/rules/task-closure-documental.mdc` | Sección alta/refinamiento backlog |
| `.cursorrules` | §8 backlog linear + forge/refine |
| `SddIA/scripts/qa/git-hooks/hook_common.sh` | Helpers provider linear + buckets todos |
| `SddIA/scripts/qa/git-hooks/pre_commit_gate.sh` | Gate ficheros nuevos en cola |
| `SddIA/scripts/qa/cursor-guard-todos.sh` | Guard Cursor |
| `.cursor/hooks.json` | Registro `preToolUse` |
| `.gitignore` | `/_refine_lab_*/`, `/_forge_lab_*/` |
| `SddIA/scripts/qa/test-linear-todos-intake-gate.sh` | Smoke helpers |
