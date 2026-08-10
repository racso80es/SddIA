---
uuid: bd81c878-5e1c-4fe5-b204-85c9136d8bc7
name: accept-pr
version: 1.0.1
contract: process-contract v1.4.0
workspace_template: ".SddIA/workspaces/{process_name}/{execution_id}/"
context:
- pr-lifecycle
- source-control
- filesystem-ops
hash_signature: sha256:f7a6ef21856490f3ac9b69133f948dcf54066d3a05fa7c5dc1019fdd3a289938
inputs:
- source_branch: Rama feature a fusionar hacia main (validada por git-operations antes de invocar)
- author: Autor del merge para el evento PullRequest_Merged
- correlation_id: UUID v4 de correlación causal (Sagas)
- cumulo_topology: Topología SSOT inyectada (paths, contratos, repository_path)
outputs:
- verdict: aprobado | abortado
- merge_commit_hash: Hash de 40 caracteres hex en main tras fusión soberana
- event_id: UUID v4 del evento emitido en pending/
- target_path: Ruta relativa del JSON padre en eda_bus.pending (`.events/pending/`)
- closed_branch: Rama origen eliminada (local y remoto si aplica)
- hygiene_failure: Detalle empírico si la rama sobrevivió tras intento de delete (Fase 4)
phases:
- name: Auditoría Genómica
  intent: Argos evalúa la rama origen. Si detecta fricción letal o vulnerabilidad, aborta
    el proceso.
  delegates_to:
  - agent:argos
- name: Fusión Soberana
  intent: Hacer checkout a la rama main y ejecutar el merge de la rama origen.
  requires_capability:
  - id: proc:git-sync
    contract: proc.git_sync
    version: '>=1.0.0'
  delegates_to:
  - skill:git-manager
- name: Sello Criptográfico de Fusión
  intent: Emitir el evento inmutable PullRequest_Merged en el bus local.
  delegates_to:
  - action:emit-pr-merged-event
- name: Sincronización y Limpieza
  intent: Hacer push de main al repositorio remoto y eliminar la rama de origen tanto
    en local como en remoto.
  delegates_to:
  - skill:git-manager
minteo_maximo: null
porcentaje_de_exito: null
---

# accept-pr

Proceso de **aceptación local soberana** de Pull Requests: auditoría Argos, merge determinista hacia `main`, sello **PullRequest_Merged** en el bus local y sincronización remota con higiene de ramas.

## Fase 1 — Auditoría Genómica

Tekton presenta a **Argos** la rama `source_branch`, diff y normas activas (`pr-acceptance-protocol`, `git-operations`). Si el veredicto es rechazo o fricción letal, `verdict: abortado` y `status_code: 1` sin mutar `main`.

## Fase 2 — Fusión Soberana

1. `skill:git-manager` → `checkout` sobre `main` (`create_if_not_exists: false`).
2. `skill:git-manager` → `merge` con `operation_payload_json`: `{ "branch_name": "<source_branch>", "no_ff": true }` (requiere norma congelada y cápsula con operación `merge`).
3. Persistir `merge_commit_hash` desde salida de merge o `get_last_commit` con `ref: HEAD`.

## Fase 3 — Sello Criptográfico de Fusión

Invocar **`action:emit-pr-merged-event`** con:

| Input | Origen |
| :--- | :--- |
| `source_branch` | input del proceso |
| `author` | input del proceso |
| `correlation_id` | input del proceso |

`repository_path` desde `cumulo_topology`. Salidas: `success`, `event_id`, `target_path`.

## Fase 4 — Sincronización y Limpieza

1. `skill:git-manager` → `push` (`remote: origin`, `branch: main`, `force: false`). Si falla, abortar fase sin ejecutar delete.
2. Eliminar rama origen **local**: `delete_branch` con `{ "branch_name": "<source_branch>", "remote": false, "force": false }` → `git branch -d`.
3. Eliminar rama origen **remota**: `delete_branch` con `{ "branch_name": "<source_branch>", "remote": true, "force": false }` → `git push origin --delete`.
4. `closed_branch` = nombre de rama **solo** si ambas operaciones (2 y 3) confirman éxito.
5. Si alguna operación falla: `closed_branch: null` y nodo **`hygiene_failure`** en salida de fase y `execution_report` (prohibido fallo silencioso).

### Contrato `hygiene_failure`

| Campo | Descripción |
| :--- | :--- |
| `survived_branch` | Rama que no pudo eliminarse por completo |
| `branch_deleted_local` | boolean |
| `branch_deleted_remote` | boolean |
| `operations[]` | `{ op, command, success, error? }` por operación |

Merge y push exitosos con higiene fallida: proceso `verdict: aprobado`, `status_code: 0`, con `hygiene_failure` explícito.

## Notas

* `context: pr-lifecycle` debe registrarse en `execution-contexts.md` y políticas del invocante.
* Prohibido `gh pr merge` como sustituto del merge soberano local en este proceso salvo evolución explícita de la norma.
