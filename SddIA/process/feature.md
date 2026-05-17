---
uuid: 1b4fa69f-4299-47ca-b2ed-380f2263239c
name: feature
version: 1.2.0
contract: process-contract v1.3.0
context:
- ecosystem-evolution
- filesystem-ops
- source-control
hash_signature: sha256:189d4fde0fcd0efc6cb2e7da052dd9cad86c07a159b70012c3ece87b0100caaf
inputs:
- feature_name: Nombre kebab-case o etiqueta humana de la feature
- refined_requirements: Requisitos crudos o semi-refinados de entrada
- cumulo_topology: Topología SSOT inyectada (paths, contratos, directorios)
- active_norm_pack: Paquete normativo activo para gobernar las salidas documentales
- persist_ref: Ruta lógica resuelta vía cumulo_topology
- base_branch: Rama de origen (habitualmente master o main)
- branch_name: Nombre de la nueva rama git a crear para la entrega
outputs:
- delivery_artifacts: Artefactos de código y la cascada documental completa
- verification_report: Resultado agregado de Argos (contenido o path de validacion.md)
- pr_url: URL de PR tras cierre (propagado vía delivery-close-cycle)
phases:
- name: Inicialización de Espacio de Trabajo
  intent: Sincronización determinista (fetch), checkout a base_branch y creación/aislamiento de la nueva rama branch_name.
  delegates_to:
  - skill:git-manager
- name: Estabilización de Requisitos
  intent: Mayeuta debe mapear sus salidas lógicas a la norma features-documentation-pattern del active_norm_pack. El transcript se persiste como clarify.md y el requisito termodinámico como objectives.md (con frontmatter obligatorio) bajo persist_ref.
  delegates_to:
  - agent:mayeuta
- name: Diseño de Blueprint
  intent: 'Dedalo consumirá el cuerpo de objectives.md como refined_requirements. Sus salidas se mapearán a la norma: la especificación técnica será spec.md y el blueprint de proceso será plan.md bajo persist_ref.'
  delegates_to:
  - agent:dedalo
- name: Ejecución
  intent: Tekton materializa los cambios. Debe generar obligatoriamente implementation.md y execution.md con frontmatter válido según el patrón documental activo en persist_ref.
  delegates_to:
  - agent:tekton
- name: Verificación
  intent: 'Argos audita la entrega. Su output (audit_report_md) se mapea unívocamente a validacion.md en persist_ref, inyectando el frontmatter exigido (branch, global, checks, git_changes).'
  delegates_to:
  - agent:argos
- name: Cierre de entrega
  intent: Consolidación final, impacto SddIA y apertura de PR. Se delega en action:execute-process inyectando el process_name canónico delivery-close-cycle junto con source_process, persist_ref y branch_name.
  delegates_to:
  - action:execute-process
minteo_maximo: null
porcentaje_de_exito: null
---

# feature

Proceso V5 para desarrollo de **features**: cadena **Inicialización → Mayeuta → Dedalo → Tekton → Argos → delivery-close-cycle** (`source_process: feature`). Sustituye el linaje legacy purgado y actúa como el puente ontológico que obliga a los agentes V5 a cumplir con la aduana documental de `features-documentation-pattern`.

## Puente Documental y Frontmatter

Los agentes V5 producen salidas *lógicas*. Este proceso impone un mapeo estricto: cuando el `active_norm_pack` incluya `features-documentation-pattern`, las salidas lógicas se materializarán con los nombres de archivo (`spec.md`, `validacion.md`, etc.) dentro de `persist_ref` (ruta resuelta siempre vía `cumulo_topology`, prohibido inferir rutas estáticas del host).

Todos los artefactos `.md` generados bajo esta directriz deberán incluir obligatoriamente el bloque de frontmatter dictado por la norma, inyectando de forma explícita los parámetros `feature_name`, `process: feature` y `created` (en formato ISO).

## Handoff en Runtime y Reglas de Orquestación

1. **Inicialización Git:** El orquestador inyectará incondicionalmente la variable `repository_path` (ruta absoluta resuelta por Cúmulo) en el `stdin` de `skill:git-manager` para asegurar el cumplimiento de su contrato congelado.
2. **Precedencia de Entrada:** El input `refined_requirements` (semilla cruda) es consumido exclusivamente en la fase de Estabilización.
3. **Mapeo de Contextos (Dedalo y Tekton):** Tras la estabilización, el runtime extraerá el contenido físico (cuerpo Markdown) de `objectives.md` para inyectarlo como la variable `refined_requirements` hacia Dedalo. Adicionalmente, el runtime inyectará `target_executor_rbac` para Dedalo y mapeará `active_norm_pack` al input `active_norms` exigido por el contrato de Tekton.
4. **Ejecución y Verificación:** Los documentos `clarify.md`, `spec.md` y `plan.md` actúan como contexto de solo lectura para la fase de Ejecución. Posteriormente, Argos derivará sus `acceptance_criteria` consumiendo el contexto unificado de `objectives.md` y `spec.md`.

## Cierre y Sellado

La última fase invoca la acción `execute-process` apuntando al `process_name` canónico `delivery-close-cycle`, inyectando sus `process_inputs` correspondientes (`persist_ref`, `branch_name` y `source_process: feature`). Este subproceso asume el snapshot final, la evaluación de impacto en el Core SddIA y la apertura de la Pull Request, propagando el `pr_url` hacia los outputs de este proceso padre.

*Nota de Arquitectura EDA:* La emisión del Sello Criptográfico (`PullRequest_Merged`) es un evento asíncrono gestionado por el bus de eventos de dominio post-fusión, estrictamente desvinculado de la apertura de este PR. La escritura eventual de `finalize-process.md` recae en la evolución del subproceso de cierre.
