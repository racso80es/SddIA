---
uuid: ac8d078c-9785-490b-9f43-ad310fe9df9d
name: bug-fix
version: 1.2.0
contract: process-contract v1.3.0
context:
- ecosystem-evolution
- filesystem-ops
- source-control
hash_signature: sha256:3c2f6fd64dc559a2463d46bd371ef54417404cdab49480511e016aad7535a19c
inputs:
- bug_summary: Semilla o reporte del defecto detectado
- cumulo_topology: Topología SSOT inyectada (paths, contratos, directorios)
- active_norm_pack: Paquete normativo activo para gobernar las salidas documentales
- persist_ref: Ruta lógica resuelta vía cumulo_topology
- base_branch: Rama de origen (habitualmente master o main)
- branch_name: Nombre de la nueva rama git a crear para la entrega
outputs:
- fix_artifacts: Artefactos de código y el subconjunto documental requerido
- verification_report: Resultado agregado de Argos (contenido o path de validacion.md)
- pr_url: URL de PR tras cierre (propagado vía delivery-close-cycle)
phases:
- name: Inicialización de Espacio de Trabajo
  intent: 'Sincronización determinista (fetch), checkout a base_branch y creación/aislamiento de la nueva rama branch_name.'
  delegates_to:
  - skill:git-manager
- name: Diseño del fix
  intent: 'Dedalo consumirá el cuerpo del bug_summary para emitir sus salidas lógicas bajo persist_ref. Se mapeará obligatoriamente a spec.md (y plan.md de ser emitido un blueprint de proceso).'
  delegates_to:
  - agent:dedalo
- name: Ejecución
  intent: 'Tekton materializa las correcciones. Debe generar obligatoriamente implementation.md y execution.md con frontmatter válido según el patrón documental activo en persist_ref.'
  delegates_to:
  - agent:tekton
- name: Verificación
  intent: 'Argos audita el fix. Su output (audit_report_md) se mapea unívocamente a validacion.md en persist_ref, inyectando el frontmatter exigido (branch, global, checks, git_changes).'
  delegates_to:
  - agent:argos
- name: Cierre de entrega
  intent: 'Consolidación final, impacto SddIA y apertura de PR. Se delega en action:execute-process inyectando el process_name canónico delivery-close-cycle junto con source_process (bug-fix), persist_ref y branch_name.'
  delegates_to:
  - action:execute-process
minteo_maximo: null
porcentaje_de_exito: null
---

# bug-fix

Proceso V5 para corrección de defectos: cadena **Inicialización → Diseño del fix → Ejecución → Verificación → delivery-close-cycle**. Este flujo opera bajo una asimetría táctica respecto a procesos de mayor envergadura: carece de la fase obligatoria de "Estabilización", priorizando la intervención directa y focalizada del Nodo Dedalo.

## Puente Documental y Frontmatter

Los agentes V5 producen salidas *lógicas*. Este proceso impone un mapeo estricto condicionado: cuando el `active_norm_pack` incluya `features-documentation-pattern`, la cascada documental es un **subconjunto obligatorio**: `spec.md`, `implementation.md`, `execution.md` y `validacion.md` (`plan.md` solo si Dedalo requiere blueprint).

*Excepción Analítica:* Si el triaje inicial detecta ambigüedad severa, el runtime de la jurisdicción local puede invocar opcionalmente a Mayeuta antes del Diseño, materializando `clarify.md` y `objectives.md` como salvaguarda S+ Grade.

Todos los artefactos `.md` incluirán el bloque de frontmatter de la norma, inyectando `process: bug-fix`, `created` (formato ISO), y asignando el identificador del defecto o nombre de rama al campo `feature_name` para preservar el contrato del índice.

## Handoff en Runtime y Reglas de Orquestación

1. **Inicialización Git:** El orquestador inyectará `repository_path` en el `stdin` de `skill:git-manager`.
2. **Precedencia de Entrada (Dedalo y Tekton):** La semilla `bug_summary` es consumida en la fase de Diseño del fix. Si existió una intervención opcional de Mayeuta, Dedalo leerá el cuerpo de `objectives.md` como alias para su análisis; de lo contrario, consumirá directamente el `bug_summary`. Adicionalmente, se inyectará `target_executor_rbac` para Dedalo y se mapeará `active_norm_pack` al input `active_norms` exigido por el contrato de Tekton.
3. **Ejecución y Verificación:** `spec.md` (y `plan.md` de existir) actúan como contexto de solo lectura para Tekton. Posteriormente, Argos derivará sus `acceptance_criteria` consumiendo el `spec.md` complementado con los parámetros de regresión implícitos en el `bug_summary`.

## Cierre y Sellado

La última fase invoca `execute-process` apuntando a `delivery-close-cycle` con sus `process_inputs` inyectados (`source_process: bug-fix`, `persist_ref` y `branch_name`). El subproceso asume la propagación del `pr_url`.

*Nota de Arquitectura EDA:* El Sello Criptográfico (`PullRequest_Merged`) es un evento asíncrono y desacoplado post-fusión. La escritura eventual de `finalize-process.md` queda supeditada a la evolución del subproceso de cierre.
