---
feature_name: kalma2-event-bus-integration
created: "2026-07-19"
process: feature
base: main
scope: kalma2-event-bus-integration
version_spec: "1.0.0"
uuid: 7047f38b-333f-4b85-bab1-1a6ff6992009
status: validacion_apto
laudo: dual-signal-delivery-plus-orchestration
---

# Especificación — kalma2-event-bus-integration

## 1. Topología de responsabilidades

```text
interfaces/kalma2/app.js              ← evoluciona (pending + poll + degraded)
kalma2-bridge (Rust)                  ← evoluciona (GET /api/status lectura)
   └─ POST /api/interact → execute-process --process kalma2-interact  (sin cambio semántico)
        └─ handlers::kalma2::run      ← evoluciona (degraded + alias correlation_id)
             ├─ chat → SYNTHESIZE | fallback (degraded:true)
             └─ execute → Kalma2_Process_Requested + acuse {event_id, correlation_id, emitted}
 peaje termodinámico                  ← evoluciona (propaga correlation_id → PEC)
 event-watcher / route-domain         ← sin cambio de contrato (ya inyecta correlation_id)
```

Invariante: bridge **solo lee** bus fractal en `/api/status`; **nunca** escribe eventos ni interpreta prompts.

## 2. Laudos Dedalo (cierran preguntas Mayeuta)

| Ref | Pregunta | Laudo | Justificación |
|-----|----------|-------|---------------|
| **L1** | ¿`Process_Execution_Completed` o `Kalma2_Process_Resolved`? | **Dual-signal; sin evento de dominio nuevo.** (a) `delivery_state` del `Kalma2_Process_Requested` = aceptación del Sistema Nervioso; (b) `Process_Execution_Completed.payload.correlation_id` = cierre del proceso hijo. | `Kalma2_Process_Resolved` duplicaría ECST/suscriptores. PEC ya existe; falta plumb de `correlation_id` (hoy se inyecta en `process_inputs` desde `route_domain_core` pero el peaje no lo proyecta al payload PEC). |
| **L2** | ¿Dónde vive el endpoint de status? | **`GET /api/status` en `kalma2-bridge`**. | Lectura FS/JSON sin lógica de negocio = compatible con ceguera espacial. Prohibido que el browser lea `.events/` directo. |
| **L3** | ¿`correlation_id` aparte de `event_id`? | **`correlation_id ≡ event_id`** del `Kalma2_Process_Requested`. Alias explícito en `data` del acuse. | Ya es el valor que `dispatch_subscriber` mete en `process_inputs.correlation_id`. No generar UUID paralelo. |
| **L4** | ¿`degraded: true` en chat fallback? | **Obligatorio.** | O5: la UI no puede pintar eco Tormentosa como veredicto S+. |

### Deudas heredadas (laudo de alcance)

| Deuda router | ¿Bloqueante? | Decisión |
|--------------|:------------:|----------|
| D1 timeout CLI | No | Fuera de v1; documentar riesgo |
| D3 task-queue-manager genérico | Parcial | Status refleja fallo de despacho; no reescribir TQM |
| D5 E2E CLI real CI | No | Smoke lab con fallback + execute |

## 3. Contratos HTTP

### 3.1 `POST /api/interact` (existente — enriquecimiento de respuesta)

Entrada sin cambio: `{"prompt": string}`.

Salida (passthrough envelope orquestador). Campos `data` relevantes:

| Campo | Chat OK | Chat degradado | Execute encolado |
|-------|---------|----------------|------------------|
| `response` | texto LLM | eco determinista | acuse textual |
| `degraded` | `false`/ausente | **`true`** | ausente |
| `emitted` | — | — | `true` |
| `event_id` | — | — | UUID dominio |
| `correlation_id` | — | — | **mismo** que `event_id` |
| `event_type` | — | — | `Kalma2_Process_Requested` |

### 3.2 `GET /api/status?event_id=<uuid>` (nuevo)

| Aspecto | Contrato |
|---------|----------|
| Auth | Localhost only (igual que bridge) |
| Ceguera | Solo localiza JSON bajo `eda_fractal.domain` / `orchestration` / `dead_letter`; proyecta enum |
| 400 | `event_id` ausente o no UUID |
| 404 | No existe en ninguna familia buscada |
| 200 | Cuerpo abajo |

```json
{
  "success": true,
  "event_id": "<uuid>",
  "correlation_id": "<uuid>",
  "status": "pending | routed | completed | failed",
  "domain": {
    "found": true,
    "event_type": "Kalma2_Process_Requested",
    "delivery_status": { "<subscriber_id>": "success|failed|…" }
  },
  "orchestration": {
    "found": false,
    "event_id": null,
    "process_name": null,
    "process_status": null
  },
  "message": "texto breve para UI"
}
```

#### Reglas de proyección `status` (orden)

1. Dominio en `dead_letter` o algún subscriber `failed` no-skip → **`failed`**
2. Existe PEC (`Process_Execution_Completed`) con `payload.correlation_id == event_id`:
   - `payload.status == success` (o ausente tratado como success) → **`completed`**
   - otro → **`failed`**
3. Dominio presente y todos los subscribers terminal-ok/skip → **`routed`**
4. Dominio presente sin consenso → **`pending`**
5. No encontrado → HTTP 404

> **Nota:** `routed` ≠ fin de feature larga en IDE. Es «Sistema Nervioso aceptó». `completed` exige PEC correlacionado (proceso orquestado que emitió peaje).

### 3.3 Routing bridge

```text
POST /api/interact     → handle_interact (existente)
GET  /api/status       → handle_status (nuevo; ANTES de static)
GET  /*                → serve_static (existente)
```

## 4. Plumb `correlation_id` → `Process_Execution_Completed`

Hoy (`thermodynamic.rs`): payload PEC = `{asset_id, process_name, status, workspace_path, execution_id?, phase_count?, persist_ref?}`. **Sin** `correlation_id`.

Cambio acotado:

1. `run_process` / peaje recibe `process_inputs` (ya disponible en el envelope de ejecución).
2. Si `process_inputs.correlation_id` es string no vacío → copiar a `payload.correlation_id` del PEC.
3. Sin `correlation_id` → omitir campo (compatibilidad con el resto del ecosistema).

No mutar ECST de dominio; PEC es familia `orchestration` (schema permisivo o ampliación documentada si hay validador estricto).

## 5. Cliente UI (`app.js`)

### 5.1 Flujo

```text
enviar()
  POST /api/interact
  si data.degraded → render response + marca «degradado»
  si data.emitted && data.event_id →
       estado pending en output
       poll GET /api/status?event_id=… cada POLL_MS (def. 1500)
       hasta completed|failed|timeout (def. 120s) o abort
  si no → render response sync (chat)
```

### 5.2 Render

| Estado | Output |
|--------|--------|
| chat | `response` |
| chat degradado | `response` + prefijo/sufijo `[degradado]` |
| execute pending/routed | acuse + línea `estado: pending\|routed` |
| execute completed | `message` del status + `process_name` / `process_status` si hay |
| execute failed / timeout | error explícito; no fingir éxito |

Constante: `POLL_MS` y `POLL_TIMEOUT_MS` en JS (sin config server).

## 6. Handler `kalma2-interact`

| Touchpoint | Cambio |
|------------|--------|
| Fallback `synthesize_mayeuta_response` | Añadir `degraded: true` en `data` |
| Chat vía Skill OK | `degraded: false` (o ausente) |
| Execute acuse | Añadir `correlation_id` (= `event_id`) en `data` (además de campos actuales) |
| Emisión evento | Sin cambio de schema ECST |

## 7. Criterios de aceptación

| ID | Criterio |
|----|----------|
| **AC1** | Chat sin CLI → `degraded:true` y UI lo marca |
| **AC2** | Execute allowlisted → `emitted:true` + `event_id` + `correlation_id` iguales |
| **AC3** | `GET /api/status` con UUID desconocido → 404 JSON |
| **AC4** | Tras route-domain lab, status ≥ `routed` |
| **AC5** | Proceso con `correlation_id` emite PEC que incluye ese campo; status → `completed` |
| **AC6** | Bridge no escribe bajo `.events/` |
| **AC7** | Smoke E2E: prompt execute desde UI (o curl) cierra lazo pending→routed/completed |

## 8. Fuera de alcance

- WebSockets/SSE
- Nuevo evento `Kalma2_Process_Resolved`
- Timeout CLI `mayeuta-llm` (D1)
- Ampliación allowlist
- Auth/TLS
- Lectura FS directa desde el browser
