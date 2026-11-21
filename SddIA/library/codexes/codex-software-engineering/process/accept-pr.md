---
context:
- pr-lifecycle
- source-control
- filesystem-ops
contract: process-contract v1.4.0
hash_signature: "sha256:44223712567a5b3913ec52388224131d6ee22b292f5367ffe7ffd6c700785dde"
inputs:
- source_branch: Rama feature a fusionar hacia main (validada por git-operations antes de invocar)
- author: Autor del merge para el evento PullRequest_Merged
- correlation_id: UUID v4 de correlación causal (Sagas)
- cumulo_topology: Topología SSOT inyectada (paths, contratos, repository_path)
minteo_maximo: null
name: accept-pr
outputs:
- 'verdict: aprobado | abortado'
- 'merge_commit_hash: Hash de 40 caracteres hex en main tras fusión soberana'
- 'event_id: Token lógico EventUri resuelto por EventManagerPort (eda://{family}/{event_id})'
- 'event_uri: Token lógico EventUri resuelto por EventManagerPort (eda://{family}/{event_id})'
- 'closed_branch: Rama origen eliminada (local y remoto si aplica)'
- 'hygiene_failure: Detalle empírico si la rama sobrevivió tras intento de delete (Fase 4)'
phases:
- delegates_to:
  - agent:argos
  intent: Argos evalúa la rama origen y la aduana exige qa-attestation/1.0 PPR del mismo tree_sha. Si detecta fricción letal, atestación inválida o vulnerabilidad, aborta el proceso.
  name: Auditoría Genómica
- delegates_to:
  - skill:git-manager
  intent: Validar sello diff + coherencia APTO (apto_coherence) antes del checkout/merge soberano; abortar sin merge si falla.
  name: Fusión Soberana
  requires_capability:
  - contract: proc.git_sync
    id: proc:git-sync
    version: '>=1.0.0'
- delegates_to:
  - action:emit-pr-merged-event
  intent: Emitir el evento inmutable PullRequest_Merged en el bus local.
  name: Sello Criptográfico de Fusión
- delegates_to:
  - skill:git-manager
  intent: Hacer push de main al repositorio remoto y eliminar la rama de origen tanto en local como en remoto.
  name: Sincronización y Limpieza
porcentaje_de_exito: null
uuid: bd81c878-5e1c-4fe5-b204-85c9136d8bc7
version: 2.0.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# accept-pr

Proceso de **aceptación local soberana** de Pull Requests: auditoría Argos, merge determinista hacia `main`, sello **PullRequest_Merged** en el bus local y sincronización remota con higiene de ramas.

## Fase 1 — Auditoría Genómica

Tekton presenta a **Argos** la rama `source_branch`, diff y normas activas (`pr-acceptance-protocol`, `git-operations`). Si el veredicto es rechazo o fricción letal, `verdict: abortado` y `status_code: 1` sin mutar `main`.

**Aduana PPR (MAN-45 / KZ-2):** antes de «Fusión Soberana», el motor exige atestación `qa-attestation/1.0` con `verdict: aprobado`, `issuer_process: pull-request-review` (emitida por `try_write_from_ppr`), `tree_sha` igual al de `source_branch` (pre-merge) y `qa_profile: full` cuando `delta_class` es `active`. `PullRequest_Presented` es solo trazabilidad/`orphan_merge` y **no** habilita fusión. `skipped-ppr-dedupe` **no** cuenta como aprobación. Fallo → `abortado` con `friction_id` (`qa_attestation_required` | `qa_attestation_tree_mismatch` | `qa_attestation_profile_blocked` | `qa_attestation_issuer_invalid` | `qa_attestation_dedupe_not_approval`); **sin** `System_Fracture_Detected`. Lab: `SDDIA_LAB_SKIP_PPR_ATTESTATION_ACCEPT`.

## Fase 2 — Fusión Soberana

Antes del merge, si `{persist_ref}/validacion.md` exige sello (`global: APTO` o campos `validated_*` / `diff_digest`), `accept-pr` recalcula el digest (misma exclusión de `validacion.md` que DCC AUT-39) y aborta con `diff_seal_mismatch` si no coincide. Lab: `SDDIA_LAB_SKIP_DIFF_SEAL_ACCEPT` o `SDDIA_LAB_SKIP_DIFF_SEAL`.

**Scope del sello (AUT-53):** si el input `persist_ref` del ciclo viene informado (no vacío), la verificación se limita **solo** a ese path. No se barre el árbol buscando otros `validacion.md` bajo `docs/features|fixes|refactors` (un fixture hermano APTO sin sello no debe abortar la fusión del ciclo). Sin `persist_ref` explícito, se conserva el fallback legacy: listado de persist refs con `validacion.md` en HEAD, **excluyendo** slugs lab `_fixture-*` (AUT-54).

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

`repository_path` desde `cumulo_topology`. Salidas: `success`, `event_id`, `event_uri`.

**Milestone `ReviewCiGreen` (tracker remoto, merge externo / `merge_already_done`):** antes del hito de ciclo de vida en la misma fase, el motor enriquece una copia de inputs con el mismo criterio que el sello: `tracker_ref` desde `PullRequest_Presented` de la rama, `persist_ref` inferido desde `source_branch` (`infer_persist_ref_from_branch`) y `tracker_ref` desde `{persist_ref}/objectives.md` si aún falta. Así `apply_review_ci_fail_soft` no queda en `tracker_ref ausente` cuando el merge en GitHub ya está hecho y solo faltaba contexto en los inputs mínimos del hook/post-merge.

## Fase 4 — Sincronización y Limpieza

**Umbral físico (AUT-53):** push de `main` y delete local/remoto de la rama origen **solo** si `merge_commit_hash` quedó persistido tras Fusión Soberana exitosa. Si la fusión abortó (p. ej. `diff_seal_mismatch`) y no hay hash, la fase devuelve `skipped: true`, `reason: merge_commit_hash ausente`, sin delete remoto ni local de higiene.

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

* **Pre-fusión coherencia (MAN-44):** `validate_for_accept_pr_merge` invoca `apto_coherence::validate_apto_coherence` antes de verificar el digest. Fallo → error `apto_coherence_*` / aborto sin merge; **sin** `System_Fracture_Detected`. Lab: `SDDIA_LAB_SKIP_APTO_COHERENCE`.

* **Pre-fusión atestación PPR (MAN-45):** ver Fase 1. Lab: `SDDIA_LAB_SKIP_PPR_ATTESTATION_ACCEPT`.

* **Store SSOT:** identidad vía `EventUri` / `EventManagerPort` (`SddIA/events/events-contract.md` §4). ADU-CA7: `scan_presented_for_branch` consulta el puerto.
* `context: pr-lifecycle` debe registrarse en `execution-contexts.md` y políticas del invocante.
* Prohibido `gh pr merge` como sustituto del merge soberano local en este proceso salvo evolución explícita de la norma.
