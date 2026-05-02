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
  - description: Ejecutar git-workspace-recon para validar entorno limpio. Tras confirmar, crear rama fix/<nombre_fix> desde master usando git-branch-manager (nunca trabajar en master).
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
  - description: Aplicar el fix al código. Consolidar hitos con git-save-snapshot (commits atómicos). Ante fallo estructural del entorno, git-tactical-retreat (solo con confirmación explícita según norma del proyecto).
    id: '5'
    name: Ejecución
  - description: Acción validate; git-workspace-recon para coherencia frente al plan. Salida validacion.md (YAML Frontmatter).
    id: '6'
    name: Validar
  - description: Cierre. git-sync-remote para publicación segura; git-create-pr con cuerpo del PR enlazando objectives.md y validacion.md. Evolution Logs si aplica.
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
# Proceso: Bug Fix (spec_version 2.0.0)

Este documento define el **proceso de tarea** para la corrección de un bug (**spec_version 2.0.0**), con **Arsenal Táctico Git (S+)**: `git-workspace-recon`, `git-branch-manager`, `git-save-snapshot`, `git-sync-remote`, `git-tactical-retreat`, `git-create-pr`. Ubicación: paths.processPath/bug-fix/ (Cúmulo). Persistencia: **Cúmulo** (paths.fixPath/<nombre_fix>).

**Interfaz de proceso:** Cumple la interfaz en Cúmulo (`process_interface`): solicita/genera en la carpeta de la tarea (Cúmulo) un **`.md` por acción** con **YAML Frontmatter** (objectives.md, spec.md, clarify.md, validacion.md). No ficheros .json separados. Norma: SddIA/norms/features-documentation-frontmatter.md.

## Propósito

El proceso **bug-fix** orquesta el ciclo del bug: triaje, documentación, reproducción, alcance mínimo del fix y trazabilidad en paths.fixPath/<nombre_fix>.

## Alcance

- **Rama:** fix/<nombre_fix> (nunca master); aislar con **git-branch-manager** tras **git-workspace-recon**.
- **Documentación:** paths.fixPath/<nombre_fix>/ con objectives.md, spec.md, clarify.md si aplica, implementation.md, validacion.md (YAML Frontmatter; sin .json separados en esa carpeta).
- **Skills Git (S+):** suite táctica en `related_skills`; además documentation, filesystem-ops, dotnet-development según `skills`.
- **Restricciones:** Alcance mínimo (causa raíz); no refactor ni ampliación de funcionalidad en la misma rama.

## Fases (resumen)

| Fase | Nombre | Descripción |
| :--- | :--- | :--- |
| **0** | Preparar entorno | **git-workspace-recon** → **git-branch-manager** (rama fix/ desde master actualizado). |
| **1** | Objetivos | objectives.md |
| **2** | Especificación | Acción **spec** → spec.md |
| **3** | Clarificación | Acción **clarify** si aplica |
| **4** | Implementación (doc) | Acción **implementation** |
| **5** | Ejecución | Fix en código; **git-save-snapshot**; **git-tactical-retreat** solo si procede y con confirmación. |
| **6** | Validar | Acción **validate** + **git-workspace-recon**; validacion.md |
| **7** | Finalizar | **git-sync-remote** → **git-create-pr** (objetivos + validación en cuerpo del PR); logs si aplica |

## Integración

El agente Bug Fix Specialist orquesta el ciclo. Referencia canónica: **paths.fixPath/<nombre_fix>/** (Cúmulo). SSOT para ese fix.
