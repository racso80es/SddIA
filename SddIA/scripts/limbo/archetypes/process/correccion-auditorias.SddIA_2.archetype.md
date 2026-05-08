---
contract_ref: paths.processPath/process-contract.json
input_ref: paths.auditsPath
name: Corrección según Auditorías
persist_ref: paths.featurePath/<nombre_correccion>
phases:
  - description: Revisar últimos informes en paths.auditsPath; consolidar hallazgos (críticos/medios/bajos). Ejecutar git-workspace-recon para validar entorno limpio antes de ramificar.
    id: '0'
    name: Análisis de auditorías
  - description: objectives.md con hallazgos priorizados y criterios de cierre. Crear rama feat/correccion-segun-auditorias o feat/correccion-auditorias-<id> con git-branch-manager (nunca master).
    id: '1'
    name: Documentación de objetivos y rama
  - description: Acción spec; spec.md (YAML Frontmatter).
    id: '2'
    name: Especificación
  - description: Acción clarify si aplica; clarify.md (YAML Frontmatter).
    id: '3'
    name: Clarificación
  - description: Acción planning; plan.md (YAML Frontmatter).
    id: '4'
    name: Planificación
  - description: Acción implementation; implementation.md (YAML Frontmatter).
    id: '5'
    name: Implementación (doc)
  - description: Acción execution; aplicar correcciones. Consolidar hitos con git-save-snapshot. Ante fallo estructural, git-tactical-retreat (con confirmación explícita).
    id: '6'
    name: Ejecución
  - description: Acción validate; git-workspace-recon para coherencia; validacion.md (YAML Frontmatter).
    id: '7'
    name: Validar
  - description: Acción finalize-process; git-sync-remote; git-create-pr con objectives.md y validacion.md en el cuerpo del PR; Evolution Logs.
    id: '8'
    name: Finalizar
principles_ref: paths.principlesPath
process_id: correccion-auditorias
related_actions:
  - spec
  - clarify
  - planning
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
  - documentation
  - security-audit
spec_version: 2.0.0
---
# Proceso: Corrección según Auditorías (spec_version 2.0.0)

Este documento define el **proceso de tarea** para la corrección de hallazgos derivados de auditorías (**spec_version 2.0.0**), con **Arsenal Táctico Git (S+)**: `git-workspace-recon`, `git-branch-manager`, `git-save-snapshot`, `git-sync-remote`, `git-tactical-retreat`, `git-create-pr`. Ubicación: paths.processPath/correccion-auditorias/ (Cúmulo). Entrada: paths.auditsPath. Persistencia: **Cúmulo** (paths.featurePath/<nombre_correccion>).

**Interfaz de proceso:** Cumple la interfaz en Cúmulo (`process_interface`): solicita/genera en la carpeta de la tarea (Cúmulo) un **`.md` por acción** con **YAML Frontmatter** (objectives.md, spec.md, clarify.md, plan.md, implementation.md, execution.md, validacion.md, finalize-process.md). No ficheros .json separados en esa carpeta de tarea. Norma: SddIA/norms/features-documentation-frontmatter.md.

## Propósito

El proceso **correccion-auditorias** orquesta el ciclo de corrección de hallazgos en paths.auditsPath: análisis, priorización, documentación y ejecución alineada con feature.

## Entrada

- **Fuentes:** Informes en paths.auditsPath (p. ej. AUDITORIA_YYYY_MM_DD.md).
- **Artefacto de análisis:** objectives.md con hallazgos y prioridades.

## Alcance

- **Rama:** feat/correccion-segun-auditorias o feat/correccion-auditorias-<identificador> (nunca master); **git-workspace-recon** antes de **git-branch-manager**.
- **Documentación:** paths.featurePath/<nombre_correccion>/ con ciclo completo en .md (YAML Frontmatter).
- **Skills:** suite táctica en `related_skills`; documentation y security-audit cuando aplique.
- **Restricciones:** Priorizar hallazgos críticos; alcance acotado a lo auditado.

## Fases (resumen)

| Fase | Nombre | Descripción |
| :--- | :--- | :--- |
| **0** | Análisis | Revisar paths.auditsPath; **git-workspace-recon** |
| **1** | Objetivos y rama | objectives.md; **git-branch-manager** |
| **2–5** | Spec → implementación (doc) | Acciones spec, clarify, planning, implementation |
| **6** | Ejecución | Código; **git-save-snapshot**; **git-tactical-retreat** si procede |
| **7** | Validar | validate + **git-workspace-recon**; validacion.md |
| **8** | Finalizar | **git-sync-remote**; **git-create-pr**; Evolution Logs |

## Integración

- Informes en paths.auditsPath (Auditor).
- Orquestación: Arquitecto y Tekton.
- SSOT: paths.featurePath/<nombre_correccion>/ (Cúmulo).

## Referencias

- paths.auditsPath, paths.featurePath (Cúmulo)
- Proceso feature: paths.processPath/feature/
- AGENTS.md — Leyes Universales
