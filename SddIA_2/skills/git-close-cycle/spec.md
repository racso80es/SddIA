---
contract_ref: paths.skillsDefinitionPath/skills-contract.json (Cúmulo)
implementation_path_ref: paths.skillCapsules.git-close-cycle
name: Git Close Cycle
owner: tekton-developer
skill_id: git-close-cycle
spec_version: 1.0.0
status: Active
---

# Skill: git-close-cycle

Cierra el **ciclo de desarrollo local** tras publicar el trabajo (y opcionalmente tras fusión en remoto): vuelve a la rama de integración (`main` o `master`, vía `origin/HEAD` o fallback), sincroniza con el remoto y elimina la rama de trabajo local.

## Request (envelope v2)

`meta.entityId` = `git-close-cycle`.

| Campo | Tipo | Obligatorio | Notas |
|-------|------|-------------|--------|
| `targetBranch` | string | sí | Rama local a eliminar (p. ej. `feat/...`). Alias JSON: `target_branch`. |
| `remote` | string | no | Por defecto `origin`. |
| `workingDirectory` | string | no | Raíz del repo si no es el cwd. |

## Secuencia Git

1. Si `targetBranch` coincide con la rama de integración detectada → salida exitosa (`skipped`).
2. `git checkout` → rama de integración.
3. `git pull <remote> HEAD`.
4. `git fetch --prune <remote>`.
5. Si existe la rama local `targetBranch`: `git branch -d`; si falla, `git branch -D`.

## CLI

`Git-Close-Cycle.bat --target-branch <rama> [--remote origin]`
