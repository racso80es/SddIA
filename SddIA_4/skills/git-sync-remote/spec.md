---
skill_id: git-sync-remote
spec_version: 1.0.0
status: Active
name: Git Sync Remote
owner: tekton-developer
contract_ref: SddIA/skills/skills-contract.json
implementation_path_ref: paths.skillCapsules.git-sync-remote
capsule_io_ref: SddIA/norms/capsule-json-io.md
parameters:
  remote:
    type: string
    required: false
    default: origin
  branch:
    type: string
    required: false
phases:
  - id: sync
    name: Fetch y pull
    steps:
      - Obtener cambios del remoto (fetch)
      - Opcional cambiar a branch
      - Integrar cambios del remoto (pull)
related_agents:
  - tekton-developer
rules:
  - envelope JSON v2
---
# Skill: git-sync-remote

## Entrada (request)

| Campo | Tipo | Descripción |
|-------|------|-------------|
| `remote` | string | Default `origin`. |
| `branch` | string | Si se indica y no es la actual, checkout antes del pull. |
