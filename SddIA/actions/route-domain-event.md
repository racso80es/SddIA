---
uuid: "9b314f74-44d3-43c4-b916-871a9fa43f45"
name: "route-domain-event"
version: "1.0.0"
contract: "actions-contract v1.2.0"
context: "event-routing"
capabilities:
  - "domain-event-routing"
  - "subscription-registry-lookup"
  - "delivery-state-ledger"
  - "event-bus-lifecycle-move"
  - "delegate-filesystem-manager"
  - "subscriber-fanout-orchestration"
inputs:
  - "event_file_path": "string; ruta relativa al workspace del JSON en eda_bus.pending o eda_bus.processing (cumulo.paths.json; fallback docs/events/pending/ o docs/events/processing/)"
outputs:
  - "success": "boolean"
  - "delivery_status": "object; mapa agente/suscriptor → success | failed según respuestas de delegación"
minteo_maximo: null
porcentaje_de_exito: null
---

# Acción: route-domain-event

## 1. Propósito

Leer un evento en cola (`pending/` o `processing/`), despacharlo a sus suscriptores, mutar su estado de entrega y moverlo a `processed/` o `dead-letter/`. Consumidor asíncrono del Bus de Eventos local (Arquitectura V2).

## 2. Orquestación

Gate **Cerbero** por `context` de cada cápsula. Rutas vía `cumulo.paths.json` → `core/event-subscriptions.json` (SSOT de suscripciones). Sin terminal cruda.

### Paso 1 — Lectura (`skill:filesystem-manager`)

| Campo | Valor |
| :--- | :--- |
| `operation` | `READ_FILE` |
| `target_path` | `<event_file_path>` |

- Abortar si `exitCode != 0`.
- Parsear contenido UTF-8 como JSON; abortar si inválido.

### Paso 2 — Tipo de evento

- Extraer `event_type` (string obligatorio del contrato V2).
- Si falta: `success: false`, `exitCode: 1`.

### Paso 2b — Validación ECST (instancia ↔ Clase)

1. Resolver catálogo desde `cumulo.paths.json` → `directories.events` + `events/index.md`.
2. Comprobar que `event_type` existe en el índice de Clases ECST.
3. Cargar la Clase `{name}.md` y leer tablas **REQUIRED**, **OPTIONAL** y **FORBIDDEN** del payload.
4. Validar que `payload` contiene todos los campos **REQUIRED** (no nulos).
5. Validar que ningún campo **FORBIDDEN** aparece con valor distinto de `null` (`hash_signature` en eventos Git: prohibido si la clave existe).
6. Si la validación falla: registrar `delivery_state.ecst_validation = "failed"` y `ecst_errors[]`; mover a `{eda_bus.dead_letter}/` sin fan-out.

*Implementación física:* `event-watcher.py` → `route_domain_event` (cápsula de esta acción).

### Paso 3 — Registro de suscripciones

1. Resolver ruta canónica: `cumulo.paths.json` → `eda_bus.subscriptions` (fallback `SddIA/core/event-subscriptions.json`).
2. Invocar `READ_FILE` sobre ese artefacto; parsear JSON.
3. Obtener `subscribers = registry[event_type]` (array).
4. Si `subscribers` es vacío o ausente: registrar en `delivery_status` como no-op documentado; continuar al Paso 6 con destino `processed/` salvo política invocante.

### Paso 3b — Filtro topológico fractal (Protocolo Acero Pilar 1)

1. Leer `origin_topology` del `payload` del evento (`core` \| `local`; default `core` en instancias legacy sin campo).
2. Para cada suscriptor, evaluar `applies_to_origin_topology` del registro en `event-subscriptions.json`.
3. Si el array está ausente → suscriptor aplica a **ambas** topologías (compatibilidad legacy).
4. Si `origin_topology` del evento no está en el array → **omitir** ese suscriptor (no registrar `failed`; no invocar cápsula).
5. Ejemplos: `sync-entity-index` e `iota-immutable-publisher` en `Domain_Entity_*` declaran `["core"]`; eventos `local` no mutan `SddIA/*/index.md` ni anclan DLT por entidad.

*Implementación física:* `event-watcher.py` → `subscriber_applies_to_topology` antes del fan-out.

### Paso 4 — Fan-out a suscriptores

Para cada elemento `subscriber` del array:

| Campo suscriptor | Delegación |
| :--- | :--- |
| `agent` + `tool` | Invocar **agente** `agent` para ejecutar **tool** indexada (p. ej. `cumulo` → `iota-immutable-publisher`) pasando el objeto evento completo (o `payload` desnormalizado + metadatos `event_id`, `event_type`, `timestamp`). |
| `agent` + `action` | Invocar **agente** `agent` para ejecutar **action** indexada (p. ej. `cumulo` → `sync-entity-index`) con el evento como contexto de entrada. |
| `agent` + `process` | Invocar **agente** orquestador para ejecutar **proceso** indexado (p. ej. `argos` → `pull-request-review`) vía `execute-process`; inputs mapeados desde payload ECST. |

Reglas:

- Resolución de cápsulas solo vía Cúmulo (`directories.agents`, `directories.tools`, `directories.actions`, `execution_capsules.tools`).
- Tras cada delegación, registrar en `delivery_status` la clave del **agent** (string del suscriptor) con valor `"success"` o `"failed"` según envelope de la cápsula (`success` y `exitCode`).
- Si `on_error` implícito del bus: un solo `failed` impide consolidación total (véase Paso 6).

### Paso 5 — Mutación del ledger (`delivery_state`)

1. Fusionar `delivery_status` en el bloque `delivery_state` del JSON del evento (claves = nombres de `agent` en suscriptores).
2. Preservar entradas previas `"success"` en reintentos; solo reinvocar suscriptores no marcados como `"success"` (idempotencia V2).
3. Serializar JSON actualizado.

### Paso 6 — Traslado (`skill:filesystem-manager`)

Destino relativo al workspace:

| Condición | `operation` | `target_path` destino |
| :--- | :--- | :--- |
| Todos los valores en `delivery_status` son `"success"` (o array de suscriptores vacío) | `MOVE_FILE` | `{eda_bus.processed}/<nombre_archivo>` |
| Algún valor es `"failed"` | `MOVE_FILE` | `{eda_bus.dead_letter}/<nombre_archivo>` |

- `destination_path` / parámetros según contrato `filesystem-manager` para `MOVE_FILE`.
- Origen: ruta actual de `event_file_path` bajo `pending/`.
- Si falta directorio destino, invocar antes `CREATE_DIR`.

### Paso 7 — Cierre (stdout)

```json
{
  "success": true,
  "exitCode": 0,
  "data": {
    "success": true,
    "delivery_status": { "<agent>": "success|failed" }
  }
}
```

En fallo de lectura, parseo o movimiento: `success: false`, `exitCode: 1`, `data: null`, `error` causal.

## 3. Límites

* No emite eventos; no ancla DLT directamente (delega en tool/action por suscriptor).
* `context: event-routing` debe existir en `execution-contexts.md` antes de producción.
* Suscriptores con `action` o `tool` inexistente en catálogo → `failed` para ese agente y destino `dead-letter/` salvo recuperación manual.
