---
feature_name: aut20-ecst-router-verify
created: "2026-11-01"
process: bug-fix
base: SddIA/engine/execute-process/src/engine/route_domain_core.rs
scope: "ECST en el despacho de route-domain-event y verify antes de Linear"
branch_name: Fix_Events_Multiple
persist_ref: docs/fixes/aut20-ecst-router-verify
tracker_ref: AUT-57
historia_ref: AUT-20
---

# Spec — AUT-57

## Problema

`route-domain-event` lee el ECST del store si el pending no es fichero. `dispatch_subscriber` solo copia ese ECST a los inputs de `tracker-stamp`, `tracker-sync-replay` y los procesos de correo/preferencias. El resto exige `payload.branch`. `Tracker_Issue_Changed` y `Domain_Entity_Telemetry_Captured` no lo traen: el suscriptor muere en `branch missing in payload`.

`load_ecst_from_inputs` acepta `inputs.event` con `event_id` sin leer `events.redb`. `tracker-stamp` verifica. `tracker-sync-replay` y `tracker-markdown-apply` escriben en Linear sin esa verificación.

## Contrato del despacho

Función única de ensamblado, usada por todas las ramas de proceso de `dispatch_subscriber`.

Siempre, si el suscriptor declara `process`:

| Campo | Valor |
|-------|--------|
| `event` | ECST que el router ya cargó (el mismo objeto del store) |
| `event_id` | `event.event_id` |
| `event_file_path` | ruta lógica `.events/pending/{event_id}.json` cuando el router la tenga |

Encima, los campos que el proceso ya exige:

| Proceso | Extra | Corte propio |
|---------|--------|----------------|
| `pull-request-review` | `branch` / `pr_*` actuales | sin `branch` → `failed` / `branch missing in payload` |
| `telegram-fallback-responder` | `text`, `chat_id` | texto vacío → `skipped-empty-text` |
| `execute-suite` | `suite_id`, `execution_strategy`, `asset_id` | sin `suite_id` → `failed` / `suite_id missing in payload` |
| `task-queue-manager` | inputs SDLC actuales (`Kalma2_Process_Requested`, `Aiua_Process_Requested`) | el error de payload SDLC se conserva |
| `tracker-sync-replay` | campos de replay actuales | — |
| resto del catálogo de dominio | solo el ECST | prohibido el corte global por `branch` |

Catálogo cubierto (claves `process` de `SddIA/core/event-domain-subscriptions.json`):

`pull-request-review`, `tracker-stamp`, `compile-ecosystem-map-snapshot`, `memory-evolution-ingest`, `cerbero-governance-react`, `fix-tool-process`, `execute-suite`, `email-triage-gateway`, `email-quick-action-ingest`, `user-preference-ingest`, `email-digest-preference-reply`, `telegram-fallback-responder`, `task-queue-manager`, `tracker-sync-replay`, `tracker-markdown-apply`.

`compile-ecosystem-map-snapshot` puede seguir ignorando el cuerpo: el contrato es que el invoke lo lleva. `cerbero-governance-react` y `fix-tool-process` no tienen atajo nativo en `run_process`; el router les pasa el ECST y los invoca. No se reescribe el núcleo fractal.

## Carga sin fichero

`run_memory_evolution_ingest` resuelve el ECST con `load_ecst_from_inputs` (`event`, luego `event_id`, luego `event_file_path`). Deja de exigir el fichero en disco. La ingesta sigue limitada a `Domain_Entity_Telemetry_Captured`.

`tracker-markdown-apply` ya carga por `load_ecst_from_inputs`. El router debe entregarle `event`.

## Verificación antes de Linear

Antes de `invoke_tool_for_process` hacia el tracker:

- `tracker_sync_replay::run`
- `tracker_markdown_apply::run`

Llaman a `verify_ecst_matches_bus`. Fallo (`ecst_not_in_bus` o `ecst_mismatch`): `success: false`, `exit_code: 1`, `data.delivery_status: failed-bus-mismatch`, cero llamadas al tracker. `tracker-stamp` no cambia.

Modo sin store (`filesystem`): `verify_ecst_matches_bus` ya es no-op. Se mantiene.

## Tests

1. Tabla con las 15 claves. Para cada una, el ensamblado contiene `event` idéntico al ECST de entrada. Ninguna clave del catálogo queda fuera de la tabla.
2. `route_domain_event` en `redb`, pending borrado:
   - `Tracker_Issue_Changed` → el estado del suscriptor no es `branch missing in payload` ni `event file not found`.
   - `Domain_Entity_Telemetry_Captured` → igual.
3. `tracker-sync-replay` y `tracker-markdown-apply` con `inputs.event` del mismo `event_id` y cuerpo distinto al store → `failed-bus-mismatch` y el mock de Linear no registra `update_issue_state`, `update_issue_description` ni `create_comment`.

## Fuera de alcance

Router `route-domain` (fractal), familia telemetría, criterios 2–3 y 5–7, cierre de AUT-20, rama nueva.
