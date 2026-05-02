---
constraints:
  - Rama fix/<nombre_fix>, nunca master.
  - Documentación obligatoria en paths.fixPath/<nombre_fix>.
  - 'Alcance mínimo: causa raíz; no refactor en la misma rama.'
contract_ref: paths.processPath/process-contract.json
name: Bug Fix
paths:
  bugPath_ref: paths.bugPath (Cúmulo)
  fixPath_ref: paths.fixPath (Cúmulo)
persist_ref: paths.fixPath/<nombre_fix>
phases:
  - description: >-
      Ejecutar git-workspace-recon para validar entorno limpio y coherencia del árbol de trabajo. Tras confirmar,
      aislar contexto con git-branch-manager creando o seleccionando la rama fix/<nombre_fix> desde la base acordada
      (nunca master como rama de trabajo activa).
    id: '0'
    name: Preparar entorno
  - description: objectives.md en carpeta de la tarea (YAML Frontmatter).
    id: '1'
    name: Documentación con objetivos
  - description: Acción spec; salida spec.md (YAML Frontmatter).
    id: '2'
    name: Especificación
  - description: Acción clarify si aplica; clarify.md (YAML Frontmatter).
    id: '3'
    name: Clarificación
  - description: Acción implementation; implementation.md (YAML Frontmatter).
    id: '4'
    name: Implementación (doc)
  - description: >-
      Acción execution; execution.md (YAML Frontmatter). Consolidar hitos atómicos con git-save-snapshot. Ante fallo
      estructural del entorno o corrupción del árbol, usar git-tactical-retreat según contrato y política del equipo.
    id: '5'
    name: Ejecución
  - description: Acción validate; validacion.md (YAML Frontmatter). Opcionalmente revalidar con git-workspace-recon.
    id: '6'
    name: Validar
  - description: >-
      Cierre del ciclo. Ejecutar git-sync-remote para publicar la rama de forma segura; a continuación git-create-pr
      incorporando al cuerpo del Pull Request el resumen de objectives.md y validacion.md (y enlace a paths.fixPath).
    id: '7'
    name: Finalizar
principles_ref: paths.principlesPath
process_id: bug-fix
process_interface_compliance: Solicita/genera en carpeta de la tarea un .md por acción con YAML Frontmatter (objectives.md, spec.md, clarify.md, validacion.md); no ficheros .json separados. Norma: features-documentation-frontmatter.md.
related_actions:
  - spec
  - clarify
  - implementation
  - execution
  - validate
  - finalize-process
related_skills:
  - git-workspace-recon
  - git-branch-manager
  - git-save-snapshot
  - git-sync-remote
  - git-tactical-retreat
  - git-create-pr
skills:
  - documentation
  - filesystem-ops
  - dotnet-development
spec_version: 2.0.0
triggers:
  - Reporte de bug
  - Fallo en CI o tests
  - Solicitud de fix con bug-id o título
---
# Proceso: Bug Fix

Este documento define el **proceso de tarea** para la corrección de un bug. Está ubicado en paths.processPath/bug-fix/ (Cúmulo). La ruta de persistencia se obtiene de **Cúmulo** (paths.fixPath/<nombre_fix>).

**Interfaz de proceso:** Cumple la interfaz en Cúmulo (`process_interface`): solicita/genera en la carpeta de la tarea (Cúmulo) un **`.md` por acción** con **YAML Frontmatter** (objectives.md, spec.md, clarify.md, validacion.md). No ficheros .json separados. Norma: SddIA/norms/features-documentation-frontmatter.md.

## Propósito

El proceso **bug-fix** orquesta el ciclo de vida del bug: triaje, documentación, reproducción, alcance mínimo del fix y trazabilidad en la carpeta de persistencia del fix (paths.fixPath/<nombre_fix>).

## Alcance

- **Rama:** fix/<nombre_fix> (nunca master). **Inicio:** git-workspace-recon → git-branch-manager.
- **Documentación:** Carpeta paths.fixPath/<nombre_fix>/ con objectives.md, spec.md, clarify.md si aplica, implementation.md, validacion.md (todos con YAML Frontmatter; no ficheros .json separados).
- **Skills Git (Grado S+):** git-workspace-recon, git-branch-manager, git-save-snapshot, git-sync-remote, git-tactical-retreat, git-create-pr (Cúmulo: paths.skillCapsules). **Dominio:** documentation, filesystem-ops, dotnet-development.
- **Ejecución:** hitos consolidados con git-save-snapshot; rescate con git-tactical-retreat solo ante fallo estructural acorde a política.
- **Cierre:** git-sync-remote → git-create-pr con objectives.md y validacion.md enlazados en el cuerpo del PR.
- **Restricciones:** Alcance mínimo (solo causa raíz); no refactorizar ni ampliar funcionalidad en la misma rama.

## Integración

El agente Bug Fix Specialist (definición en paths.processPath/bug-fix/) orquesta el ciclo. En la descripción del PR y en Evolution Logs, la referencia canónica es **paths.fixPath/<nombre_fix>/** (Cúmulo). SSOT para ese fix.
