---
common_workflows:
  finalize-process: publicar rama y abrir PR vía skills (git-sync-remote → git-create-pr) o acción finalize-process
  save_progress: 'git commit -m ''<type>: <short_description>'''
  start_task: crear/cambiar rama vía git-branch-manager
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

- start_task: crear/cambiar rama vía git-branch-manager
- save_progress: git commit -m '<type>: <short_description>'
- finalize-process: publicar rama y abrir PR vía skills (git-sync-remote → git-create-pr) o acción finalize-process

## Alcance

Skill de definición únicamente (sin cápsula ejecutable). Consumido por agentes y procesos (feature, bug-fix).

---
*Definición en paths.skillsDefinitionPath/git-operations/ (contrato paths.skillsDefinitionPath/skills-contract.md).*
