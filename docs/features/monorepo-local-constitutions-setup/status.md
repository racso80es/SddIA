---
feature_name: monorepo-local-constitutions-setup
status: completed
updated: 2026-05-11
branch_name: main
---

# Estado: forja constitucional global

## Línea base

- Rama operativa: `main` (sincronizada con `origin/main`).
- Integración: PR #1 y PR #2 (`feat/monorepo-local-constitutions-setup`); cierre documental y starter-kit en commits posteriores sobre `main`.
- Seguimiento de deuda transversal: `SddIA/evolution/f81e4b2a-6c0d-4a8f-9e31-2d7b8a4c1e00.md`.

## Completado

### Motor federal

- `SddIA/CONSTITUTION_CORE.md` §6: bylaws tácticos bajo `.SddIA/constitution/`; el Core prevalece en colisión.
- `SddIA/agents/cumulo.instructions.json`: mapa `.SddIA/local.paths.json`, jerarquía federal y `[EVO]` motor (`directories.evolution`, `normative_documents.evolution_log`).
- `SddIA/core/cumulo.paths.json`: `normative_documents.evolution_log` y `normative_documents.evolution_contract`.
- Normas `paths-via-cumulo.md` y `sddia-evolution-sync.md` alineadas a claves `directories.*` / `normative_documents.*` (sin `paths.sddiaEvolution*` en Core).
- Evolución motor: `SddIA/evolution/e1f2a3b4-c5d6-4789-e012-3456789abcde.md` (`hash_integrity` sellado).

### Laboratorios `SddIA_1`…`SddIA_4`

- `.SddIA/local.paths.json` con `local_constitution` y `local_evolution`.
- `.SddIA/constitution/` (`CONSTITUTION.md`, `constitution.json`, `README.md`).
- `.SddIA/evolution/` (`README.md`, `Evolution_log.md`, entrada UUID de migración).
- Purga en raíz: `CONSTITUTION.md`, `constitution.json`, `constitution/`.
- `constitution.json` (L8 motor / L9 local) alineado a claves Cúmulo actuales en `SddIA_1`, `SddIA_3` y `SddIA_4`; `SddIA_2` sin L8 explícita en JSON (solo L9 local).
- Script one-shot: `scripts/migrate-local-constitutions-once.py`.

### Rutas `.SddIA/` (Core y tools de laboratorio)

- Contratos e índices del Core y `definition_path_ref` en cápsulas `scripts/tools/{name}/spec.json` usan `.SddIA/` como convención canónica.
- Inventarios `.SddIA/tools/` (`index.md`, `README.md`) alineados en los cuatro laboratorios.

### Starter-kit (plantilla cliente)

- Plantilla bajo `SddIA/scripts/starter-kit/.SddIA/`.
- `local.paths.json`, `constitution/`, `evolution/`, `sddia-sync.ps1` (zonas sagradas) y READMEs en convención `.SddIA/`.
- `CONSTITUTION.md` con trazabilidad motor (`directories.evolution`) vs instancia (`directories.local_evolution`).

### Integridad documental (motor)

- `hash_integrity` sellado en todos los ficheros de `SddIA/evolution/` (incl. `e1f2a3b4`, `80a6462c`, `51b4d573`, `ebdc4cb8`).

## Pendiente / deuda (fuera de esta feature)

- Arquetipos Limbo y copias legacy en `SddIA_1`…`SddIA_4` (process, normas, actions) respecto al SSOT `SddIA/`.
- Referencias `paths.sddiaEvolution*` y rutas `.sddia/` en Limbo y en scripts de migración mecánica (`migrate-local-tools-once.ps1`, `kernel-raw-execute.ps1`).
- Barrido global de duplicados y gobernanza `spec.json` vs `{name}.md` (mapa en `f81e4b2a` §3–4).
