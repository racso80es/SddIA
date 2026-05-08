---
contract_ref: paths.processPath/process-contract.json
name: Refactorization
persist_ref: paths.featurePath/refactorization-<nombre_refactor>
phases:
  - description: Ejecutar git-workspace-recon para validar entorno limpio. Tras confirmar, crear rama feat/refactorization-<nombre_refactor> desde master usando git-branch-manager.
    id: '0'
    name: Preparar entorno
  - description: objectives.md en carpeta de la tarea.
    id: '1'
    name: Documentación con objetivos
  - description: Acción spec; salida spec.md (YAML Frontmatter).
    id: '2'
    name: Especificación
  - description: Acción clarify; clarify.md (YAML Frontmatter).
    id: '3'
    name: Clarificación
  - description: Acción planning; plan.md (YAML Frontmatter).
    id: '4'
    name: Planificación
  - description: Acción implementation; implementation.md (YAML Frontmatter).
    id: '5'
    name: Implementación (doc)
  - description: Aplicar el plan al código. Consolidar hitos con git-save-snapshot (commits atómicos). Ante fallo estructural, git-tactical-retreat (con confirmación explícita según norma).
    id: '6'
    name: Ejecución
  - description: Acción validate; git-workspace-recon para coherencia frente al plan. validacion.md (YAML Frontmatter).
    id: '7'
    name: Validar
  - description: Cierre. git-sync-remote; git-create-pr con resumen de objectives.md y validacion.md en el cuerpo del PR. Evolution Logs.
    id: '8'
    name: Finalizar
process_id: refactorization
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
spec_version: 2.0.0
---
# Proceso: Refactorización (spec_version 2.0.0)

Este documento define el **proceso de tarea** para una refactorización (**spec_version 2.0.0**), con **Arsenal Táctico Git (S+)**: `git-workspace-recon`, `git-branch-manager`, `git-save-snapshot`, `git-sync-remote`, `git-tactical-retreat`, `git-create-pr`. Ubicación: paths.processPath/refactorization/ (Cúmulo). Acciones: paths.actionsPath (Cúmulo). Persistencia: **Cúmulo** (paths.featurePath/refactorization-<nombre_refactor>).

**Interfaz de proceso:** Cumple la interfaz en Cúmulo (`process_interface`): solicita/genera en la carpeta de la tarea (Cúmulo) un **`.md` por acción** con **YAML Frontmatter** (objectives.md, spec.md, clarify.md, plan.md, implementation.md, execution.md, validacion.md). No ficheros .json separados. Norma: SddIA/norms/features-documentation-frontmatter.md.

## Propósito

El proceso **refactorization** define el ciclo completo para una refactorización: mismo flujo que feature (rama, documentación, spec, implementación, validación, cierre), sin añadir funcionalidad nueva. Orquesta **spec**, **clarify**, **planning**, **implementation**, **execution**, **validate** y **finalize-process**.

## Alcance del procedimiento

Ruta de la tarea: Cúmulo (paths.featurePath/refactorization-<nombre_refactor>).

| Fase | Nombre | Descripción |
| :--- | :--- | :--- |
| **0** | Preparar entorno | **git-workspace-recon** → **git-branch-manager**: rama `feat/refactorization-<nombre_refactor>` desde `master` actualizado. |
| **1** | Documentación con objetivos | objectives.md en carpeta de la tarea (Cúmulo). |
| **2** | Especificación | Acción spec → spec.md (YAML Frontmatter). |
| **3** | Clarificación | Acción clarify → clarify.md (YAML Frontmatter). |
| **4** | Planificación | Acción planning → plan.md (YAML Frontmatter). |
| **5** | Implementación (doc) | Acción implementation → implementation.md (YAML Frontmatter). |
| **6** | Ejecución | Refactor en código; **git-save-snapshot**; **git-tactical-retreat** si procede (confirmación explícita). |
| **7** | Validar | Acción validate; **git-workspace-recon**; validacion.md (YAML Frontmatter). |
| **8** | Finalizar | **git-sync-remote** → **git-create-pr** (objectives + validacion en PR); Evolution Logs. |

## Contenido mínimo de la carpeta de la tarea (Cúmulo)

| Documento | Contenido |
| :--- | :--- |
| **objectives.md** | Objetivo, alcance, análisis de situación actual, ley aplicada. |
| **spec.md** | Especificación técnica (YAML Frontmatter). |
| **clarify.md** | Clarificaciones (YAML Frontmatter, si aplica). |
| **implementation.md** | Touchpoints y plan de implementación (YAML Frontmatter). |
| **validacion.md** | Resultado de la validación pre-PR (YAML Frontmatter). |

## Actualización de Evolution Logs

Al cierre (fase 8): paths.evolutionPath + paths.evolutionLogFile — sección con fecha, título, resumen y referencia a objectives.md en la carpeta de la tarea (Cúmulo).

## Integración con Agentes

- **Arquitecto:** Inicia el procedimiento y asegura la fase 1 y la ubicación (Cúmulo).
- **Tekton Developer:** Ejecuta fases 4–8; aplica la SPEC como marco legal.
- **Cúmulo:** Valida documentación en la carpeta de la tarea (Cúmulo) como SSOT.

## Dependencias

El proceso **refactorization** utiliza las mismas acciones que **feature** en paths.actionsPath (Cúmulo). Referencia: paths.processPath/feature/.

## Estándares de Calidad

- **Grado S+:** Trazabilidad objetivo → PR: recon → rama → documentación → snapshots → validación (recon) → sync → PR.
- **Ley GIT:** Ningún commit en `master`; trabajo en rama dedicada con documentación en la carpeta de la tarea (Cúmulo).
- **SSOT:** Documentación canónica en la carpeta de la tarea (Cúmulo).

---
*Proceso reflejo de feature, adaptado al contexto de refactorización. Referencia: paths.processPath/feature/.*
