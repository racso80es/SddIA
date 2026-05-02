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
  - description: >-
      Ejecutar git-workspace-recon para validar entorno limpio. Tras confirmar, usar git-branch-manager para aislar el
      contexto en la rama feat/create-skill-<skill-id> (nunca master como trabajo activo).
    id: '0'
    name: Preparar entorno
  - description: objectives.md y spec.md (YAML Frontmatter); acción spec según contrato de acciones.
    id: '1'
    name: Objetivos y especificación
  - description: >-
      Definición en paths.skillsDefinitionPath, cápsula opcional en paths.skillCapsules, índice y Cúmulo. Durante la
      mutación del repositorio, consolidar hitos con git-save-snapshot. Ante fallo estructural, valorar git-tactical-retreat
      según política y confirmación requerida.
    id: '2'
    name: Definición, cápsula e integración
  - description: Acción validate; validacion.md (YAML Frontmatter). Verificar contrato skills y binario/cápsula si aplica.
    id: '3'
    name: Validar
  - description: >-
      Cierre. Ejecutar git-sync-remote y git-create-pr incorporando al cuerpo del Pull Request objectives.md,
      validacion.md y referencia a paths.skillsDefinitionPath / paths.skillCapsules.
    id: '4'
    name: Finalizar
process_doc_ref: paths.processPath/create-skill/
process_id: create-skill
process_interface_compliance: 'Genera en carpeta de la tarea un .md por acción con YAML Frontmatter (objectives.md, spec.md, implementation.md, validacion.md); no ficheros .json separados. Entregable con ejecutable: cápsula en paths.skillCapsules[<skill-id>]. Norma: features-documentation-frontmatter.md.'
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
skills_contract_ref: SddIA/skills/skills-contract.json
spec_version: 2.0.0
triggers:
  - Crear nueva skill con skill-id (definición y, si aplica, cápsula)
  - Solicitud de incorporación de skill al índice y Cúmulo
---
# Proceso: Creación de skills (create-skill)

Este documento define el **proceso de tarea** para crear una nueva skill en el proyecto. Está ubicado en paths.processPath/create-skill/ (Cúmulo). Las rutas se obtienen de **Cúmulo** (paths.skillsDefinitionPath, paths.skillCapsules, paths.skillsIndexPath, paths.skillsRustPath).

**Interfaz de proceso:** Cumple la interfaz en Cúmulo (`process_interface`): la tarea genera en la carpeta de la tarea (Cúmulo) un **`.md` por acción** con **YAML Frontmatter** (objectives.md, spec.md, implementation.md, validacion.md). No ficheros .json separados para esas acciones. El **resultado ejecutable** (si la skill tiene implementación invocable) es la cápsula en **paths.skillCapsules[<skill-id>]** según **SddIA/skills/skills-contract.md** y envelope **SddIA/norms/capsule-json-io.md**. Norma: SddIA/norms/features-documentation-frontmatter.md.

## Propósito

El proceso **create-skill** incorpora una skill al ecosistema: definición en paths.skillsDefinitionPath, cápsula opcional, índice paths.skillsIndexPath y registro en Cúmulo (paths.skillCapsules). Analogía con **create-tool** sustituyendo herramientas por skills (ver paths.processPath/create-tool/spec.md).

## Alcance del procedimiento

- **Documentación de la tarea:** paths.featurePath/create-skill-<skill-id>/.
- **Definición:** paths.skillsDefinitionPath/<skill-id>/ con spec.md y spec.json (**implementation_path_ref** obligatorio si hay cápsula ejecutable).
- **Cápsula (implementación invocable):** paths.skillCapsules[<skill-id>].

**Skills solo documentales:** únicamente paths.skillsDefinitionPath/<skill-id>/ (sin entrada nueva en skillCapsules si no hay binario ni scripts).

Fases (resumen operativo): **0** git-workspace-recon → git-branch-manager (rama feat/create-skill-&lt;skill-id&gt;) | **1** Objetivos y especificación | **2** Definición, cápsula opcional, índice, Cúmulo (hitos con git-save-snapshot; rescate con git-tactical-retreat si aplica) | **3** Validación | **4** git-sync-remote → git-create-pr con artefactos de la tarea en el cuerpo del PR.

## Restricciones

- skill_id en kebab-case. Rama feat/create-skill-<skill-id>. Windows 11, PowerShell 7+ (convención GesFer). Contrato skills obligatorio.

## Implementación ejecutable (cápsula)

Alineado a **SddIA/skills/skills-contract.md**:

- Código Rust: paths.skillsRustPath (p. ej. `src/bin/<skill_bin>.rs`).
- Binario: copiar a **&lt;cápsula&gt;/bin/** tras build o script de instalación del repo.
- Cápsula: **manifest.json**, documentación **.md**, launcher **.bat** (prefiere `bin/*.exe`), fallback **.ps1** si no hay binario.

**Prohibiciones:** no usar como entrega principal del registro evolution únicamente `.ps1` donde el contrato exija binario Rust (ver norma evolution).

## Referencias

- Contrato: SddIA/skills/skills-contract.md, skills-contract.json.
- Cúmulo: paths.skillsDefinitionPath, paths.skillCapsules, paths.skillsIndexPath, paths.skillsRustPath.
- Proceso análogo: paths.processPath/create-tool/spec.md.
- Portabilidad del proceso: SddIA/norms/reproducir-create-skill-en-otros-entornos-sddia.md; skill asociada: paths.skillsDefinitionPath/reproducir-create-skill-sddia/spec.md.
- Portabilidad de una skill concreta: paths.skillsDefinitionPath/reproducir-skills-en-otros-entornos-sddia/spec.md.
- Machine-readable: paths.processPath/create-skill/spec.json.
