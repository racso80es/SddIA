---
context: ecosystem-evolution
contract: process-contract v1.4.0
hash_signature: "sha256:7778755b77c68da59ece78afce97c829fbef740f157135ef87e6312c9024a2b6"
inputs:
- source_process: 'Origen del flujo: feature | bug-fix | refactorization'
- persist_ref: Carpeta de tarea / referencia de persistencia acordada en el ciclo
- branch_name: Nombre de rama de trabajo bajo control de versión
- pr_title: Título del pull request (forja GitHub)
- pr_body: Cuerpo Markdown del PR (opcional)
- target_branch: Rama destino del PR (default main)
minteo_maximo: null
name: delivery-close-cycle
outputs:
- 'pr_url: URL del pull request abierto o actualizado'
- 'event_id: UUID v4 del evento PullRequest_Presented'
- 'event_uri: Token lógico EventUri resuelto por EventManagerPort (eda://{family}/{event_id})'
- 'closed_branch: Rama cerrada o higienizada según política local'
- 'evolution_entry: Referencia opcional a entrada en evolution/'
phases:
- delegates_to:
  - skill:git-manager
  intent: Consolidar commit final del trabajo antes del cierre remoto.
  name: Snapshot final
  id: snapshot_final
- delegates_to:
  - agent:argos
  intent: Evaluar mutaciones bajo SddIA/ para feature, bug-fix y refactorization; propagar sddia_paths.
  name: Impacto SddIA condicional
  id: impacto_sddia_condicional
- delegates_to:
  - agent:argos
  intent: Invocar sddia-qa gate-evolution --json --range --sync-base; bloquear si material sin registro evolution.
  name: Aduana evolution
  id: aduana_evolution
- delegates_to:
  - agent:argos
  intent: Invocar sddia-qa audit-eda-coverage --scan --json; si orphan_count > 0, Argos registra Ruido de Sistema (block) salvo excepción documentada de backfill Fase C en curso.
  name: Aduana EDA genómica
  id: aduana_eda_genomica
- delegates_to:
  - agent:argos
  intent: Invocar sddia-qa verify-process-integrity y verify-tools-index; bloquear si alguno falla.
  name: Aduana integridad índices
  id: aduana_integridad_indices
- intent: Exigir atestación QA aprobada para el tree_sha de HEAD y árbol cerrado (salvo validacion.md en persist_ref) antes del push.
  name: Gate pre-PR QA
  id: gate_pre_pr_qa
- intent: Tras global APTO, contrastar coherencia apto_coherence (execution/plan) y sellar validated_*_sha + diff_digest (excl. validacion.md); fricción apto_coherence_* sin System_Fracture; lab SDDIA_LAB_SKIP_APTO_COHERENCE.
  name: Sello diff validacion
  id: sello_diff_validacion
- delegates_to:
  - skill:git-manager
  intent: Publicar la rama de trabajo en origin antes de abrir el PR.
  name: Publicación remota
  requires_capability:
  - contract: proc.git_sync
    id: proc:git-sync
    version: '>=1.0.0'
  id: publicacion_remota
- delegates_to:
  - skill:shell-executor
  intent: Crear o resolver el PR en GitHub vía gh (shell-executor); capturar pr_url.
  name: Apertura en forja
  id: apertura_en_forja
- delegates_to:
  - action:emit-pr-presented-event
  intent: Emitir PullRequest_Presented en eda_bus.pending para anclaje posterior vía watcher.
  name: Sello Presentación ECST
  id: sello_presentacion_ecst
- delegates_to:
  - skill:git-manager
  intent: 'Cerrar ciclo local (close-cycle): limpieza de ramas temporales y estado de repo consistente.'
  name: Higiene local
  id: higiene_local
porcentaje_de_exito: null
uuid: 5417c92c-da7f-4d46-b245-55cf1b17961a
version: 2.0.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# delivery-close-cycle

Proceso paramétrico de **cierre de entrega** reutilizable desde `feature`, `bug-fix` y `refactorization`. Encadena snapshot git, evaluación condicional de impacto en el Core SddIA, publicación remota, apertura de PR en forja, sello **PullRequest_Presented** e higiene local.

**Emisión Presented:** vía `action:emit-pr-presented-event` / `EventManagerPort` (`events-contract.md` §4). **Simetría fractal (presentación):** el merge soberano y el sello **PullRequest_Merged** pertenecen exclusivamente a `accept-pr` + `emit-pr-merged-event`, no a este proceso.

## Fase Publicación remota

1. `skill:git-manager` → `push` con `operation_payload_json`: `{ "remote": "origin", "branch": "<branch_name>", "force": false }`.
2. Abortar si `success` es `false`.
3. Push rechazado `non-fast-forward` / `(fetch first)` con `[rejected]`: envelope `status: blocked`, `friction_id: F-DCC-PUSH-NON-FAST-FORWARD`, `operator_hint` (sincronizar vía `skill:git-manager` fetch + pull). **No** `System_Fracture_Detected`.

## Fase Apertura en forja

1. `skill:shell-executor` → `executable: gh`, `arguments` según `pull-request-orchestration.md` (típ. `pr create` con `--title`, `--head`, `--base`).
2. Capturar `pr_url` desde salida de `gh` o `gh pr view --json url` si el PR ya existía.
3. **`gh` prohibido en `git-manager`.**

## Fase Sello Presentación ECST

Invocar **`action:emit-pr-presented-event`** con:

| Input | Origen |
| :--- | :--- |
| `branch` | `branch_name` del proceso |
| `status` | `"presented"` |
| `emitter_agent` | `"delivery-close-cycle"` |
| `pr_url` | `state.pr_url`, `inputs.pr_url` o `gh pr view` si la forja quedó `blocked` antes de `gh` |
| `correlation_id` | input del proceso si el orquestador lo inyecta |

Sin URL resoluble: envelope `status: blocked`, `friction_id: F-DCC-PR-URL-MISSING`; no se emite `PullRequest_Presented`.

Salidas propagadas al envelope del proceso: `event_id`, `event_uri`, `pr_url`.

## Notas operativas

* La fase **Impacto SddIA condicional** debe evaluarse como no-op documentado cuando no aplique (`source_process` distinto de `feature` / `bug-fix` / `refactorization`, o sin cambios bajo `SddIA/`), sin bloquear el resto del ciclo.
* **Aduana evolution:** ejecutar `SddIA/target/debug/sddia-qa gate-evolution --json --range --sync-base`. Bloquear si `EVOL_MATERIAL_UNREGISTERED` / `EVOL_HASH_MISMATCH`. No emitir `System_Fracture_Detected` (gate accionable, F4b). CI usa `--require-synced-base` (AEL-CA13).
* **Aduana integridad índices:** ejecutar `SddIA/target/debug/sddia-qa verify-process-integrity` y `verify-tools-index` antes de la publicación remota; veredicto `block` si alguno falla (paridad job CI `sddia-index-integrity`).
* **Gate pre-PR QA:** antes de Publicación remota, comprobar atestación `qa-attestation/1.0` con `verdict: aprobado` y `tree_sha` de HEAD, y `git status --porcelain` vacío salvo `{persist_ref}/validacion.md`. Envelope `status: blocked` con `friction_id: qa_attestation_required` o `worktree_not_closed`. No `System_Fracture_Detected`. Lab: `SDDIA_LAB_SKIP_PRE_PR_QA_GATE`.
* **Sello diff validacion:** tras gate pre-PR, exigir `global: APTO` y **contraste `apto_coherence`** (`execution`/`plan` del mismo `persist_ref`) antes de sellar; escribir `validated_*_sha` + `diff_digest`; commitear vía `skill:git-manager`. Fricciones `validacion_not_apto`, `diff_seal_commit_failed`, `apto_coherence_execution_blocked`, `apto_coherence_open_phases`, `apto_coherence_resolution` **sin** `System_Fracture_Detected`. Lab: `SDDIA_LAB_SKIP_DIFF_SEAL` (omite sello) y `SDDIA_LAB_SKIP_APTO_COHERENCE` (omite solo coherencia).
* **Aduana EDA genómica:** ejecutar `SddIA/target/debug/sddia-qa audit-eda-coverage --scan --json` (o `./sddia-run.sh --audit-eda-coverage --scan --json`). Si `orphan_count > 0` (entidad indexada sin `Domain_Entity_Created` correlacionado), Argos emite **Ruido de Sistema** con veredicto `block`. **Excepción `backfill-manifest.json`:** si `{persist_ref}/backfill-manifest.json` declara `correlation_id` y **no** tiene `merkle_anchored: true`, el gate degrada el veredicto a `warn` con `argos_noise: "backfill Fase C en curso"`. El backfill canónico sigue siendo `entity-manager` → `emit-domain-mutation` con sello `--anchor-merkle` al cierre.
* Todas las rutas y políticas se resuelven exclusivamente vía `cumulo.paths.json` y normas enlazadas (`git-operations`, `pull-request-orchestration`).

## Perfil laboratorio (`execute-process` nativo)

Variables de entorno para cápsulas físicas sin efectos destructivos por defecto:

| Variable | Efecto |
| :--- | :--- |
| `SDDIA_LAB_SKIP_SNAPSHOT` | Omite `get_last_commit` en Snapshot final |
| `SDDIA_LAB_SKIP_IMPACT_ASSESSMENT` | Omite gate Impacto SddIA (fase 2) |
| `SDDIA_LAB_SKIP_GIT_PUSH` | Omite push en Publicación remota |
| `SDDIA_LAB_SIMULATE_GH_PR` | Simula `pr_url` sin invocar `gh` |
| `SDDIA_LAB_SKIP_HIGIENE` | Omite checkout/delete en Higiene local |
| `SDDIA_LAB_SKIP_INDEX_INTEGRITY` | Omite Aduana integridad índices (`verify-process-integrity` + `verify-tools-index`) |
| `SDDIA_LAB_SKIP_EVOLUTION_GATE` | Omite Aduana evolution (`gate-evolution --range --sync-base`) |
| `SDDIA_LAB_SKIP_PRE_PR_QA_GATE` | Omite Gate pre-PR QA |
| `SDDIA_LAB_SKIP_DIFF_SEAL` | Omite Sello diff validacion |
| `SDDIA_LAB_SKIP_APTO_COHERENCE` | Omite contraste coherencia APTO (MAN-44; solo lab) |
| `SDDIA_LAB_DELETE_FEATURE_BRANCH` | Si `1`, intenta borrar rama tras sello (solo lab explícito) |

**Anti-recursión hook (Ola B):** el hook pre-push omite re-entrada si `SDDIA_HOOK_DELIVERY_CLOSE=1` (inyectado por `invoke_process`; guarda `in_delivery_close_cycle`). La fase Publicación remota puede fijar `SDDIA_SKIP_HOOKS=1` **solo** en el subproceso `git-manager` cuando `source_process == git-hook-pre-push`. El gate evolution del hook **no** corre si hay ramas nuevas (delega en este proceso); corre si no hay ramas nuevas (PR OPEN).

### Fase Impacto SddIA condicional (perfil laboratorio)

Handler `delivery-impact-assessment`: diff name-only contra `origin/<target_branch>`; filtra prefijo `SddIA/`. Evalúa si `source_process` ∈ {`feature`, `bug-fix`, `refactorization`}. No bloquea el ciclo en lab (Argos IDE fuera de alcance). Propaga `sddia_impact` en envelope del proceso.
