---
feature_name: fracture-provider-sink
created: "2026-10-30"
process: refactorization
base: main
scope: sumidero de fractura según provider
tracker_ref: AUT-30
---

# Especificación

## Resolución

`inputs_with_self_hosted_slug` copia el payload e inyecta el único índice con `project_root` `.` o `./`. Si hay varios, gana el único `status: active`. Si no hay uno solo, error. Un slug presente no se sustituye.

`manifest_provider` usa ese slug. `invoke` escribe fichero solo con `LifecycleTarget::File`. `Unconfigured` devuelve `TRACKER_UNCONFIGURED`.

## Alta remota

`list_issues` incluye `description`. El mock de laboratorio mezcla las issues del store. Coincidencia: la descripción contiene `fracture_hash: {12 hex}`. Estado distinto de `state_map.done` y `state_map.cancelled` → `already_open`. Solo cerradas → `regression_opened`. Ninguna → `materialized`.

`create_issue` pasa por el puerto: título `{[FIX]|[REGRESIÓN]} {process_name} — fractura sistémica`, cuerpo `build_pbi_body` con `tracker_column: backlog` y `refinement: pending`, etiquetas `pbi` y `fix`, `state_name` = `state_map.backlog`. `target_path` es el identificador.

El gate `create_issue` acepta la acción `materialize-fracture-pbi` cuando su `.md` existe. No es un proceso con contexto `tracker-operations`.

## Hecho y enriquecimiento

Tras un alta nueva se emite `Fracture_Pbi_Materialized` con `target_path` y, si es remoto, `tracker_ref`. Mayeuta sale de `System_Fracture_Detected` y suscribe ese hecho. Enrich comenta la referencia opaca antes de buscar un markdown.

## Route y depósito

Si el suscriptor declara `action` o `tool`, el route no desvía a `AGENT_PHASE` por tier cognitivo. `emit_fracture_for_denial` propaga el error y el envelope marca `fracture_deposited`. El watcher registra el fallo de apertura de `events.redb` y hace `drop` del store antes de spawn de `route-domain-event`.
