---
skill_id: git-branch-manager
spec_version: 1.0.0
status: Active
name: Branch Manager
owner: tekton-developer
contract_ref: SddIA/skills/skills-contract.json
implementation_path_ref: paths.skillCapsules.git-branch-manager
capsule_io_ref: SddIA/norms/capsule-json-io.md
parameters:
  branchName:
    type: string
    required: true
  create:
    type: boolean
    required: false
    default: false
phases:
  - id: checkout
    name: Checkout o creación
    steps:
      - Si create=true, crear rama branchName; si no, cambiar a rama branchName
related_agents:
  - tekton-developer
rules:
  - envelope JSON v2
---
# Skill: git-branch-manager

## Objetivo

Crear rama nueva desde HEAD o cambiar a rama existente.

## Entrada (request)

| Campo | Tipo | Descripción |
|-------|------|-------------|
| `branchName` | string | Obligatorio. Nombre de rama (puede incluir prefijo feat/ si se desea). |
| `create` | boolean | Default false. true = crear rama nueva. |
