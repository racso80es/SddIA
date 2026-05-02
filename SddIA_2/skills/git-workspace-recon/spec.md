---
contract_ref: paths.skillsDefinitionPath/skills-contract.json (Cúmulo)
implementation_path_ref: paths.skillCapsules.git-workspace-recon
invocation_schema: "2.0"
name: Git Workspace Recon
owner: tekton-developer
skill_id: git-workspace-recon
spec_version: 1.0.0
status: Active
---

# Skill: git-workspace-recon

Ejecutable: `paths.skillsRustPath` → `git_workspace_recon.exe` en cápsula (`bin/`).

## Envelope (v2)

`meta.schemaVersion` = `2.0`, `meta.entityKind` = `skill`, `meta.entityId` = `git-workspace-recon`.

## Request

- `workingDirectory` (opcional): raíz del repo.

## Resultado

Objeto con `repositoryRoot`, `branch`, `headAbbrev`, `statusPorcelain`, `remotesText`, `lastCommitOneline`.
