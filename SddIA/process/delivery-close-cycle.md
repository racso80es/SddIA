---
uuid: 5417c92c-da7f-4d46-b245-55cf1b17961a
name: delivery-close-cycle
version: 1.1.0
contract: process-contract v1.3.0
context:
- ecosystem-evolution
hash_signature: sha256:bebaa3c536ff8a67dca9f61c841e594a4efcfd51593dd6badbc8549422232b60
inputs:
- source_process: 'Origen del flujo: feature | bug-fix | refactorization'
- persist_ref: Carpeta de tarea / referencia de persistencia acordada en el ciclo
- branch_name: Nombre de rama de trabajo bajo control de versión
- pr_title: Título del pull request (forja GitHub)
- pr_body: Cuerpo Markdown del PR (opcional)
- target_branch: Rama destino del PR (default main)
outputs:
- pr_url: URL del pull request abierto o actualizado
- event_id: UUID v4 del evento PullRequest_Presented
- target_path: Ruta relativa del JSON en eda_bus.pending
- closed_branch: Rama cerrada o higienizada según política local
- evolution_entry: Referencia opcional a entrada en evolution/
phases:
- name: Snapshot final
  intent: Consolidar commit final del trabajo antes del cierre remoto.
  delegates_to:
  - skill:git-manager
- name: Impacto SddIA condicional
  intent: Si source_process == feature y existen mutaciones bajo SddIA/, Argos registra
    impacto y evolución del Core.
  delegates_to:
  - agent:argos
- name: Aduana EDA genómica
  intent: Invocar audit-entity-eda-coverage.py --scan --json; si orphan_count > 0, Argos
    registra Ruido de Sistema (block) salvo excepción documentada de backfill Fase C en curso.
  delegates_to:
  - agent:argos
- name: Publicación remota
  intent: Publicar la rama de trabajo en origin antes de abrir el PR.
  delegates_to:
  - skill:git-manager
- name: Apertura en forja
  intent: Crear o resolver el PR en GitHub vía gh (shell-executor); capturar pr_url.
  delegates_to:
  - skill:shell-executor
- name: Sello Presentación ECST
  intent: Emitir PullRequest_Presented en eda_bus.pending para anclaje posterior vía watcher.
  delegates_to:
  - action:emit-pr-presented-event
- name: Higiene local
  intent: 'Cerrar ciclo local (close-cycle): limpieza de ramas temporales y estado
    de repo consistente.'
  delegates_to:
  - skill:git-manager
minteo_maximo: null
porcentaje_de_exito: null
---

# delivery-close-cycle

Proceso paramétrico de **cierre de entrega** reutilizable desde `feature`, `bug-fix` y `refactorization`. Encadena snapshot git, evaluación condicional de impacto en el Core SddIA, publicación remota, apertura de PR en forja, sello **PullRequest_Presented** e higiene local.

**Simetría fractal (presentación):** el merge soberano y el sello **PullRequest_Merged** pertenecen exclusivamente a `accept-pr` + `emit-pr-merged-event`, no a este proceso.

## Fase Publicación remota

1. `skill:git-manager` → `push` con `operation_payload_json`: `{ "remote": "origin", "branch": "<branch_name>", "force": false }`.
2. Abortar si `success` es `false`.

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
| `pr_url` | salida de la fase anterior (opcional en payload ECST v1.1) |
| `correlation_id` | input del proceso si el orquestador lo inyecta |

Salidas propagadas al envelope del proceso: `event_id`, `target_path`, `pr_url`.

## Notas operativas

* La fase **Impacto SddIA condicional** debe evaluarse como no-op documentado cuando no aplique (`source_process != feature` o sin cambios bajo `SddIA/`), sin bloquear el resto del ciclo.
* **Aduana EDA genómica:** ejecutar `python SddIA/scripts/qa/audit-entity-eda-coverage.py --scan --json`. Si `orphan_count > 0` (entidad indexada sin `Domain_Entity_Created` correlacionado), Argos emite **Ruido de Sistema** con veredicto `block` hasta backfill Fase C (`--emit --skip-dlt` + `--anchor-merkle` obligatorio al cierre). Excepción temporal: manifiesto `backfill-manifest.json` con `correlation_id` activo en la feature.
* Todas las rutas y políticas se resuelven exclusivamente vía `cumulo.paths.json` y normas enlazadas (`git-operations`, `pull-request-orchestration`).

## Perfil laboratorio (`execute-process.py`)

Variables de entorno para cápsulas físicas sin efectos destructivos por defecto:

| Variable | Efecto |
| :--- | :--- |
| `SDDIA_LAB_SKIP_SNAPSHOT` | Omite `get_last_commit` en Snapshot final |
| `SDDIA_LAB_SKIP_IMPACT_ASSESSMENT` | Omite gate Impacto SddIA (fase 2) |
| `SDDIA_LAB_SKIP_GIT_PUSH` | Omite push en Publicación remota |
| `SDDIA_LAB_SIMULATE_GH_PR` | Simula `pr_url` sin invocar `gh` |
| `SDDIA_LAB_SKIP_HIGIENE` | Omite checkout/delete en Higiene local |
| `SDDIA_LAB_DELETE_FEATURE_BRANCH` | Si `1`, intenta borrar rama tras sello (solo lab explícito) |

**Anti-recursión hook (Ola B):** cuando `source_process == git-hook-pre-push`, la fase Publicación remota ejecuta push con `SDDIA_SKIP_HOOKS=1` **solo** en el subproceso `git-manager`. El hook pre-push omite re-entrada si `SDDIA_HOOK_DELIVERY_CLOSE=1` (inyectado por `invoke_process` del hook).

### Fase Impacto SddIA condicional (perfil laboratorio)

Handler `delivery-impact-assessment`: diff name-only contra `origin/<target_branch>`; filtra prefijo `SddIA/`. Solo evalúa si `source_process == feature`. No bloquea el ciclo en lab (Argos IDE fuera de alcance). Propaga `sddia_impact` en envelope del proceso.
