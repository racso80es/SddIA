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
  - description: Ejecutar validación pre-PR. Invocar git-workspace-recon para verificar la coherencia de los archivos mutados contra el plan. Generar validacion.md. Dejar constancia del conjunto de rutas mutadas (incl. bajo ./SddIA/) para la Evaluación de Impacto SDDIA obligatoria en fase 8 antes de git-sync-remote.
    id: '7'
    name: Validar
  - description: 'Evaluación de Impacto SDDIA (innegociable): antes de git-sync-remote, analizar archivos modificados; si existe cualquier mutación bajo SddIA/, ejecutar sddia_evolution_register (cápsula paths.skillCapsules.sddia-evolution-register) y un git-save-snapshot adicional que consolide el registro de evolución. Después: git-sync-remote, git-create-pr con objectives.md y validacion.md; paso final git-close-cycle (rama de trabajo). Evolution Logs según contrato.'
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
  - git-close-cycle
  - sddia-evolution-register
spec_version: 2.0.0
---
# Proceso: Feature (spec_version 2.0.0)

Este documento define el **proceso de tarea** para desarrollar una funcionalidad (**spec_version 2.0.0**), integrando el **Arsenal Táctico Git (grado S+)**: máquina de estados basada en `git-workspace-recon`, `git-branch-manager`, `git-save-snapshot`, `git-sync-remote`, `git-tactical-retreat`, `git-create-pr`, `git-close-cycle` y, cuando muta `./SddIA/`, `sddia_evolution_register` vía cápsula `sddia-evolution-register` (contrato en paths.skillsDefinitionPath y cápsulas en paths.skillCapsules, Cúmulo). Está ubicado en paths.processPath/feature/ (Cúmulo). Las acciones que orquesta están en paths.actionsPath (Cúmulo). La ruta de persistencia se obtiene de **Cúmulo** (paths.featurePath/<nombre_feature>).

**Interfaz de proceso:** Cumple la interfaz en Cúmulo (`process_interface`): solicita/genera en la carpeta de la tarea (Cúmulo) un **`.md` por acción** con **YAML Frontmatter** (objectives.md, spec.md, clarify.md, plan.md, implementation.md, execution.md, validacion.md, finalize-process.md). No ficheros .json separados. Norma: SddIA/norms/features-documentation-frontmatter.md.

## Propósito

El proceso **feature** define el procedimiento formal de ciclo completo para desarrollar una funcionalidad o tarea: desde la preparación del entorno y la rama hasta el cierre y la apertura del Pull Request. Orquesta las acciones **spec**, **clarify**, **planning**, **implementation**, **execution**, **validate** y **finalize-process** en secuencia, fija la ubicación de la documentación de la tarea y garantiza trazabilidad en los logs de evolución.

Proporciona un flujo repetible y auditado, alineado con las Leyes Universales (soberanía documental en AGENTE_CUMULO, prohibición estricta de commits directos en `master`). Las transiciones Git del ciclo se gobiernan por el Arsenal Táctico (reconocimiento de workspace, ramificación, snapshots atómicos, sincronización remota, retirada táctica y creación de PR).

## Alcance del procedimiento

Ruta de la tarea: Cúmulo (paths.featurePath/<nombre_feature>).

| Fase | Nombre | Descripción |
| :--- | :--- | :--- |
| **0** | Preparar entorno | **git-workspace-recon:** validar entorno limpio y coherente antes de ramificar. Tras confirmar, **git-branch-manager:** crear rama `feat/<nombre_feature>` o `fix/<nombre>` desde `master` actualizado. No trabajar en `master`. Invocación según contrato (paths.skillsDefinitionPath, paths.skillCapsules, Cúmulo). |
| **1** | Documentación con objetivos | Documentar objetivo, alcance y ley aplicada. La documentación de la tarea se ubica en la carpeta de la tarea (Cúmulo)/objectives.md. |
| **2** | Especificación | Ejecutar o generar SPEC (acción **spec**). Entrada: requerimiento o borrador, carpeta de la tarea (Cúmulo)/objectives.md; salida: especificación técnica en paths.actionsPath (spec/) y copia/canon en carpeta de la tarea (Cúmulo)/spec.md (YAML Frontmatter) |
| **3** | Clarificación | Ejecutar o generar clarificaciones (acción **clarify**). Especificación técnica: paths.actionsPath/clarify/. Entrada: carpeta de la tarea (Cúmulo)/objectives.md, spec.md; salida: carpeta de la tarea (Cúmulo)/clarify.md (YAML Frontmatter) |
| **4** | Planificación | Ejecutar o generar plan (acción **planning**). Entrada: Especificación, Clarificación. Salida: carpeta de la tarea (Cúmulo)/plan.md (YAML Frontmatter). |
| **5** | Implementación | Generar documento de implementación. Especificación técnica: paths.actionsPath/implementation/. Entrada: carpeta de la tarea (Cúmulo)/objectives.md, spec.md, clarify.md; salida: carpeta de la tarea (Cúmulo)/implementation.md (YAML Frontmatter) |
| **6** | Ejecución | Aplicar el plan al código (Tekton Developer). Especificación técnica: paths.actionsPath/execution/. Entrada: carpeta de la tarea (Cúmulo)/implementation.md; salida: carpeta de la tarea (Cúmulo)/execution.md (YAML Frontmatter). **git-save-snapshot:** consolidar hitos con commits atómicos. Ante corrupción severa del entorno: **git-tactical-retreat** (solo con confirmación explícita según norma del proyecto). |
| **7** | Validar | Ejecutar validación pre-PR (acción **validate**). **git-workspace-recon:** verificar coherencia de archivos mutados frente al plan. Salida: carpeta de la tarea (Cúmulo)/validacion.md (YAML Frontmatter). Dejar trazado el inventario de rutas mutadas (incluye comprobar si hay cambios bajo `./SddIA/`) como insumo obligatorio para la fase 8. Especificación técnica: paths.actionsPath/validate/. |
| **8** | Finalizar | **Evaluación de Impacto SDDIA (innegociable):** antes de **git-sync-remote**, analizar archivos modificados del workspace. Si **existe cualquier mutación** bajo el directorio **`SddIA/`**, es **obligatorio** ejecutar **`sddia_evolution_register`** (binario Rust / cápsula **paths.skillCapsules.sddia-evolution-register**) y realizar un **git-save-snapshot adicional** que consolide el registro de evolución; solo entonces proceder con **git-sync-remote** y **git-create-pr** (cuerpo del PR con objectives.md y validacion.md). **Paso final:** **git-close-cycle** con la rama de trabajo de la tarea (`targetBranch`), según paths.actionsPath/finalize-process/. Actualizar Evolution Logs según contrato. Especificación técnica: paths.actionsPath/finalize-process/. Entrada: carpeta de la tarea (Cúmulo); salida: finalize-process.md, logs de evolución y PR. |

### Evaluación de Impacto SDDIA (innegociable, fases 7–8)

Norma operativa de cierre:

1. **Análisis previo a sync:** identificar todas las rutas mutadas respecto a la base de la rama (mismo criterio que alimenta validación y recon).
2. **Si alguna ruta cae bajo `SddIA/`** (cualquier archivo o directorio dentro de ese árbol): no invocar **git-sync-remote** hasta haber ejecutado **`sddia_evolution_register`** conforme a `SddIA/norms/sddia-evolution-sync.md` y la cápsula del Cúmulo, y hasta haber consolidado esos artefactos con un **git-save-snapshot** dedicado (commit atómico adicional).
3. **Orden canónico de cierre con mutación SddIA:** `sddia_evolution_register` → `git-save-snapshot` (consolidación) → `git-sync-remote` → `git-create-pr` → `git-close-cycle` (rama de trabajo).

Sin cumplimiento de los puntos 1–2 cuando aplique mutación en `SddIA/`, el proceso **no** está cerrado en grado S+.

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

*   Si hubo mutación bajo **`SddIA/`**, el registro detallado y la consolidación en commit **preceden** a **git-sync-remote** (ver «Evaluación de Impacto SDDIA» arriba y norma `SddIA/norms/sddia-evolution-sync.md`).
*   **paths.evolutionPath + paths.evolutionLogFile:** Añadir una línea con formato `[YYYY-MM-DD] [feat/<nombre>] [Descripción breve del resultado.] [Estado].`
*   **paths.evolutionPath + paths.evolutionLogFile:** Añadir una sección con fecha y título de la feature, resumen y referencia a la carpeta de la tarea (Cúmulo)/objectives.md.

## Integración con Agentes

*   **Arquitecto / Spec Architect:** Puede iniciar el procedimiento y asegurar que la fase 1 y la ubicación paths.featurePath/<nombre_feature>/ (Cúmulo) se respeten.
*   **Clarifier:** Responsable de la fase 3 (clarificación) y de persistir decisiones en el SPEC y en la carpeta de la feature.
*   **Tekton Developer:** Ejecuta las fases 4 (plan), 5 (implementación), 6 (ejecución con snapshots Git), 7 (validación con recon de workspace) y 8 (Evaluación de Impacto SDDIA si muta `SddIA/`, sync remoto, PR y **git-close-cycle**); aplica la SPEC como marco legal.
*   **Cúmulo:** Valida que la documentación de la tarea esté en paths.featurePath/<nombre_feature>/ como SSOT para esa feature.

## Dependencias con otras acciones

*   El proceso **feature** invoca o utiliza los resultados de las acciones **spec**, **clarify**, **plan**, **implementation**, **execution**, **validate** y **finalize-process** en paths.actionsPath (Cúmulo).
*   La **documentación de la tarea** (objetivo, spec, clarifications, plan, validacion) debe residir en **paths.featurePath/<nombre_feature>/** (Cúmulo) para aprobación y revisión humana.

## Estándares de Calidad

*   **Grado S+:** Trazabilidad desde el objetivo hasta el PR: git-workspace-recon → rama (git-branch-manager) → paths.featurePath → spec/clarify/plan → implementación → execution (git-save-snapshot / git-tactical-retreat si aplica) → validación (git-workspace-recon) → **[si mutación en `SddIA/`: sddia_evolution_register → git-save-snapshot adicional]** → git-sync-remote → git-create-pr → git-close-cycle → Evolution Logs.
*   **Ley GIT:** Ningún commit directo en `master`; todo el trabajo en rama `feat/` o `fix/` con documentación en paths.featurePath/<nombre_feature>/ (Cúmulo).
*   **Single Source of Truth:** Para cada feature, la documentación canónica de la tarea es paths.featurePath/<nombre_feature>/ (Cúmulo); la referencia en PR y en Evolution Log es esa ruta.

## Alcance para Fix (bug)

El mismo patrón de persistencia se aplica a correcciones de bugs mediante el proceso **bug-fix**. La ubicación de la documentación se obtiene del agente Cúmulo (paths.fixPath/<nombre_fix>). Ver paths.processPath/bug-fix/.

## Referencia de ejecución

Procedimiento aplicado en la rama **feat/e2e-product-back-mocked** (2026-02-10). Documentación de la tarea: paths.featurePath/<nombre_feature>/. Acciones relacionadas: paths.actionsPath (spec/, clarify/, planning/, execution/, validate/, finalize-process/).
