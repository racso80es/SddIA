---
feature_name: cognitive-tier-native-dispatch
created: "2026-10-31"
base: main
scope: cápsula nativa antes que AGENT_PHASE en store
process: bug-fix
branch_name: Fix_Events_Multiple
persist_ref: docs/fixes/cognitive-tier-native-dispatch
tracker_ref: AUT-26
execution_id: "73d65b19-25cb-489c-94e5-abffe60b1e46"
---

# Especificación — despacho nativo con tier cognitivo

## Corte

`dispatch_subscriber` calcula `store_backed` cuando `event_path` no es un fichero. Si eso es cierto, el agente tiene tier `high`, `medium` o `low`, y `subscriber_declares_native` es falso, devuelve en `dispatch_cognitive_subscriber`. Una salida 0 de ese runtime es `success`.

`subscriber_declares_native` recorre `action` y `tool`. Un `process` no vacío no entra. Tekton + `tracker-stamp` y Argos + `pull-request-review` caen en `AGENT_PHASE` con el ECST solo en `events.redb`.

## Decisión

El predicado incluye `process` con la misma regla que `action` y `tool`: cadena no vacía tras recorte. El orden del corte no cambia: la cápsula nativa gana al tier. `AGENT_PHASE` queda para el objeto que solo trae `agent`.

No se añade un escritor nuevo del acuse. La rama de `notify-humanized-pr-merged` ya devuelve `success` solo en `NotifyOutcome::Sent` y `skipped-empty-message` en `SkippedEmpty`. Al no entrar en el runtime, esa rama es la entrega.

## Regresión

En el módulo `tier_dispatch` de `route_domain_core`:

| Suscriptor | Path ausente | Runtime | Estado |
|------------|--------------|---------|--------|
| solo `agent`, tier cognitivo | sí | `AGENT_PHASE` | el test vigente |
| solo `agent`, tier `none` | sí | no arranca | el test vigente |
| `process` no vacío, tier cognitivo | sí | no arranca | fallo de orquestador en el temporal, no `success` del script |
| `action` `notify-humanized-pr-merged`, tier cognitivo | sí | no arranca | estado de la acción, no del script |
| `process` no vacío, tier `none` | sí | no arranca | mismo camino nativo |

El script de `SDDIA_AGENT_RUNTIME_COMMAND` no debe ejecutarse en las filas nativas. Un workspace `.SddIA/workspaces/eda-dispatch/{event_id}/{agente}` creado por el runtime no cuenta como entrega.

## Fuera de alcance

Puerto de carga y no-op de `tracker-stamp` (AUT-21). Lock de `events.redb` (AUT-16). `tracker_ref` del puente GitHub (AUT-25). Reenvío manual del Telegram de AUT-8. Trait o backend nuevo del bus.

## Criterios de aceptación

Coinciden con el cuerpo sellado de AUT-26: cápsula nativa también en store, `AGENT_PHASE` solo sin cápsula, `success` de la acción post-merge solo tras el envío o `skipped-empty-message`, `tracker-stamp` nativo, tier `none` intacto, test de regresión, `validacion.md` APTO en esta rama, AUT-26 en Done sin fichero nuevo en `docs/todos/`.
