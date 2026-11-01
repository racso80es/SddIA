---
feature_name: event-bus-audit-store-signals
created: "2026-10-31"
base: main
scope: señales de entrega y residuo en la auditoría redb
process: bug-fix
branch_name: Fix_Events_Multiple
persist_ref: docs/fixes/event-bus-audit-store-signals
tracker_ref: AUT-27
execution_id: "6cd0a99f-1d58-43ee-8e30-31e9e13a090b"
---

# Especificación — señales del store en event-bus-audit

## Corte

Con `writes_store`, `run_audit` retorna en `audit_state_from_store` y no ejecuta el cruce de cobertura del modo filesystem. Esa función agrega conteos. Ante `EventStore::open` en error devuelve un `ScanState` por defecto. `list_deliveries` ya entrega `(event_id, subscriber_id, status)` y no se usa más que para sumar `failed`, `pending` y `success`.

El id canónico de suscriptor es `sddia_eda_catalog::subscriber_id`: `agente` más `process`, `action` o `tool`. El presentado vivo guarda claves `argos` y `cumulo`. El catálogo de `PullRequest_Presented` declara `argos.pull-request-review`, `tekton.tracker-stamp` y `argos.send-telegram-notification`.

## Decisión

Cuatro anomalías nuevas, solo en el camino redb, sobre eventos de familia dominio:

| kind | Condición |
|------|-----------|
| `delivery_fuera_de_catalogo` | `subscriber_id` de una fila que no está en el plan aplicable del `event_type` |
| `suscriptor_sin_fila` | id del plan aplicable sin fila en `subscriber_delivery` |
| `success_sin_efecto` | `success`, tier cognitivo, workspace de despacho vacío; `tracker-stamp` exige además ausencia del jsonl de `payload.tracker_ref` |
| `residual_filesystem` | `*.json` bajo el pending de Cúmulo; el detalle cita la fractura `F-EDA-RESIDUAL-FILESYSTEM` si la traza nombra ese fichero |
| `store_unavailable` | `EventStore::open` falló. El informe lo dice en sección propia. Los conteos de esa corrida no se presentan como bus vacío sano |

El plan aplicable sale de `plan_route` con el registro de dominio y, si `.SddIA/project.md` declara `codex_slug: codex-software-engineering`, el registro del códice. `subscriber_applies_to_topology` filtra por el payload del ECST. Ids `_disposition` y claves vacías no son suscriptores.

`needs_kaizen` no incorpora estas kinds. `success` del proceso de auditoría se mantiene.

## Regresión

Tests en `SddIA/tools/event-bus-audit`, modo redb de laboratorio, base temporal:

- `open` con error: el informe contiene `store_unavailable` y no un escaneo de ceros presentado como bus sano.
- `PullRequest_Merged` con `argos.notify-humanized-pr-merged` y `tekton.tracker-stamp` en `success`, workspace vacío y sin jsonl: dos `success_sin_efecto`. Con jsonl de `tracker_ref`, esa fila de Tekton no se marca.
- `PullRequest_Presented` con filas `argos` y `cumulo`: `delivery_fuera_de_catalogo` para esas dos, y `suscriptor_sin_fila` para los tres ids del catálogo.
- Un JSON en pending más un `System_Fracture_Detected` cuya traza lo nombra: `residual_filesystem` con el `friction_id`.

La comprobación sobre `events.redb` del repo es de ejecución, cuando `open` responde. Si el store está bloqueado, el informe dice `store_unavailable`. Esa corrida no sustituye el fixture.

## Fuera de alcance

Despacho `AGENT_PHASE` (AUT-26). Puerto de lectura (AUT-21). Plan precargado del puente GitHub (AUT-25). Reparar el lock (AUT-16). Reescritura de `reconcile_unstamped`. Alta o cambio de frontmatter de la entidad `event-bus-audit`.

## Criterios de aceptación

Coinciden con el cuerpo sellado de AUT-27: cruce por evento de dominio, `success_sin_efecto` con la cláusula de `tracker-stamp`, `store_unavailable` ante `open` fallido, inventario residual con fractura asociada, los dos ids vivos cuando el store abre, test de regresión, `validacion.md` APTO en esta rama, AUT-27 en Done sin fichero nuevo en `docs/todos/`.
