---
skill_id: git-workspace-recon
spec_version: 1.0.0
status: Active
name: Git Workspace Recon
owner: tekton-developer
contract_ref: SddIA/skills/skills-contract.json
implementation_path_ref: paths.skillCapsules.git-workspace-recon
capsule_io_ref: SddIA/norms/capsule-json-io.md
parameters:
  request:
    description: Objeto libre; puede ser {}.
    required: false
    type: object
phases:
  - id: recon
    name: Reconocimiento
    steps:
      - Leer envelope v2
      - Ejecutar git status --porcelain, remote -v, log -1 --oneline
related_agents:
  - tekton-developer
rules:
  - Invocación vía envelope JSON v2 (SddIA/norms/capsule-json-io.md)
  - Ley COMANDOS; git solo a través de esta skill o skills equivalentes
---
# Skill: git-workspace-recon

**skill_id:** `git-workspace-recon`

## Objetivo

Obtener un **snapshot legible** del estado del repositorio Git actual (rama, porcelana, remotes, último commit) para diagnóstico u orquestación.

## Entrada (request)

| Campo | Tipo | Descripción |
|-------|------|-------------|
| — | — | Sin campos obligatorios. |

## Salida (result)

| Campo | Descripción |
|-------|-------------|
| `branch` | Rama actual o mensaje de error. |
| `gitStatusPorcelain` | `{ exitCode, output }`. |
| `gitRemoteV` | `{ exitCode, output }`. |
| `gitLogOneLine` | `{ exitCode, output }`. |

## Ejemplo envelope

```json
{"meta":{"schemaVersion":"2.0","entityKind":"skill","entityId":"git-workspace-recon"},"request":{}}
```
