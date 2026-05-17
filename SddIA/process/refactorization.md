---
uuid: ae01e3ff-af68-4b94-90b3-97e5c03d75ee
name: refactorization
version: 1.2.0
contract: process-contract v1.3.0
context:
- ecosystem-evolution
- filesystem-ops
- source-control
hash_signature: sha256:5417bf69d8e1011536a3cb69edde6e8fd4314c6f5a4a128417329e84817f675b
inputs:
- refactor_goal: Propósito central de la refactorización
- refined_constraints: Restricciones de alcance complementarias y límites
- cumulo_topology: Topología SSOT inyectada (paths, contratos, directorios)
- active_norm_pack: Paquete normativo activo para gobernar las salidas documentales
- persist_ref: Ruta lógica resuelta vía cumulo_topology
- base_branch: Rama de origen (habitualmente master o main)
- branch_name: Nombre de la nueva rama git a crear para la entrega
outputs:
- refactor_artifacts: Artefactos de código y la cascada documental completa
- verification_report: Resultado agregado de Argos (contenido o path de validacion.md)
- pr_url: URL de PR tras cierre (propagado vía delivery-close-cycle)
phases:
- name: Inicialización de Espacio de Trabajo
  intent: 'Sincronización determinista (fetch), checkout a base_branch y creación/aislamiento de la nueva rama branch_name.'
  delegates_to:
  - skill:git-manager
- name: Estabilización de alcance
  intent: 'Mayeuta debe mapear sus salidas lógicas a la norma features-documentation-pattern del active_norm_pack. El transcript se persiste como clarify.md y el requisito termodinámico como objectives.md (con frontmatter obligatorio) bajo persist_ref.'
  delegates_to:
  - agent:mayeuta
- name: Diseño de refactor
  intent: 'Dedalo consumirá el cuerpo de objectives.md como refined_requirements. Sus salidas se mapearán a la norma: la especificación técnica será spec.md y el blueprint de proceso será plan.md bajo persist_ref.'
  delegates_to:
  - agent:dedalo
- name: Ejecución
  intent: 'Tekton materializa los cambios. Debe generar obligatoriamente implementation.md y execution.md con frontmatter válido según el patrón documental activo en persist_ref.'
  delegates_to:
  - agent:tekton
- name: Verificación
  intent: 'Argos audita la entrega. Su output (audit_report_md) se mapea unívocamente a validacion.md en persist_ref, inyectando el frontmatter exigido (branch, global, checks, git_changes).'
  delegates_to:
  - agent:argos
- name: Cierre de entrega
  intent: 'Consolidación final, impacto SddIA y apertura de PR. Se delega en action:execute-process inyectando el process_name canónico delivery-close-cycle junto con source_process (refactorization), persist_ref y branch_name.'
  delegates_to:
  - action:execute-process
minteo_maximo: null
porcentaje_de_exito: null
---

# refactorization

Proceso V5 para refactorización de deuda técnica y arquitectura: cadena **Inicialización → Estabilización de alcance → Diseño de refactor → Ejecución → Verificación → delivery-close-cycle**. Este flujo opera bajo el mismo puente ontológico documental que `feature`, obligando a los agentes a cumplir con la aduana de `features-documentation-pattern`.

## Puente Documental y Frontmatter

Los agentes V5 producen salidas *lógicas*. Este proceso impone un mapeo estricto: cuando el `active_norm_pack` incluya `features-documentation-pattern`, las salidas lógicas se materializarán en los archivos físicos de la norma (`spec.md`, `validacion.md`, etc.) dentro de `persist_ref` (ruta resuelta siempre vía `cumulo_topology`). Todos los artefactos `.md` generados deberán incluir obligatoriamente el bloque de frontmatter dictado por la norma, inyectando los parámetros de etiqueta de tarea en el campo `feature_name`, `process: refactorization` y `created` (formato ISO).

## Handoff en Runtime y Reglas de Orquestación

1. **Inicialización Git:** El orquestador inyectará incondicionalmente la variable `repository_path` en el `stdin` de `skill:git-manager`.
2. **Precedencia de Entrada:** Las semillas `refactor_goal` y `refined_constraints` son consumidas exclusivamente en la fase de Estabilización de alcance.
3. **Mapeo de Contextos (Dedalo y Tekton):** Tras la estabilización, el runtime extraerá el contenido físico de `objectives.md` para inyectarlo como variable central hacia Dedalo. Adicionalmente, se inyectará `target_executor_rbac` para Dedalo y se mapeará `active_norm_pack` al input `active_norms` exigido por el contrato de Tekton.
4. **Ejecución y Verificación:** `clarify.md`, `spec.md` y `plan.md` actúan como contexto de solo lectura para Tekton. Posteriormente, Argos derivará sus `acceptance_criteria` consumiendo el contexto unificado de `objectives.md` y `spec.md`.

## Cierre y Sellado

La última fase invoca la acción `execute-process` apuntando a `delivery-close-cycle`, inyectando sus `process_inputs` correspondientes (`persist_ref`, `branch_name` y `source_process: refactorization`). El subproceso asume el snapshot final y la propagación del `pr_url`.

*Nota de Arquitectura EDA:* La emisión del Sello Criptográfico (`PullRequest_Merged`) pertenece a la jurisdicción del bus de eventos post-fusión, estrictamente desvinculado de la apertura de este PR. La escritura eventual de `finalize-process.md` recae en la evolución del subproceso de cierre o en el runtime padre.
