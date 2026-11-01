---
feature_name: aut20-ecst-residual-dispatch
created: "2026-11-01"
process: bug-fix
base: SddIA/engine/execute-process/src/engine/route_domain_core.rs
scope: "ECST en telegram y task-queue; test de despacho por clave"
branch_name: Fix_Events_Multiple
persist_ref: docs/fixes/aut20-ecst-residual-dispatch
tracker_ref: AUT-58
historia_ref: AUT-20
---

# Spec — AUT-58

## Problema

AUT-57 dejó el ECST en el catálogo que no cortaba por contrato propio. Siguen fuera:

| Rama | Qué arma hoy | Qué falta |
|------|----------------|-----------|
| `telegram-fallback-responder` | `text`, `chat_id` | `event`, `event_id`, `event_file_path` |
| SDLC (`Kalma2_Process_Requested`, `Aiua_Process_Requested`) → `task-queue-manager` | `sdlc_process_request_inputs` | el mismo ECST |

`domain_process_catalog_inputs_include_ecst` llama a `build_ecst_subscriber_inputs` una vez por clave y no lee la clave. El test pasa aunque esas dos ramas omitan el ECST.

## Contrato

Una función en `route_domain_core` arma el mapa de inputs de una clave de proceso. `dispatch_subscriber` la usa en todas las ramas que hoy invocan un proceso. `build_ecst_subscriber_inputs` sigue siendo el bloque ECST; la función nueva lo fusiona con los campos propios.

Camino de éxito (el proceso se invoca):

| Clave | Extra | El mapa incluye |
|-------|--------|-----------------|
| `telegram-fallback-responder` | `text`, `chat_id` si viene | ECST |
| `task-queue-manager` | campos actuales de `sdlc_process_request_inputs` | ECST |
| resto de las 15 claves | los extras ya sellados en AUT-57 | ECST, sin regresión |

Cortes que no invocan y no cambian de motivo:

| Clave | Corte |
|-------|--------|
| `telegram-fallback-responder` | texto vacío → `skipped-empty-text` |
| `task-queue-manager` | `proceso no permitido` |
| `execute-suite` | sin `suite_id` → `suite_id missing in payload` |
| `pull-request-review` | sin `branch` → `branch missing in payload` |

Catálogo (claves `process` de `SddIA/core/event-domain-subscriptions.json`): las mismas 15 que AUT-57.

## Tests

1. La función de ensamblado, llamada con cada una de las 15 claves y el payload mínimo de su camino de éxito, devuelve `event` idéntico al ECST de entrada. La clave entra en la llamada. Prohibido un bucle que ignore la clave y solo llame a `build_ecst_subscriber_inputs`.
2. `route_domain_event` en `redb`, pending borrado:
   - `TelegramMessage_Received` con `text` no vacío → el estado del suscriptor no es un corte previo al handler por ECST ausente.
   - `Kalma2_Process_Requested` con `process` en la allowlist → igual.
3. Texto vacío de telegram y proceso SDLC fuera de allowlist conservan su estado actual.

## Fuera de alcance

Criterios 2–7, verificación de bus, handlers de telegram y de la cola, efectos remotos, router fractal, cierre de AUT-20.
