---
skill_id: git-save-snapshot
spec_version: 1.0.0
status: Active
name: Git Save Snapshot
owner: tekton-developer
contract_ref: SddIA/skills/skills-contract.json
implementation_path_ref: paths.skillCapsules.git-save-snapshot
capsule_io_ref: SddIA/norms/capsule-json-io.md
parameters:
  commitMessage:
    type: string
    required: true
  all:
    type: boolean
    required: false
    default: true
  files:
    type: array
    required: false
phases:
  - id: commit
    name: Add y commit
    steps:
      - git add (-A o lista files) y git commit -m
related_agents:
  - tekton-developer
rules:
  - envelope JSON v2
  - No sustituye invoke-commit para flujos con scope conventional detallado
---
# Skill: git-save-snapshot

## Entrada (request)

| Campo | Tipo | Descripción |
|-------|------|-------------|
| `commitMessage` | string | Obligatorio. |
| `all` | boolean | Default true → `git add -A`. |
| `files` | string[] | Si `all` es false, lista de rutas para `git add`. |
