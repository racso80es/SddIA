---
feature_name: monorepo-local-constitutions-setup
branch_name: feat/monorepo-local-constitutions-setup
process: feature v1.0.0 V5
persist_ref: docs/features/monorepo-local-constitutions-setup/
---

# Plan: forja constitucional global

## Objetivo

Instanciar constitución táctica bajo `.SddIA/constitution/` en cada laboratorio, alinear Cúmulo con la jerarquía Core vs local, purgar legacy en raíz y registrar evolución motor e instancia.

## Convención

- Instancia local: **`.SddIA/`** (mayúsculas), coherente con tools ya desplegadas.
- Motor federal: `SddIA/CONSTITUTION_CORE.md` y `SddIA/core/cumulo.paths.json`.

## Iteración por laboratorio

| Workspace | Perfil | Origen táctico |
|-----------|--------|----------------|
| SddIA_1 | Backend Admin | GesFer.Admin.Back |
| SddIA_3 | Backend Admin (réplica) | GesFer.Admin.Back |
| SddIA_2 | Frontend Admin | GesFer.Admin.Front |
| SddIA_4 | Frontend Product | GesFer.Product.Front |

## Entregables por nodo

1. `.SddIA/local.paths.json` con `local_constitution` y `local_evolution`.
2. `.SddIA/constitution/CONSTITUTION.md` + `constitution.json`.
3. `.SddIA/evolution/` (README, Evolution_log, entrada de migración).
4. Purga de `CONSTITUTION.md`, `constitution.json` y `constitution/` en raíz del lab.

## Motor

- Cláusula de paradoja en `SddIA/agents/cumulo.instructions.json`.
- Alineación de `CONSTITUTION_CORE.md` §6 a `.SddIA/constitution/`.
- Registro en `SddIA/evolution/`.
