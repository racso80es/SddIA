---
feature_name: kalma2-canal-telemetria-progreso
created: "2026-08-15"
process: feature
base: main
scope: kalma2-canal-telemetria-progreso
version_spec: "1.0.0"
uuid: c8f4a2e1-9d3b-4f67-a1c5-8e2b6d09f4a7
status: dedalo_locked
laudo: C-ephemeral-progress-leaf
agent: dedalo
branch_name: feat/kalma2-canal-telemetria-progreso
persist_ref: docs/features/kalma2-canal-telemetria-progreso
pbi_ref: docs/todos/pending/[OPERATIVO] PBI: Canal Asíncrono de Telemetría de Progreso y Observabilidad Activa para Interfaces Externas (Kalma2).md
document_id: PBI-KALMA2-CANAL-TELEMETRIA-PROGRESO
depends_on:
  - docs/features/kalma2-event-bus-integration
  - docs/todos/done/[ARQUITECTURA] PBI-044 — Pasarela asíncrona Kalma2 y desacople por bus de eventos.md
adjacent_not_merged:
  - docs/todos/pending/[Kaizen] Process_Execution_Completed — suscriptores y auditoría de circuito EDA.md
---

# Especificación — kalma2-canal-telemetria-progreso

## 1. Topología de responsabilidades

```text
execute-process (Rust)                ← evoluciona (emit_progress_trace fire-and-forget)
   └─ escribe PTC bajo eda_fractal.progress   ← hoja NUEVA (≠ telemetry peaje)
kalma2-bridge (Rust)                  ← evoluciona (SSE progreso; status intacto)
   ├─ GET /api/status                 ← SIN cambio semántico (veredicto)
   └─ GET /api/progress/stream        ← NUEVO (progreso; ≠ SSE chat)
interfaces/kalma2/{app.js,css,html}   ← evoluciona (consola + dual-canal)
event-sweeper / runtime               ← evoluciona (poda hoja progress)
familia telemetry peaje               ← INTANGIBLE
eda_fractal.domain / orchestration    ← INTANGIBLE (salvo lectura status ya existente)
```

Invariantes:

1. Orquestador **no** conoce bridge/WUI (ceguera espacial).
2. Bridge **solo lee** la hoja `progress`; **nunca** escribe trazas ni interpreta fases de negocio.
3. `GET /api/status` = veredicto/correlación (laudo PBI-044 / `kalma2-event-bus-integration`). El stream de progreso es **complementario**, no sustituto.
4. SSE de chat LLM (`Accept: text/event-stream` en interact) ≠ bus de progreso.

## 2. Laudos Dedalo (cierran abiertos Mayeuta D7)

| Ref | Pregunta | Laudo | Justificación |
|-----|----------|-------|---------------|
| **L1** | Leaf Cúmulo | **Nueva hoja** `eda_fractal.progress` → `./.events/progress` | Opción C Mayeuta; I1/I4. Prohibido `.SddIA/events/`, `eda_fractal.telemetry`, `eda_fractal.domain`. Sin `*_subscriptions` (no fan-out). |
| **L2** | ECST vs schema efímero | **Schema no-ECST** — cápsula **Progress Trace Capsule (PTC)** | Trinity ECST = `telemetry\|orchestration\|domain` (`events-contract` v1.1.0). Progreso UI no es Clase de Evento ni peaje. No catalogar bajo `SddIA/events/`. |
| **L3** | Dónde vive el contrato | Norma táctica bajo `directories.library_norms`: `progress-trace-contract.md` (+ schema JSON colindante opcional **fuera** de `capability_contracts`) | I2: no capability-contract; no path bajo `.SddIA/`. |
| **L4** | Join key Kalma2 | **`correlation_id`** (= `event_id` de `Kalma2_Process_Requested`, laudo L3 event-bus) | Homologa PBI `process_id` → `correlation_id`. Emisión solo si `process_inputs.correlation_id` no vacío. |
| **L5** | Nombre lógico de traza | Campo `trace_id` (UUID v4); no `event_id` ECST en raíz | Evita colisión ontológica con bus. |
| **L6** | Endpoint bridge | **`GET /api/progress/stream?correlation_id=<uuid>`** `text/event-stream` | I6: distinto del SSE chat. Query `process_id` del PBI **rechazada** como alias público (usar solo `correlation_id`). |
| **L7** | Mecanismo FS | Lectura inicial (replay) + watch (`notify`/poll acotado) sobre `{progress}/{correlation_id}/` | Bridge proyecta bytes JSON → SSE; sin business coupling. |
| **L8** | Dual-canal UI | Poll `/api/status` **permanece**; consola consume `/api/progress/stream` en paralelo | I5: overreach «sustituir polling» **rechazado**. |
| **L9** | Sweeper | Extender perímetro del daemon `event-sweeper` / `eda_sweep` con poda de hoja `progress` (TTL + correlación terminal PEC) | No mezclar con purge peaje/`route-telemetry`. |
| **L10** | Latencia &lt;100 ms | **AC interfaz Kalma2 (AC5)**; no gate Core ni peaje | I7. |
| **L11** | Kaizen PEC 404 | **Fuera de alcance** | I8 / `adjacent_not_merged`. |

## 3. Contrato PTC (Progress Trace Capsule)

### 3.1 Envelope runtime (JSON UTF-8, un archivo = una traza)

```json
{
  "trace_id": "<uuid-v4>",
  "correlation_id": "<uuid-v4>",
  "timestamp": "<ISO-8601 UTC>",
  "phase": "spec | clarify | plan | implementation | validation | closure",
  "severity": "info | warn | error | kaizen_alert",
  "source_agent": "cerbero | mayeuta | dedalo | tekton | argos | cumulo | orchestrator | <indexado>",
  "message": "<texto legible del hito>",
  "metadata": {}
}
```

| Campo | Obligatorio | Regla |
|-------|:-----------:|-------|
| `trace_id` | Sí | UUID v4 de la cápsula |
| `correlation_id` | Sí | Saga Kalma2 / ejecución |
| `timestamp` | Sí | ISO-8601 UTC |
| `phase` | Sí | Enum UI cerrado (§3.2); **no** copiar literal libre del `phase.name` del proceso sin mapear |
| `severity` | Sí | Enum cerrado |
| `source_agent` | Sí | Badge UI; **≠** `emitter_agent` ECST |
| `message` | Sí | No vacío tras trim |
| `metadata` | No | Objeto; extensible (p. ej. `process_name`, `phase_name_raw`) |

**Prohibido en raíz PTC:** `event_type`, `emitter_agent`, `payload`, `delivery_state`, `event_family`. Si algún día se exigiera ECST, sería feature distinta que muta `events-contract` (fuera de este PBI).

### 3.2 Mapa fase proceso → `phase` UI

| `phase.name` (feature / bug-fix / refactorization) | `phase` PTC |
|------------------------------------------------------|-------------|
| Inicialización Git / equivalentes init | `spec` |
| Estabilización de Requisitos / Clarify* | `clarify` |
| Diseño de Blueprint / Diseño del fix | `plan` |
| Ejecución | `implementation` |
| Verificación | `validation` |
| Cierre documental* / Cierre de entrega / finalize* | `closure` |
| Desconocido | `implementation` + `metadata.phase_name_raw` |

Emisión mínima: **inicio y fin** de cada fase orquestada cuando exista `correlation_id` (chispazos; no flood de cada cápsula hija salvo que `metadata` lo justifique en iteración posterior).

### 3.3 Layout FS (vía Cúmulo)

```text
{repo}/{eda_fractal.progress}/{correlation_id}/{trace_id}.json
```

- Crear directorio de correlación bajo demanda.
- Escritura best-effort (crear dirs + write); **cualquier error IO se traga** (log local opcional; nunca falla `execute-process`).
- `.events/` ya está en `.gitignore` → sin cambio aduana git para residuos runtime.

### 3.4 Mutación topología (forja Tekton, no Dedalo)

En `SddIA/core/cumulo.paths.json` → bloque `eda_fractal`:

```json
"progress": "./.events/progress"
```

Sin clave de subscriptions. Actualizar lectores de topología (`BusTopology` / helpers bridge) para resolver la hoja. Registrar hito en `directories.evolution` vinculando `uuid` del PBI/feature.

## 4. Emisión Core (`execute-process`)

| Aspecto | Contrato |
|---------|----------|
| API lógica | `emit_progress_trace(...)` en módulo orquestación (p. ej. junto a peaje **pero sin** llamar `route-telemetry`) |
| Trigger | Barreras de fase en `executor` (entrada/salida) si `correlation_id` presente |
| `source_agent` | Agente de `delegates_to` si hay; si no, `orchestrator` |
| Fallo bridge | Irrelevante: no hay push HTTP desde Core |
| Grade S+ | Sin `unwrap`/`expect` en path de emisión; warnings cero en touchpoints tocados |

**Prohibido:** emitir PTC a `eda_fractal.telemetry`, encolar en `eda_bus.pending`, o registrar en `telemetry_compliance.emitted_registry`.

## 5. Contratos HTTP (bridge)

### 5.1 `GET /api/status` — sin cambio de semántica

Veredicto terminal / proyección correlación (precedente event-bus / full-cycle). UI sigue pollando.

### 5.2 `GET /api/progress/stream?correlation_id=<uuid>` — nuevo

| Aspecto | Contrato |
|---------|----------|
| Auth | Localhost only (paridad bridge) |
| 400 | `correlation_id` ausente o no UUID |
| 200 | `Content-Type: text/event-stream; charset=utf-8` |
| Replay | Tras abrir: emitir SSE por cada `*.json` existente bajo el subdir (orden por `timestamp` / nombre) |
| Live | Watch del subdir; cada alta → un evento SSE |
| Frame | `event: progress\ndata: <PTC json compacto>\n\n` |
| Heartbeat | Comentarios SSE periódicos opcionales (`: ping`) para proxies; no son trazas |
| Cierre | Cliente cierra al veredicto terminal del poll status o timeout UI; servidor tolera drop |

Routing (orden):

```text
POST /api/interact          → existente (chat SSE interno si aplica)
GET  /api/status            → existente
GET  /api/progress/stream   → nuevo (ANTES de static)
GET  /*                     → static
```

### 5.3 SSE chat vs progreso

| Canal | Origen | Propósito |
|-------|--------|-----------|
| Chat stream | LLM / interact | Tokens conversación |
| Progress stream | Hoja `progress` | Chispazos de fase |

Prohibido multiplexar ambos en el mismo handler.

## 6. WUI Kalma2

```text
enviar() / execute path
  POST /api/interact
  si emitted && correlation_id|event_id →
       poll GET /api/status (sin eliminar)
       EventSource GET /api/progress/stream?correlation_id=…
       consola: append cromático por severity + badge [agent]
  al completed|failed|timeout → close EventSource
```

| Severity | Tratamiento visual (orientativo) |
|----------|----------------------------------|
| `info` | neutro / azul frío |
| `warn` | ámbar |
| `error` | rojo |
| `kaizen_alert` | acento distinto (no reutilizar solo “error”) |

AC refresco &lt;100 ms = medición interfaz (escritura FS → paint); fallo de latencia **no** tumba ejecución Core.

## 7. Higiene (sweeper)

| Regla | Acción |
|-------|--------|
| Existe PEC terminal (`completed`/`failed`) con mismo `correlation_id` | Purgar árbol `{progress}/{correlation_id}/` |
| Edad de archivos/dir &gt; TTL (def. configurable; sugerido 24h) | Purgar aunque no haya PEC (huérfanos) |
| Hoja `telemetry` peaje | **No tocar** en esta lógica |

Semillas Kaizen por fallos de poda: solo vía `agent:cumulo` / `Kaizen_Alert_Required` — Tekton no escribe `docs/todos/`.

## 8. Criterios de aceptación (mapeo AC Mayeuta)

| ID | Criterio | Gate |
|----|----------|------|
| **AC1** | Bridge caído ⇒ `execute-process` OK sin reintento bloqueante por progreso | Smoke: emitir con bridge off |
| **AC2** | Ninguna PTC en `eda_fractal.domain` ni anclaje DLT | Inspección paths / tests |
| **AC3** | Ninguna PTC en fan-out peaje / `route-telemetry` | Aislamiento hoja + asserts |
| **AC4** | `/api/status` intacto; progreso = canal adicional | Contrato + UI dual |
| **AC5** | Latencia WUI &lt;100 ms = AC interfaz, no Core | Medición manual/lab UI |
| **AC6** | Sin residuos tras poda alineada a cierre/TTL | Sweeper smoke |
| **AC7** | Rust touchpoints sin panics/warnings | `cargo check`/`test` perímetro |

## 9. Touchpoints de forja (Tekton)

| Artefacto | Cambio |
|-----------|--------|
| `SddIA/core/cumulo.paths.json` | `eda_fractal.progress` |
| `SddIA/library/norms/progress-trace-contract.md` | Norma PTC (vía forja entidad/norm) |
| `SddIA/engine/execute-process` | `emit_progress_trace` + ganchos fase |
| `SddIA/interfaces/kalma2-bridge` | Ruta SSE + watch FS vía Cúmulo |
| `interfaces/kalma2/app.js` (+ css/html) | Consola + EventSource dual |
| `sddia_daemon_runtime` / `event-sweeper` | Poda `progress` |
| `SddIA/evolution/` | Hito topología + feature uuid |
| README Kalma2 | Documentar dual-canal; **no** «sustituye status» |

## 10. Fuera de alcance

- Sustituir poll de `/api/status` por SSE de progreso
- Reutilizar familia `telemetry` / `Raw_Execution_Finished` / `Daemon_Heartbeat`
- Schema bajo `capability_contracts` o `.SddIA/`
- Cuarta familia ECST en `events-contract`
- Liquidar Kaizen PEC suscriptores / 404 post-purge
- Auth/TLS, historial durable de trazas, WebSockets
- Latencia &lt;100 ms como invariante del Core
