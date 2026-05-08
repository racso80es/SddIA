---
contract_ref: paths.processPath/process-contract.json
name: Feature
persist_ref: paths.featurePath/<nombre_feature>
phases:
  - description: Ejecutar git-workspace-recon para validar entorno limpio. Tras confirmar, crear rama feat/<nombre> o fix/<nombre> desde master usando git-branch-manager.
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
  - description: Aplicar el plan al código. Consolidar hitos ejecutando git-save-snapshot (commits atómicos). Si el entorno se corrompe severamente, usar git-tactical-retreat.
    id: '6'
    name: Ejecución
  - description: Ejecutar validación pre-PR. Invocar git-workspace-recon para verificar la coherencia de los archivos mutados contra el plan. Generar validacion.md. Evaluación de Impacto SDDIA (innegociable) analizar el conjunto de archivos modificados; si hay cualquier mutación bajo SddIA/, exigir en el cierre fase 8 sddia_evolution_register y git-save-snapshot adicional antes de git-sync-remote.
    id: '7'
    name: Validar
  - description: Antes de git-sync-remote aplicar Evaluación de Impacto SDDIA si hay mutación en SddIA/ ejecutar obligatoriamente sddia_evolution_register y git-save-snapshot adicional consolidando el registro de evolución; luego git-sync-remote git-create-pr con objectives.md y validacion.md y actualizar Evolution Logs de producto.
    id: '8'
    name: Finalizar
principles_ref: paths.principlesPath
process_id: feature
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
  - sddia-evolution-register
spec_version: 2.1.0
---
# Proceso: Feature

Este documento define el **proceso de tarea** para desarrollar una funcionalidad. Está ubicado en paths.processPath/feature/ (Cúmulo). Las acciones que orquesta están en paths.actionsPath (Cúmulo). La ruta de persistencia se obtiene de **Cúmulo** (paths.featurePath/<nombre_feature>).

**Interfaz de proceso:** Cumple la interfaz en Cúmulo (`process_interface`): solicita/genera en la carpeta de la tarea (Cúmulo) un **`.md` por acción** con **YAML Frontmatter** (objectives.md, spec.md, clarify.md, plan.md, implementation.md, execution.md, validacion.md, finalize-process.md). No ficheros .json separados. Norma: SddIA/norms/features-documentation-frontmatter.md.

## Propósito

El proceso **feature** define el procedimiento formal de ciclo completo para desarrollar una funcionalidad o tarea: desde la creación de la rama hasta el cierre y la apertura del Pull Request. Orquesta las acciones **spec**, **clarify**, **planning**, **implementation**, **execution**, **validate** y **finalize-process** en secuencia, fija la ubicación de la documentación de la tarea y garantiza trazabilidad en los logs de evolución.

Proporciona un flujo repetible y auditado, alineado con las Leyes Universales (soberanía documental en AGENTE_CUMULO, no commits en `master`).

## Alcance del procedimiento

Ruta de la tarea: Cúmulo (paths.featurePath/<nombre_feature>).

| Fase | Nombre | Descripción |
| :--- | :--- | :--- |
| **0** | Preparar entorno | Ejecutar **git-workspace-recon** para validar entorno limpio. Tras confirmar, crear rama `feat/<nombre_feature>` o `fix/<nombre_feature>` desde `master` usando **git-branch-manager**. No trabajar en `master`. Invocar según contrato (paths.skillsDefinitionPath y paths.skillCapsules del Cúmulo). |
| **1** | Documentación con objetivos | Documentar objetivo, alcance y ley aplicada. La documentación de la tarea se ubica en la carpeta de la tarea (Cúmulo)/objectives.md. |
| **2** | Especificación | Ejecutar o generar SPEC (acción **spec**). Entrada: requerimiento o borrador, carpeta de la tarea (Cúmulo)/objectives.md; salida: especificación técnica en paths.actionsPath (spec/) y copia/canon en carpeta de la tarea (Cúmulo)/spec.md (YAML Frontmatter) |
| **3** | Clarificación | Ejecutar o generar clarificaciones (acción **clarify**). Especificación técnica: paths.actionsPath/clarify/. Entrada: carpeta de la tarea (Cúmulo)/objectives.md, spec.md; salida: carpeta de la tarea (Cúmulo)/clarify.md (YAML Frontmatter) |
| **4** | Planificación | Ejecutar o generar plan (acción **plan**). Entrada: Especificación, Clarificación. Salida: carpeta de la tarea (Cúmulo)/plan.md (YAML Frontmatter). |
| **5** | Implementación | Generar documento de implementación. Especificación técnica: paths.actionsPath/implementation/. Entrada: carpeta de la tarea (Cúmulo)/objectives.md, spec.md, clarify.md; salida: carpeta de la tarea (Cúmulo)/implementation.md (YAML Frontmatter) |
| **6** | Ejecución | Aplicar el plan al código (Tekton Developer). Especificación técnica: paths.actionsPath/execution/. Entrada: carpeta de la tarea (Cúmulo)/implementation.md; salida: carpeta de la tarea (Cúmulo)/execution.md (YAML Frontmatter). Consolidar hitos con **git-save-snapshot** (commits atómicos). Si el entorno se corrompe severamente, **git-tactical-retreat**. |
| **7** | Validar | Ejecutar validación pre-PR. Especificación técnica: paths.actionsPath/validate/. Invocar **git-workspace-recon** para verificar la coherencia de los archivos mutados contra el plan. Entrada: carpeta de la tarea (Cúmulo); salida: carpeta de la tarea (Cúmulo)/validacion.md (YAML Frontmatter). **Evaluación de Impacto SDDIA (innegociable):** analizar los archivos modificados; si existe **cualquier** mutación bajo el directorio **SddIA/**, el cierre (fase 8) debe incluir **sddia_evolution_register** y un **git-save-snapshot** adicional antes de **git-sync-remote** (debe quedar reflejado en validacion.md o en el checklist de cierre). |
| **8** | Finalizar | Cierre del ciclo. Especificación técnica: paths.actionsPath/finalize-process/. **Antes de git-sync-remote:** si el árbol de trabajo o la validación incluye mutaciones en **SddIA/**, es **obligatorio** ejecutar **sddia_evolution_register** y un **git-save-snapshot** adicional que consolide el registro de evolución SddIA; solo entonces **git-sync-remote** (subida al Leviatán) y **git-create-pr** inyectando objectives.md y validacion.md en el cuerpo del Pull Request. Actualizar Evolution Logs de producto (paths.evolutionPath). |

## Implementación

Este proceso se implementa como **procedimiento** que combina:

*   **Skills y agentes** para spec, clarify y plan (cuando se requiera trazabilidad con token de auditor), según las acciones definidas en paths.actionsPath (Cúmulo).
*   **Ubicación obligatoria de la documentación de la tarea:** paths.featurePath/<nombre_feature>/ (Cúmulo).

### Contenido mínimo de la carpeta de la tarea (Cúmulo: paths.featurePath/<nombre_feature>/)

| Documento | Contenido |
| :--- | :--- |
| **objectives.md** | Objetivo, alcance, ley aplicada (YAML Frontmatter + contenido MD). |
| **spec.md** | Especificación técnica (YAML Frontmatter + contenido MD). |
| **clarify.md** | Clarificaciones y decisiones (YAML Frontmatter + contenido MD). |
| **plan.md** | Plan de implementación / task roadmap (YAML Frontmatter + contenido MD). |
| **implementation.md** | Touchpoints y ítems de implementación (YAML Frontmatter + contenido MD). |
| **execution.md** | Registro de ítems aplicados (YAML Frontmatter + contenido MD). |
| **validacion.md** | Resultado de validación pre-PR (YAML Frontmatter + contenido MD). |
| **finalize-process.md** | Cierre, PR, Evolution Logs (YAML Frontmatter + contenido MD). |

### Actualización de Evolution Logs

Al cierre de la feature (fase 8):

*   **paths.evolutionPath + paths.evolutionLogFile:** Añadir una línea con formato `[YYYY-MM-DD] [feat/<nombre>] [Descripción breve del resultado.] [Estado].`
*   **paths.evolutionPath + paths.evolutionLogFile:** Añadir una sección con fecha y título de la feature, resumen y referencia a la carpeta de la tarea (Cúmulo)/objectives.md.

### Evaluación de Impacto SDDIA (innegociable)

**Norma canónica:** `SddIA/norms/sddia-evolution-sync.md`. Herramienta estándar: binario **`sddia_evolution_register`** (skill **sddia-evolution-register**, cápsula en Cúmulo: paths.skillCapsules).

Antes de ejecutar **git-sync-remote**, el ejecutor del proceso debe **analizar los archivos modificados** (respecto a la base acordada de la rama, p. ej. salida de **git-workspace-recon** o inventario explícito en validacion.md). Si existe **cualquier** mutación dentro del directorio **SddIA/**:

1. Es **obligatorio** ejecutar **`sddia_evolution_register`** según el contrato de evolución (paths.sddiaEvolutionContractFile, paths.sddiaEvolutionPath vía Cúmulo).
2. Es **obligatorio** ejecutar un **`git-save-snapshot`** **adicional** que incluya los artefactos generados o actualizados por ese registro, **antes** de **git-sync-remote** y **git-create-pr**.

Sin este doble paso (registro + snapshot), el PR no cumple el cierre S+ para cambios bajo **SddIA/**. La trazabilidad en **paths.evolutionPath** (evolución de producto) es independiente del registro de evolución SddIA bajo **paths.sddiaEvolutionPath**.

## Integración con Agentes

*   **Arquitecto / Spec Architect:** Puede iniciar el procedimiento y asegurar que la fase 1 y la ubicación paths.featurePath/<nombre_feature>/ (Cúmulo) se respeten.
*   **Clarifier:** Responsable de la fase 3 (clarificación) y de persistir decisiones en el SPEC y en la carpeta de la feature.
*   **Tekton Developer:** Ejecuta las fases 4 (plan), 5 (implementación), 6 (ejecución), 7 (validación) y 8 (cierre/PR); aplica la SPEC como marco legal.
*   **Cúmulo:** Valida que la documentación de la tarea esté en paths.featurePath/<nombre_feature>/ como SSOT para esa feature.

## Dependencias con otras acciones

*   El proceso **feature** invoca o utiliza los resultados de las acciones **spec**, **clarify**, **plan**, **implementation**, **execution**, **validate** y **finalize-process** en paths.actionsPath (Cúmulo).
*   La **documentación de la tarea** (objetivo, spec, clarifications, plan, validacion) debe residir en **paths.featurePath/<nombre_feature>/** (Cúmulo) para aprobación y revisión humana.

## Estándares de Calidad

*   **Grado S+:** Trazabilidad desde el objetivo hasta el PR: rama → paths.featurePath → spec/clarify/plan → implementación → execution → validación → Evolution Logs → PR.
*   **Evaluación de Impacto SDDIA:** Cambios bajo **SddIA/** exigen **sddia_evolution_register** + **git-save-snapshot** adicional **antes** de **git-sync-remote** (fases 7–8); ver subsección dedicada.
*   **Arsenal Táctico Git (Grado S+):** Máquina de estados operativa con **git-workspace-recon**, **git-branch-manager**, **git-save-snapshot**, **git-sync-remote**, **git-tactical-retreat** y **git-create-pr** (definición y cápsulas en Cúmulo: paths.skillsDefinitionPath, paths.skillCapsules).
*   **Ley GIT:** Ningún commit en `master`; todo el trabajo en rama `feat/` o `fix/` con documentación en paths.featurePath/<nombre_feature>/ (Cúmulo).
*   **Single Source of Truth:** Para cada feature, la documentación canónica de la tarea es paths.featurePath/<nombre_feature>/ (Cúmulo); la referencia en PR y en Evolution Log es esa ruta.

## Alcance para Fix (bug)

El mismo patrón de persistencia se aplica a correcciones de bugs mediante el proceso **bug-fix**. La ubicación de la documentación se obtiene del agente Cúmulo (paths.fixPath/<nombre_fix>). Ver paths.processPath/bug-fix/.

## Referencia de ejecución

Procedimiento aplicado en la rama **feat/e2e-product-back-mocked** (2026-02-10). Documentación de la tarea: paths.featurePath/<nombre_feature>/. Acciones relacionadas: paths.actionsPath (spec/, clarify/, planning/, execution/, validate/, finalize-process/).
