---
uuid: ac8d078c-9785-490b-9f43-ad310fe9df9d
name: bug-fix
version: 1.4.0
contract: process-contract v1.4.0
workspace_template: ".SddIA/workspaces/{process_name}/{execution_id}/"
context:
- ecosystem-evolution
- filesystem-ops
- source-control
hash_signature: sha256:d143033143044a5595d8946d4146c5eb2b04bd09ca32d1be82264f0b2dd2f8c3
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
- name: Cierre documental en rama
  intent: 'Antes del merge: mover PBI a docs/todos/done/ en la rama del PR; validacion.md con pbi_archived true según features-documentation-pattern v1.2.0 (sin merged_pr obligatorio).'
  delegates_to:
  - skill:filesystem-manager
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

## Cierre documental en rama (obligatorio, pre-merge)

**Antes** de `delivery-close-cycle` y del merge en `main`, en la **misma rama** del fix:

| Paso | Acción | Artefacto |
|------|--------|-----------|
| 1 | Archivar PBI | `docs/todos/pending/` → `docs/todos/done/` (conservar `document_id`) |
| 2 | Cerrar validación Argos | `{persist_ref}/validacion.md` → `global: APTO`, `pbi_archived: true` |
| 3 | Incluir en el PR único | Diff de código + documentación + PBI en `done/` |

**Definición operativa de Done:**

```text
Done = un único PR mergeado en main
     + validacion.md APTO en ese PR (pbi_archived: true)
     + PBI en docs/todos/done/ en esa misma rama
```

Prohibido depender de un segundo PR `docs/cerrar-pbi-*` para campos que solo existen tras el merge (`merged_pr`, `merge_commit`). La trazabilidad del merge se obtiene de GitHub / git.

*Referencia normativa:* `features-documentation-pattern` v1.2.0 § Validación en fase única; regla Cursor `task-closure-documental`.

## Perfil laboratorio vs runtime IDE

| Aspecto | Laboratorio (`execute-process` nativo) | Runtime IDE completo |
| :--- | :--- | :--- |
| Fase 1 Inicialización | `workspace-init` físico (git-manager + `objectives.md` mínimo) | Igual; `persist_ref` bajo `docs/fixes/` si rama `fix/` o `source_process: bug-fix` |
| Fases 2–4 (Dedalo…Argos) | `simulated` / agentes IDE | Agentes V5; cascada mínima `spec.md` + `implementation.md` + `execution.md` + `validacion.md` |
| Fase 5 Cierre documental en rama | Manual / operador IA en rama `fix/*` | PBI en `done/` + `validacion.md` pre-merge |
| Fase 6 Cierre de entrega | Delega en `delivery-close-cycle` con `source_process: bug-fix` | Orquestador inyecta `pr_title`, `pr_body` |
| Inputs workspace-init | `branch_name` + `persist_ref` + (`bug_summary` \| `fix_name`); no exige `feature_name` | Misma regla en handler nativo `workspace_init` |

**Contrato git-manager fetch:** toda invocación `fetch` debe incluir `prune` (boolean) según `skill-io-git-manager-frozen.md` §3.7.
