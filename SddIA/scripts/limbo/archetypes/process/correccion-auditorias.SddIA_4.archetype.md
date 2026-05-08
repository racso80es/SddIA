---
contract_ref: paths.processPath/process-contract.json
input_ref: paths.auditsPath
name: Corrección según Auditorías
persist_ref: paths.featurePath/<nombre_correccion>
phases:
  - description: >-
      Ejecutar git-workspace-recon para validar entorno limpio. Tras confirmar, usar git-branch-manager para aislar el
      contexto en la rama feat/correccion-segun-auditorias o feat/correccion-auditorias-<identificador> (nunca master como trabajo activo).
    id: '0'
    name: Preparar entorno
  - description: Revisar últimos informes en paths.auditsPath; consolidar hallazgos (críticos/medios/bajos).
    id: '1'
    name: Análisis de auditorías
  - description: objectives.md con hallazgos priorizados y criterios de cierre.
    id: '2'
    name: Documentación de objetivos
  - description: Acción spec; spec.md, spec.json.
    id: '3'
    name: Especificación
  - description: Acción clarify si aplica; clarify.md, clarify.json.
    id: '4'
    name: Clarificación
  - description: Acción planning; plan.
    id: '5'
    name: Planificación
  - description: Acción implementation; implementation.md, implementation.json.
    id: '6'
    name: Implementación (doc)
  - description: >-
      Acción execution; execution.json o registro equivalente. Consolidar hitos atómicos con git-save-snapshot. Ante fallo
      estructural del entorno, valorar git-tactical-retreat según política y confirmación requerida.
    id: '7'
    name: Ejecución
  - description: Acción validate; validacion.json.
    id: '8'
    name: Validar
  - description: >-
      Acción finalize-process; Evolution Logs. Ejecutar git-sync-remote y git-create-pr incorporando al cuerpo del Pull Request
      el resumen de objectives.md, hallazgos abordados y validacion (referencia a paths.featurePath/<nombre_correccion>).
    id: '9'
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
# Proceso: Corrección según Auditorías

Este documento define el **proceso de tarea** para la corrección de hallazgos derivados de auditorías (paths.auditsPath). Está ubicado en paths.processPath/correccion-auditorias/ (Cúmulo). La ruta de persistencia se obtiene de **Cúmulo** (paths.featurePath/<nombre_correccion>).

**Interfaz de proceso:** Cumple la interfaz en Cúmulo (`process_interface`): solicita/genera en la carpeta de la tarea (Cúmulo) al menos un **`.md`** (objectives.md, spec.md, clarify.md) y al menos un **`.json`** (spec.json, audit-hallazgos.json o similar, validacion.json).

## Propósito

El proceso **correccion-auditorias** orquesta el ciclo de corrección de hallazgos reportados en los informes de auditoría (paths.auditsPath): análisis de auditorías recientes, priorización de hallazgos, documentación de objetivos, y ejecución de correcciones mediante feature o bug-fix según el tipo de hallazgo.

## Entrada

- **Fuentes:** Informes en paths.auditsPath (p. ej. AUDITORIA_YYYY_MM_DD.md, validacion-*.json).
- **Artefacto de análisis:** Documento de objetivos (objectives.md) que consolida hallazgos, prioridades y alcance.

## Alcance

- **Rama:** feat/correccion-segun-auditorias o feat/correccion-auditorias-<identificador> (nunca master). **Inicio:** git-workspace-recon → git-branch-manager.
- **Documentación:** Carpeta paths.featurePath/<nombre_correccion>/ con objectives.md (objetivo, hallazgos consolidados, prioridades), spec.md/spec.json, clarify.md si aplica, implementation, validacion.json.
- **Skills Git (Grado S+):** git-workspace-recon, git-branch-manager, git-save-snapshot, git-sync-remote, git-tactical-retreat, git-create-pr. **Dominio:** documentation, invoke-command, security-audit cuando aplique.
- **Ejecución:** hitos con git-save-snapshot; rescate con git-tactical-retreat si aplica.
- **Cierre:** git-sync-remote → git-create-pr con objectives y validacion enlazados en el cuerpo del PR.
- **Restricciones:** Priorizar hallazgos críticos (compilación, seguridad, violación de capas); alcance acotado por lo reportado en auditorías.

## Fases

0. **Preparar entorno:** git-workspace-recon; git-branch-manager para la rama de corrección.
1. **Análisis de auditorías:** Revisar últimos informes en paths.auditsPath y consolidar hallazgos (críticos / medios / bajos).
2. **Documentación de objetivos:** Redactar objectives.md con hallazgos priorizados y criterios de cierre.
3. **Especificación y plan:** Acciones spec, clarify, planning según ciclo feature.
4. **Implementación y ejecución:** Aplicar correcciones; cada hallazgo puede ser un ítem de implementation; consolidar con git-save-snapshot.
5. **Validación y cierre:** validacion.json; registrar en paths.auditsPath o Evolution Logs; git-sync-remote y git-create-pr con artefactos de la tarea en el PR.

## Integración

- Los agentes **Auditor (Back/Front/Process)** generan los informes en paths.auditsPath.
- El ciclo de corrección lo orquestan **Arquitecto** y **Tekton** usando este proceso.
- Referencia canónica de la tarea: paths.featurePath/<nombre_correccion>/ (Cúmulo). SSOT para esa ronda de corrección.

## Referencias

- paths.auditsPath (Cúmulo)
- paths.featurePath (Cúmulo)
- Proceso feature: paths.processPath/feature/
- AGENTS.md — Leyes Universales (COMPILACIÓN, GIT, Soberanía documental)
