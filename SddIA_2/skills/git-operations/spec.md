---
common_workflows:
  start_task: git-workspace-recon → git-branch-manager (crear rama feat/ o fix/)
  save_progress: git-save-snapshot o invoke-commit (commit atómico)
  finalize-process: git-sync-remote → git-create-pr
contract_ref: paths.skillsDefinitionPath/skills-contract.json (Cúmulo)
name: Git Operations Standard
rules:
  - 'STRICT PROHIBITION: Never commit directly to ''master'' or ''main''.'
  - 'Mandatory Branching: Work must be done in ''feat/<desc>'' or ''fix/<desc>'' branches.'
  - 'Commit Message Format: Use Conventional Commits (e.g. ''feat: add login'', ''fix: resolve null ref'').'
  - 'Pre-Push: Always run local validation (build/tests) before requesting to push.'
skill_id: git-operations
---
# Skill: Git Operations Standard

**skill_id:** `git-operations`

## Objetivo

Uso seguro y semántico de Git: ramas feat/fix, commits convencionales, pre-push.

## Reglas

- **STRICT PROHIBITION:** Never commit directly to 'master' or 'main'.
- **Mandatory Branching:** Work must be done in 'feat/<desc>' or 'fix/<desc>' branches.
- **Commit Message Format:** Use Conventional Commits (e.g. 'feat: add login', 'fix: resolve null ref').
- **Pre-Push:** Always run local validation (build/tests) before requesting to push.

## Common workflows

- start_task: git-workspace-recon → git-branch-manager
- save_progress: git-save-snapshot o invoke-commit
- finalize-process: git-sync-remote → git-create-pr

## Alcance

Skill de definición únicamente (sin cápsula ejecutable). Consumido por agentes y procesos (feature, bug-fix).

---
*Definición en paths.skillsDefinitionPath/git-operations/ (contrato paths.skillsDefinitionPath/skills-contract.md).*
