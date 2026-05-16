---
uuid: bd81c878-5e1c-4fe5-b204-85c9136d8bc7
name: accept-pr
version: 1.0.0
contract: process-contract v1.3.0
context:
- pr-lifecycle
- source-control
- filesystem-ops
hash_signature: sha256:6b324b6381a78b2425a017d570068a97ffb1d7382bf17dd784dcfe563d55dbb4
inputs:
- source_branch: Rama feature a fusionar hacia main (validada por git-operations antes de invocar)
- author: Autor del merge para el evento PullRequest_Merged
- correlation_id: UUID v4 de correlación causal (Sagas)
- cumulo_topology: Topología SSOT inyectada (paths, contratos, repository_path)
outputs:
- verdict: aprobado | abortado
- merge_commit_hash: Hash de 40 caracteres hex en main tras fusión soberana
- event_id: UUID v4 del evento emitido en pending/
- target_path: Ruta relativa del JSON en .SddIA/events/pending/
- closed_branch: Rama origen eliminada (local y remoto si aplica)
phases:
- name: Auditoría Genómica
  intent: Argos evalúa la rama origen. Si detecta fricción letal o vulnerabilidad, aborta
    el proceso.
  delegates_to:
  - agent:argos
- name: Fusión Soberana
  intent: Hacer checkout a la rama main y ejecutar el merge de la rama origen.
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

1. `skill:git-manager` → `push` (`remote: origin`, `branch: main`, `force: false`).
2. Eliminar rama origen local: `git branch -d` vía secuencia acordada en evolución de `git-manager` o higiene documentada en `git-operations` (hasta existir `delete_branch` congelado).
3. Eliminar rama origen remota: `push` con refdelete o `skill:shell-executor` + `gh` según `pull-request-orchestration.md` si la política del repo lo exige.

## Notas

* `context: pr-lifecycle` debe registrarse en `execution-contexts.md` y políticas del invocante.
* Prohibido `gh pr merge` como sustituto del merge soberano local en este proceso salvo evolución explícita de la norma.
