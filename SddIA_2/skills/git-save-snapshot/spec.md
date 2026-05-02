---
contract_ref: paths.skillsDefinitionPath/skills-contract.json (Cúmulo)
implementation_path_ref: paths.skillCapsules.git-save-snapshot
name: Git Save Snapshot
owner: tekton-developer
skill_id: git-save-snapshot
spec_version: 1.0.0
status: Active
---

# Skill: git-save-snapshot

Similar a `invoke-commit` pero orientado a **request JSON** (camelCase) del arsenal Git.

Envelope v2: `meta.entityId` = `git-save-snapshot`.

Obligatorio: `commitMessage` y (`addAll: true` o `files: [...]`).
