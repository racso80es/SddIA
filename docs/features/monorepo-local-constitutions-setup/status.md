---
feature_name: monorepo-local-constitutions-setup
status: in_progress
updated: 2026-05-11
branch_name: feat/monorepo-local-constitutions-setup
pr: https://github.com/racso80es/SddIA/pull/1
---

# Estado: forja constitucional global

## Completado

### Motor federal

- `SddIA/CONSTITUTION_CORE.md` §6: bylaws tácticos bajo `.SddIA/constitution/`; el Core prevalece en colisión.
- `SddIA/agents/cumulo.instructions.json`: mapa `.SddIA/local.paths.json` y cláusula de jerarquía federal.
- Evolución motor: `SddIA/evolution/e1f2a3b4-c5d6-4789-e012-3456789abcde.md`.

### Laboratorios `SddIA_1`…`SddIA_4`

- `.SddIA/local.paths.json` con `local_constitution` y `local_evolution`.
- `.SddIA/constitution/` (`CONSTITUTION.md`, `constitution.json`, `README.md`).
- `.SddIA/evolution/` (`README.md`, `Evolution_log.md`, entrada UUID de migración).
- Purga en raíz: `CONSTITUTION.md`, `constitution.json`, `constitution/`.
- Script reutilizable: `SddIA/scripts/migrate-local-constitutions-once.py`.

### Starter-kit (plantilla cliente)

- Carpeta canónica renombrada a `SddIA/scripts/starter-kit/.SddIA/` (antes `.sddia/`).
- `local.paths.json` alineado a laboratorios (`local_evolution`, `local_evolution_log`).
- Plantilla `.SddIA/evolution/` (`README.md`, `Evolution_log.md`).
- `constitution.json` con `L9_LOCAL_EVOLUTION` y `paths_ref` a `.SddIA/local.paths.json`.
- `CONSTITUTION.md` con frontmatter YAML y cláusula de sumisión al Core federal.
- `sddia-sync.ps1`: zonas sagradas `.SddIA/tools/`, `.SddIA/constitution/`, `.SddIA/evolution/`.
- READMEs y `local-security-contract.json` actualizados a `.SddIA/`.

## Pendiente / deuda

- Referencias legacy `.sddia/` en contratos Core, evolution `f81e4b2a`, tools de laboratorio y arquetipos Limbo.
- Barrido global de duplicados (process, normas, limbo) fuera del alcance de esta feature.
- `hash_integrity` en evolutions editados tras el cierre documental.
- Decidir commit en la rama del PR o PR aparte para starter-kit y documentación de estado.

## PR

- Rama: `feat/monorepo-local-constitutions-setup`
- PR: https://github.com/racso80es/SddIA/pull/1 (constitución + motor; starter-kit y `status.md` pueden requerir commit adicional).
