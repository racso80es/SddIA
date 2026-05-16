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
  - "event_file_path": "string; ruta relativa al workspace del JSON en .SddIA/events/pending/ (o ruta absoluta validada por Cúmulo)"
outputs:
  - "success": "boolean"
  - "delivery_status": "object; mapa agente/suscriptor → success | failed según respuestas de delegación"
minteo_maximo: null
porcentaje_de_exito: null
---

# Acción: route-domain-event

## 1. Propósito

Leer un evento pendiente, despacharlo a sus suscriptores, mutar su estado de entrega y moverlo a `processed/` o `dead-letter/`. Consumidor asíncrono del Bus de Eventos local (Arquitectura V2).

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

### Paso 3 — Registro de suscripciones

1. Resolver ruta canónica: `SddIA/core/event-subscriptions.json` (clave acordada en topología Core; no hardcodear rutas de host).
2. Invocar `READ_FILE` sobre ese artefacto; parsear JSON.
3. Obtener `subscribers = registry[event_type]` (array).
4. Si `subscribers` es vacío o ausente: registrar en `delivery_status` como no-op documentado; continuar al Paso 6 con destino `processed/` salvo política invocante.

### Paso 4 — Fan-out a suscriptores

Para cada elemento `subscriber` del array:

| Campo suscriptor | Delegación |
| :--- | :--- |
| `agent` + `tool` | Invocar **agente** `agent` para ejecutar **tool** indexada (p. ej. `cumulo` → `iota-immutable-publisher`) pasando el objeto evento completo (o `payload` desnormalizado + metadatos `event_id`, `event_type`, `timestamp`). |
| `agent` + `action` | Invocar **agente** `agent` para ejecutar **action** indexada (p. ej. `argos` → `log-audit-intent`) con el evento como contexto de entrada. |

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
| Todos los valores en `delivery_status` son `"success"` (o array de suscriptores vacío) | `MOVE_FILE` | `.SddIA/events/processed/<nombre_archivo>` |
| Algún valor es `"failed"` | `MOVE_FILE` | `.SddIA/events/dead-letter/<nombre_archivo>` |

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
