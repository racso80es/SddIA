---
contract_ref: paths.skillsDefinitionPath/skills-contract.json (Cúmulo)
implementation_path_ref: paths.skillCapsules.git-tactical-retreat
name: Git Tactical Retreat
owner: tekton-developer
skill_id: git-tactical-retreat
spec_version: 1.0.0
status: Active
---

# Skill: git-tactical-retreat

**Visión Zero:** si `hardReset` o `discardWorkingTree` es true, `confirmDestructive` debe ser true.

Si `targetRef` ≠ `HEAD` y `hardReset`, se ejecuta `git fetch --all` antes del reset.

Envelope v2: `meta.entityId` = `git-tactical-retreat`.
