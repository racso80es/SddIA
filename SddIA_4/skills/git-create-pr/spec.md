---
skill_id: git-create-pr
spec_version: 1.0.0
status: Active
name: Git Create PR
owner: tekton-developer
contract_ref: SddIA/skills/skills-contract.json
implementation_path_ref: paths.skillCapsules.git-create-pr
capsule_io_ref: SddIA/norms/capsule-json-io.md
parameters:
  branch:
    type: string
    required: false
  base:
    type: string
    required: false
  title:
    type: string
    required: false
  body:
    type: string
    required: false
  bodyFile:
    type: string
    required: false
  pushFirst:
    type: boolean
    required: false
    default: true
phases:
  - id: pr
    name: Push y PR
    steps:
      - Publicar rama en remoto (si pushFirst=true)
      - Crear Pull Request con gh (si disponible)
related_agents:
  - tekton-developer
rules:
  - envelope JSON v2
  - Requiere gh instalado y autenticado
---
# Skill: git-create-pr

## Entrada (request)

| Campo | Tipo | Descripción |
|-------|------|-------------|
| `branch` | string | Default rama actual. |
| `base` | string | Rama base del PR (default detectada / main). |
| `title` | string | Default nombre de rama. |
| `body` | string | Cuerpo del PR. |
| `bodyFile` | string | Ruta de fichero con cuerpo (prioridad sobre body si ambos). |
| `pushFirst` | boolean | Default true. |

## Notas

Si `gh pr create` falla, la respuesta incluye `hint` con URL de comparación cuando el remoto es GitHub.
