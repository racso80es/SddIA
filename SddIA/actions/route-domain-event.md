---
uuid: "9b314f74-44d3-43c4-b916-871a9fa43f45"
name: "route-domain-event"
version: "1.1.0"
contract: "actions-contract v1.2.0"
context: "event-routing"
capabilities:
  - "domain-event-routing"
  - "subscription-registry-lookup"
  - "subscriber-witness-ledger"
  - "event-bus-lifecycle-move"
  - "delegate-filesystem-manager"
  - "subscriber-fanout-orchestration"
inputs:
  - "event_file_path": "string; ruta relativa al JSON padre en eda_bus.pending (cumulo.paths.json → event_bus/.events/pending/)"
outputs:
  - "success": "boolean"
  - "delivery_status": "object; mapa subscriber_id → success | failed | skipped-* según respuestas de delegación"
minteo_maximo: null
porcentaje_de_exito: null
---

# Acción: route-domain-event

## 1. Propósito

Leer un evento padre inmutable en `eda_bus.pending`, despacharlo a sus suscriptores vía **testigos atómicos** en `eda_bus.subscribers.*`, y dejar el padre intacto hasta que `event-sweeper.py` confirme consenso en `processed/`. Consumidor asíncrono del Bus de Eventos local (Ola C V3).

## 2. Orquestación

Gate **Cerbero** por `context` de cada cápsula. Rutas vía `cumulo.paths.json` → `core/event-subscriptions.json` (SSOT de suscripciones). Sin terminal cruda.

### Paso 1 — Lectura (`skill:filesystem-manager`)

| Campo | Valor |
| :--- | :--- |
| `operation` | `READ_FILE` |
| `target_path` | `<event_file_path>` (padre en `pending/`) |

- Abortar si `exitCode != 0`.
- Parsear contenido UTF-8 como JSON; abortar si inválido.
- **Prohibido** mutar el JSON padre tras la lectura.

### Paso 2 — Tipo de evento

- Extraer `event_type` (string obligatorio del contrato ECST).
- Si falta: `success: false`, `exitCode: 1`.

### Paso 2b — Validación ECST (instancia ↔ Clase)

1. Resolver catálogo desde `cumulo.paths.json` → `directories.events` + `events/index.md`.
2. Comprobar que `event_type` existe en el índice de Clases ECST.
3. Cargar la Clase `{name}.md` y leer tablas **REQUIRED**, **OPTIONAL** y **FORBIDDEN** del payload.
4. Validar que `payload` contiene todos los campos **REQUIRED** (no nulos).
5. Validar que ningún campo **FORBIDDEN** aparece con valor distinto de `null`.
6. Si la validación falla: escribir testigo `ecst-gate` en `subscribers/dead-letter/` con `error_trace`; **no** mover ni mutar el padre; abortar fan-out.

*Implementación física:* `event-watcher.py` → `route_domain_event`.

### Paso 3 — Registro de suscripciones

1. Resolver ruta canónica: `cumulo.paths.json` → `eda_bus.subscriptions`.
2. Invocar `READ_FILE` sobre ese artefacto; parsear JSON.
3. Obtener `subscribers = registry[event_type]` (array).
4. Si `subscribers` es vacío o ausente: registrar no-op; continuar al Paso 7.

### Paso 3b — Filtro topológico fractal (Protocolo Acero Pilar 1)

1. Leer `origin_topology` del `payload` (`core` \| `local`; default `core`).
2. Para cada suscriptor, evaluar `applies_to_origin_topology`.
3. Si el array está ausente → suscriptor aplica a **ambas** topologías (compatibilidad legacy).
4. Si `origin_topology` no está en el array → omitir suscriptor (no registrar `failed`).

*Implementación física:* `subscriber_applies_to_topology` en `eda_bus_utils.py`.

### Paso 4 — Fan-out a suscriptores (testigos)

Para cada `subscriber` aplicable:

1. Calcular `subscriber_id` (p. ej. `cumulo`, `argos`, `cumulo.sync-entity-index` si el agente se repite).
2. Escribir testigo en `subscribers/processing/[event_id].[subscriber_id].json` con `state: processing`.
3. Delegar según tabla:

| Campo suscriptor | Delegación |
| :--- | :--- |
| `agent` + `tool` | Agente ejecuta tool (p. ej. `cumulo` → `iota-immutable-publisher`). |
| `agent` + `action` | Agente ejecuta action (p. ej. `cumulo` → `sync-entity-index`). |
| `agent` + `process` | Agente ejecuta proceso vía `execute-process` (p. ej. `argos` → `pull-request-review`). |

4. Tras delegación, registrar en `delivery_status` la clave `subscriber_id` con `"success"`, `"failed"` o `"skipped-*"`.
5. Promover testigo a `subscribers/processed/` (éxito o skip) o `subscribers/dead-letter/` (fallo, con `error_trace`).

Reglas:

- Resolución de cápsulas solo vía Cúmulo.
- El padre en `pending/` **nunca** se mueve en este paso.
- Idempotencia: reintentos pueden reescribir testigos si no hay consenso previo.

### Paso 5 — Cierre (stdout)

```json
{
  "success": true,
  "exitCode": 0,
  "data": {
    "success": true,
    "delivery_status": { "<subscriber_id>": "success|failed|skipped-*" },
    "parent_path": ".events/pending/<event_id>.json"
  }
}
```

En fallo de lectura, parseo o delegación crítica: `success: false`, `exitCode: 1`, `error` causal.

### Paso 6 — Recolección (delegado)

`event-sweeper.py` escanea `pending/`; si todos los suscriptores requeridos tienen testigo en `processed/`, purga el padre y archiva testigos. Si hay testigos en `dead-letter/`, emite alerta Kaizen sin borrar el padre.

## 3. Límites

* No emite eventos; no ancla DLT directamente (delega en tool/action por suscriptor).
* `context: event-routing` debe existir en `execution-contexts.md`.
* Suscriptores con `action` o `tool` inexistente → testigo `dead-letter/` para ese `subscriber_id`.
