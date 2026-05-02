---
skill_id: git-close-cycle
spec_version: 1.0.0
status: Active
name: Git Close Cycle
owner: tekton-developer
contract_ref: SddIA/skills/skills-contract.json
implementation_path_ref: paths.skillCapsules.git-close-cycle
capsule_io_ref: SddIA/norms/capsule-json-io.md
parameters:
  target_branch:
    type: string
    required: true
  remote:
    type: string
    required: false
    default: origin
phases:
  - id: close_local
    name: Cierre local post-fusión
    steps:
      - Resolver rama base (origin/HEAD, main o master)
      - Checkout rama base
      - pull remote HEAD
      - fetch --prune
      - branch -d target_branch (fallback -D)
related_agents:
  - tekton-developer
rules:
  - envelope JSON v2
---
# Skill: git-close-cycle

## Propósito

Cerrar el **ciclo de desarrollo local** tras una **fusión en remoto**: quedar en la rama base actualizada y eliminar la rama de trabajo indicada.

## Entrada (`request`)

| Campo | Tipo | Descripción |
|-------|------|-------------|
| `target_branch` | string | **Requerido.** Nombre de la rama local a eliminar (p. ej. `feat/...`). |
| `remote` | string | Opcional. Por defecto `origin`. |

## Comportamiento

1. Resolver rama base: `refs/remotes/<remote>/HEAD` → nombre corto; si no, `main` o `master` si existen.
2. Fallar si `target_branch` está vacío o coincide con la rama base.
3. `git checkout <base>`.
4. `git pull <remote> HEAD`.
5. `git fetch --prune <remote>`.
6. Si la rama `target_branch` no existe localmente, terminar con éxito e informar omisión.
7. En caso contrario: `git branch -d <target_branch>`; si falla, `git branch -D <target_branch>`.

## Orquestación

Invocar desde **finalize-process** cuando la tarea está finalizada y, para poder usar `-d` sin forzar, la rama ya está fusionada en remoto. La invocación debe usar cápsula / `.tekton_request.json` (norma COMANDOS).
