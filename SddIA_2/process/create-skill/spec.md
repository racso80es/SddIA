---
contract_ref: paths.processPath/process-contract.json
inputs:
  description: Descripción breve. Obligatorio.
  skill_id: kebab-case. Obligatorio.
name: Create Skill
paths:
  featurePath_ref: paths.featurePath (Cúmulo)
  skillCapsulesRef: paths.skillCapsules
  skillsDefinitionPath_ref: paths.skillsDefinitionPath (Cúmulo)
  skillsIndexPath_ref: paths.skillsIndexPath (Cúmulo)
  skillsRustPath_ref: paths.skillsRustPath (Cúmulo)
persist_ref: paths.featurePath/create-skill-<skill-id>
phases:
  - description: Ejecutar git-workspace-recon para validar entorno limpio. Tras confirmar, crear rama feat/create-skill-<skill-id> desde master usando git-branch-manager.
    id: '0'
    name: Preparar entorno
  - description: Objetivos, spec e implementación documentada; definición en paths.skillsDefinitionPath.
    id: '1'
    name: Especificación y definición SddIA
  - description: Cápsula, índice, Cúmulo; hitos con git-save-snapshot. Ante fallo estructural, git-tactical-retreat (con confirmación explícita).
    id: '2'
    name: Implementación de cápsula e integración
  - description: Validación contractual y operativa de la skill.
    id: '3'
    name: Validar
  - description: Cierre. git-sync-remote; git-create-pr inyectando objectives.md y validacion.md en el cuerpo del PR.
    id: '4'
    name: Finalizar
process_doc_ref: paths.processPath/create-skill/
process_id: create-skill
process_interface_compliance: 'Genera en carpeta de la tarea un .md por acción con YAML Frontmatter (objectives.md, spec.md, implementation.md, validacion.md); no ficheros .json separados. Entrega ejecutable: cápsula en paths.skillCapsules[<skill-id>]. Norma: features-documentation-frontmatter.md.'
related_actions:
  - spec
  - implementation
  - validate
  - finalize-process
related_skills:
  - git-workspace-recon
  - git-branch-manager
  - git-save-snapshot
  - git-sync-remote
  - git-tactical-retreat
  - git-create-pr
spec_version: 2.0.0
skills_contract_ref: SddIA/skills/skills-contract.json
triggers:
  - Crear nueva skill con skill-id
  - Solicitud de creación de skill ejecutable o solo definición
---
# Proceso: Creación de skills (create-skill) (spec_version 2.0.0)

Este documento define el **proceso de tarea** para crear una nueva skill (**spec_version 2.0.0**), con **Arsenal Táctico Git (S+)**: `git-workspace-recon`, `git-branch-manager`, `git-save-snapshot`, `git-sync-remote`, `git-tactical-retreat`, `git-create-pr`. Está en paths.processPath/create-skill/ (Cúmulo). Rutas: **Cúmulo** (paths.skillsDefinitionPath, paths.skillCapsules, paths.skillsIndexPath).

**Interfaz de proceso:** Cumple la interfaz en Cúmulo (`process_interface`): la tarea genera en la carpeta de la tarea (Cúmulo) un **`.md` por acción** con **YAML Frontmatter** (objectives.md, spec.md, implementation.md, validacion.md). No ficheros .json separados en esa carpeta. El **resultado ejecutable** (si aplica) es la cápsula en **paths.skillCapsules[&lt;skill-id&gt;]** con artefactos alineados a SddIA/skills/skills-contract.json. Norma: SddIA/norms/features-documentation-frontmatter.md.

## Propósito

El proceso **create-skill** incorpora una skill al ecosistema: definición en paths.skillsDefinitionPath, cápsula opcional, índice paths.skillsIndexPath y entrada en paths.skillCapsules del Cúmulo. Es **espejo** del proceso **create-tool** sustituyendo herramientas por skills (ver paths.processPath/create-tool/spec.md).

## Alcance del procedimiento

- **Documentación de la tarea:** paths.featurePath/create-skill-&lt;skill-id&gt;/.
- **Definición (SddIA):** paths.skillsDefinitionPath/&lt;skill-id&gt;/ con spec.md y spec.json (implementation_path_ref obligatorio si hay cápsula).
- **Cápsula (implementación):** paths.skillCapsules[&lt;skill-id&gt;] cuando el skill sea invocable.

**Flujo Git (S+):** Fase 0 — **git-workspace-recon** y **git-branch-manager** (`feat/create-skill-<skill-id>`). Implementación — **git-save-snapshot**; emergencia — **git-tactical-retreat** (confirmación). Cierre — **git-sync-remote** y **git-create-pr** (objectives + validacion en el PR).

Fases orientativas: 0 Preparar entorno | 1 Objetivos y especificación | 1b Definición en SddIA | 2–6 Cápsula, manifest, launchers, índice, Cúmulo | 7 Opcional Rust | 8 Validación | 9 Cierre (sync + PR).

## Restricciones

- skill_id en kebab-case. Rama feat/create-skill-&lt;skill-id&gt;. Windows 11, PowerShell 7+. Contrato skills obligatorio para skills con ejecutable.

## Implementación de la skill (ejecutable)

La skill invocable debe implementarse **preferentemente como ejecutable Rust** (`.exe`), en **la raíz de la cápsula** (paridad con paths.processPath/create-tool y skills-contract actualizado).

**Estructura esperada de la cápsula:**

```
paths.skillCapsules[<skill-id>]/
├── <skill-name>.exe             # Ejecutable Rust compilado (OBLIGATORIO si hay binario)
├── manifest.json               # Metadatos (OBLIGATORIO)
├── <skill-name>-config.json     # Configuración (si aplica)
├── <skill-name>.md              # Documentación de uso (OBLIGATORIO)
├── <Skill-Name>.bat             # Launcher que invoca el .exe
└── <skill-name>.ps1             # Fallback si no hay .exe
```

**Fuente Rust:**

Código en paths.skillsRustPath (Cúmulo), p. ej. `scripts/skills-rs/src/bin/<skill_bin>.rs` o módulo dedicado.

**Compilación (referencia):**

1. Desarrollar bajo paths.skillsRustPath.
2. Compilar con el manifest Cargo del proyecto skills-rs del repo.
3. Copiar el `.exe` a la cápsula en paths.skillCapsules[&lt;skill-id&gt;].
4. Actualizar paths.skillsIndexPath.
5. Actualizar SddIA/agents/cumulo.paths.json (`skillCapsules`).

**Nota:** Cápsulas legadas con `bin/` pueden coexistir hasta migración; las **nuevas** skills creadas con este proceso usan `.exe` en raíz de cápsula.

## Referencias

- Contrato: SddIA/skills/skills-contract.json, skills-contract.md.
- Cúmulo: paths.skillsDefinitionPath, paths.skillCapsules, paths.skillsIndexPath, paths.skillsRustPath.
- Proceso análogo: paths.processPath/create-tool/spec.md.
- Machine-readable: paths.processPath/create-skill/spec.json.
