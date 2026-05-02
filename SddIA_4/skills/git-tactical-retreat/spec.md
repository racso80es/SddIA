---
skill_id: git-tactical-retreat
spec_version: 1.0.0
status: Active
name: Git Tactical Retreat
owner: tekton-developer
contract_ref: SddIA/skills/skills-contract.json
implementation_path_ref: paths.skillCapsules.git-tactical-retreat
capsule_io_ref: SddIA/norms/capsule-json-io.md
parameters:
  hardReset:
    type: boolean
    required: false
  cleanUntracked:
    type: boolean
    required: false
  confirmDestructive:
    type: boolean
    required: true
  target:
    type: string
    required: false
    default: HEAD
phases:
  - id: destructive
    name: Reset / clean
    steps:
      - Validar confirmDestructive si hardReset o cleanUntracked
      - Opcional git reset --hard target
      - Opcional git clean -fd
related_agents:
  - tekton-developer
  - security-engineer
rules:
  - envelope JSON v2
  - Visión Zero confirmDestructive obligatorio si hay operación destructiva
---
# Skill: git-tactical-retreat

## Entrada (request)

| Campo | Tipo | Descripción |
|-------|------|-------------|
| `hardReset` | boolean | `git reset --hard`. |
| `cleanUntracked` | boolean | `git clean -fd`. |
| `confirmDestructive` | boolean | **true** obligatorio si cualquier operación destructiva está activa. |
| `target` | string | Referencia para reset (default `HEAD`). |
