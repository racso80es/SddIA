---
contract_ref: paths.skillsDefinitionPath/skills-contract.json (Cúmulo)
implementation_path_ref: paths.skillCapsules.git-branch-manager
name: Git Branch Manager
owner: tekton-developer
skill_id: git-branch-manager
spec_version: 1.0.0
status: Active
---

# Skill: git-branch-manager

- `create: true` → crear rama y cambiar a ella.
- `create: false` y `checkout: true` → cambiar a rama existente.
- `create: false` y `checkout: false` → crear referencia de rama sin cambio de contexto.

Envelope v2: `meta.entityId` = `git-branch-manager`.
