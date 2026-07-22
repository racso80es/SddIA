---
feature_name: kalma2-pasarela-asincrona-eda
created: "2026-07-22"
process: feature
base: main
scope: kalma2-pasarela-asincrona-eda
version_spec: "1.0.0"
uuid: c4e8a1b2-7f3d-4a9e-b6c1-2d8f0e5a9b47
status: dedalo_locked
document_id: PBI-044-KALMA2-PASARELA-ASINCRONA-EDA
pbi_uuid: 8c71b50f-7067-472a-a149-40041920b054
branch_name: feat/kalma2-pasarela-asincrona-eda
persist_ref: docs/features/kalma2-pasarela-asincrona-eda
laudo: spawn-detach-202-preassign-uuid
depends_on:
  - docs/features/kalma2-event-bus-integration
  - docs/features/kalma2-process-dispatch
  - docs/features/kalma2-full-cycle
  - docs/features/kaizen-kalma2-feature-cycle-observability
---

# Especificación — kalma2-pasarela-asincrona-eda

## 1. Topología de responsabilidades

```text
Cliente HTTP / UI Kalma2
  → POST /api/execute | /api/interact (mode=execute)
       kalma2-bridge (aduana inerte)
         ├─ valida prompt mínimo
         ├─ preasigna correlation_id (UUID v4)
         ├─ responde 202 + accepted  ← libera socket (R1)
         └─ spawn fondo: execute-process kalma2-interact
              └─ handlers::kalma2 (genoma)
                   └─ Kalma2_Process_Requested
                        event_id ≡ correlation_id preasignado (R3/Q3)
  → event-watcher → route-domain-event → TQM / ciclo  (R4 intacto)
  → Process_Execution_Completed @ eda_fractal.orchestration (R5)
  → Cliente: GET /api/status?event_id=<correlation_id>
```

**Invariantes:** bridge **nunca** escribe bajo `.events/**` (R2/Q1 veto). Emisión = genoma. Paths solo vía `SddIA/core/cumulo.paths.json`. Sin evento nuevo. Sin reabrir allowlist/despacho.

## 2. Laudos Dedalo (cierran D8 Mayeuta)

| Ref | Pregunta | Laudo | Justificación |
|-----|----------|-------|---------------|
| **L1** | Fire-and-forget sin write al bus | **Spawn + reaper en hilo** de `execute-process --process kalma2-interact`. HTTP **no** hace `join`/`output`/`wait` del ciclo. | Cumple Q1; preserva ceguera; reutiliza emisor existente. Cápsula ingest ciega = **no** en H1+H2 (over-engineering). |
| **L2** | Código HTTP del acuse | **202 Accepted** obligatorio en camino execute no bloqueante. | Q2; semántica async. Errores de validación/orquestador ausente → 400/500 síncronos (sin spawn). |
| **L3** | Origen `correlation_id` | Bridge genera UUID v4 **antes** del spawn; lo inyecta en `--inputs` como `correlation_id`. Handler honra ese id como `event_id` del ECST. | Q3; `correlation_id ≡ event_id`. Sin UUID paralelo. |
| **L4** | Plumb en genoma | **Diff mínimo** en `handlers/kalma2.rs`: si `process_inputs.correlation_id` es UUID no vacío, usarlo en `build_kalma2_process_event`; si ausente/ inválido, `Uuid::new_v4()` (compat legado). | No reescribe emisión ni allowlist; solo identidad del evento. |
| **L5** | Contrato UI | Tratar `status=="accepted"` + `correlation_id` como señal de poll (igual que hoy `emitted`+`event_id`). | H2; UI no espera `emitted` en el mismo request. |
| **L6** | `mode=chat` / SSE | **Sin cambio** (sigue síncrono / fuera Done). | AC-R5; D7. |
| **L7** | R6 Telegram | **Defer H3** — fuera Done mínimo. | Q4. |
| **L8** | Zombies / stdio del hijo | Tras `Command::spawn`, reaper en `thread::spawn` que solo hace `wait` (no bloquea HTTP). `stdout`/`stderr` → `Stdio::null()` (o log opcional vía env; default null). | Evita zombies; evita buffer pipe lleno. |

### Causa raíz empírica

`run_orchestrator_inputs` spawnea el motor y **bloquea** el request hasta stdout/timeout (`SDDIA_CLIENT_TIMEOUT_SECONDS`, default 120s). El bus ya despacha en background tras emitir; la aduana HTTP no.

## 3. Contratos

### 3.1 Acuse HTTP — `POST /api/execute` y `POST /api/interact` (`mode=execute`)

**Entrada** (sin cambio semántico):

```json
{ "prompt": "<texto no vacío>", "process": "<opcional>", "mode": "execute" }
```

(`mode` implícito en `/api/execute`.)

**Salida éxito (202):**

```json
{
  "success": true,
  "status": "accepted",
  "correlation_id": "<uuid>",
  "event_id": "<uuid>",
  "message": "intención aceptada; consultar GET /api/status",
  "duration_ms": 0
}
```

| Campo | Regla |
|-------|-------|
| `correlation_id` / `event_id` | Mismo UUID preasignado |
| `status` | Literal `"accepted"` |
| `duration_ms` | Tiempo hasta el acuse (esperado ≪ 50 ms) |
| `emitted` | **Ausente** en el acuse HTTP (emisión es async; no mentir) |

**Errores síncronos (sin spawn):**

| Caso | HTTP | Cuerpo |
|------|------|--------|
| prompt vacío / JSON inválido | 400 | `success:false`, `message` |
| orquestador no resoluble | 500 | `success:false`, `message` |
| fallo al `spawn` del hijo | 500 | `success:false`, `message` (socket aún no liberado como accepted) |

**Fuera de contrato H1+H2:** esperar envelope completo del orquestador (`data.emitted`, `seal`, fases) en el mismo request.

### 3.2 Inputs al genoma (spawn)

```json
{
  "prompt": "<trim>",
  "mode": "execute",
  "correlation_id": "<uuid preasignado>",
  "process": "<opcional si el cliente lo envió>"
}
```

### 3.3 Emisión ECST (genoma — L4)

| Antes | Después |
|-------|---------|
| `event_id = Uuid::new_v4()` siempre | `event_id = correlation_id` si UUID válido en inputs; else `new_v4()` |
| `correlation_id` en data del envelope = `event_id` | Igual (identidad preservada) |
| `event_type` | `Kalma2_Process_Requested` (sin cambio) |
| Write path | `eda_fractal.domain` vía helpers existentes (sin cambio de topología) |

Prohibido: `Kalma2_Interaction_Requested` u otro tipo paralelo.

### 3.4 `GET /api/status` (sin cambio de contrato)

Sigue proyectando pending→routed→completed/failed vía dominio + PEC en `eda_fractal.orchestration`. El cliente usa el `correlation_id` del acuse 202 como `event_id` de consulta. 404 transitorio post-acuse (antes de write genómico) es esperado; UI ya tolera 404 en poll.

## 4. Diseño de spawn (detalle L1/L8)

```text
handle_execute / interact(mode=execute):
  1. parse + validate prompt
  2. resolve_orchestrator → Err? 500
  3. cid = Uuid::new_v4()
  4. inputs = { prompt, mode:execute, correlation_id:cid, process? }
  5. Command::new(bin)
       .args(["--process","kalma2-interact","--inputs", inputs_json])
       .current_dir(repo)
       .stdout(Stdio::null()).stderr(Stdio::null())
       .spawn()
     → Err? 500
  6. thread::spawn { let _ = child.wait(); }   // reaper
  7. reply(202, accepted envelope con cid)
```

**Prohibido en bridge:** `fs::write` / create bajo paths EDA (`eda_bus.*`, `eda_fractal.*`); `run_orchestrator_inputs` con join en el camino execute.

**Compat:** `run_orchestrator_inputs` puede permanecer para `mode=chat` / caminos síncronos; execute **no** lo usa.

## 5. UI (`interfaces/kalma2/app.js`) — H2

`forjarProceso`:

1. `fetch /api/execute` → parse JSON.
2. Si `r.status === 202` (o `data.status === "accepted"`) y hay `correlation_id`|`event_id` → mostrar acuse breve + `pollStatus(id, …)`.
3. Rama legado `payload.emitted && payload.event_id` puede permanecer como fallback defensivo (no es el camino feliz post-feature).
4. No bloquear UI esperando envelope de emisión en el POST.

## 6. Superficie de mutación (Tekton)

| Artefacto | Acción | Genoma indexado |
|-----------|--------|-----------------|
| `SddIA/interfaces/kalma2-bridge/src/main.rs` | Desacople execute + 202 + UUID | No (interfaz) |
| `SddIA/engine/execute-process/src/engine/handlers/kalma2.rs` | Honrar `correlation_id` | No (handler código) |
| `interfaces/kalma2/app.js` | Poll tras accepted | No |
| Tests unitarios bridge/handler | Timing + UUID plumb | No |
| Eventos / subscriptions / allowlist | **Sin cambio** | — |

Forja vía `entity-manager` **no** requerida si no se mutan `.md` indexados del genoma.

## 7. Criterios de aceptación (Argos)

| ID | Criterio | Liga |
|----|----------|------|
| **AC-R1** | Smoke POST execute → HTTP 202, cuerpo `accepted`+`correlation_id`, `duration_ms` p99 &lt; 50 ms; request no espera Argos/cascada | O1 |
| **AC-R2** | Tras acuse: rastro `Kalma2_Process_Requested` con `event_id == correlation_id` **o** evidencia de spawn correlacionado; audit/grep: cero writes EDA desde crate `kalma2-bridge` | O2 |
| **AC-R3** | TQM/ciclo no regresa a baseline process-dispatch; orphan/DL no empeora | O4 |
| **AC-R4** | `GET /api/status?event_id=<cid>` proyecta hasta terminal vía PEC | O5 |
| **AC-R5** | Chat/SSE no mutados como Done | L6 |
| **AC-R6** | Documentado defer H3 | L7 |
| **AC-DONE-PBI** | `validacion.md` APTO + PBI en `done/` + `pbi_archived:true` mismo PR | O8 |

### Smokes / tests mínimos

1. **Timing:** N disparos lab locales; medir hasta cabecera/cuerpo 202 (sin await del hijo).
2. **Correlación:** spawn con cid fijo → archivo dominio con mismo `event_id` (o test unitario de `build_kalma2_process_event` / emit con cid).
3. **No-write-bus:** búsqueda estática en bridge: no APIs de write EDA nuevas; caminos execute no llaman helpers de sellado fractal.
4. **Regresión:** execute con `SDDIA_LAB_SKIP_GIT=1` (o equivalente lab) sigue emitiendo y status llega a terminal conocido.

## 8. Fuera de alcance (ratificado)

waiting-for-shell Cursor Agent · IOTA DLT · systemd · reescritura emisión/allowlist/process-dispatch · chat SSE · PBI-043 DI · PPR#136 F3 · H3 Telegram · cápsula ingest ciega nueva (salvo fallo demostrable de L1).

## 9. Riesgos

| Riesgo | Mitigación |
|--------|------------|
| 404 en poll antes de write genómico | UI ya reintenta; documentar ventana breve |
| Spawn OK pero hijo falla al emitir | Status→failed/DL vía nervio; acuse HTTP no miente `emitted` |
| UUID ignorado por handler | Test unitario L4 + smoke correlación |
| Zombie processes | Reaper L8 obligatorio |
| UI solo mira `emitted` | L5: rama `accepted` |

## 10. Handoff Tekton

Consumir este `spec.md` + `plan.md`. Ejecutar H1 luego H2. No tocar subscriptions ni inventar eventos. Git solo `skill:git-manager`. Semillas Kaizen/TODOs solo Cumulo / `Kaizen_Alert_Required`.
