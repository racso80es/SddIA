---
feature_name: kalma2-aiua-perceptive-bridge
created: "2026-09-09"
process: feature
base: main
scope: instance-ui
branch_name: feat/kalma2-aiua-perceptive-bridge
persist_ref: docs/features/kalma2-aiua-perceptive-bridge
execution_id: "f075b5ea-aac3-47cb-b5ba-a18c5431a7e5"
document_id: PBI-NUCLEO-PUENTE-PERCEPTIVO-KALMA2
---

# Spec — kalma2-aiua-perceptive-bridge

## 1. WUI (`interfaces/kalma2/`)

`index.html`: botón `id="aiua-pulse"` en `.actions`, junto a los existentes.

`app.js`:

- `setBusy(busy)` también deshabilita `#aiua-pulse`.
- `enviarAiuaStimulus()`:
  - prompt = `#prompt`.value trim; vacío → return.
  - `setBusy(true)`; `#status` = `deliberando · Tormentosa asimilando estímulo…`.
  - `POST /api/aiua/interact` body `{prompt}` `Content-Type: application/json`.
  - 200 + `body.success`: `#output` = `body.response`; `#status` = `ok · pensamiento asimilado`; `appendProgressTrace({source_agent:"aiua", phase:"Consolidacion-Memoria", message:"thought_id=… model=… duration_ms=…"})`.
  - error red/HTTP: `#status` failed + `#output` mensaje sanitizado (`body.message` o status).
  - `finally` → `setBusy(false)`.
- Listener click `#aiua-pulse`. No tocar `enviarChat`, `forjarProceso`, `syncGenome`, streams, `Ctrl+Enter`.
- Cero `localStorage`. Cero slug de modelo.

Opcional: spans `#aiua-last-thought` / `#aiua-last-model` fuera de `#cognitive-pulse`.

## 2. Puente (`SddIA/interfaces/kalma2-bridge/src/main.rs`)

### 2.1 Spawn parametrizado

`run_orchestrator_inputs(repo, bin, process, inputs)` — `process` es argumento. `run_orchestrator` (Mayeuta síncrono) llama con `"kalma2-interact"`. Camino Aiúa llama con `"aiua-stimulus-processing"`.

Timeout: `max(SDDIA_CLIENT_TIMEOUT_SECONDS unwrap_or 120, SDDIA_GEMINI_HTTP_TIMEOUT_SECS parseado si presente)`. El loop de espera del join **no** es polling post-acuse CLI; es el wait del hijo HTTP.

### 2.2 Ruta

`dispatch`, antes de `serve_static`:

```text
(Method::Post, "/api/aiua/interact") => handle_aiua_interact(req, &repo)
```

### 2.3 `handle_aiua_interact`

1. Body `{prompt: string no vacío}` → 400 si falta.
2. `resolve_orchestrator`. Fallo ELF → `emit_system_fracture(..., "prosthetic_collapse", …, attempted_action: "aiua_interact")` + HTTP 500.
3. Spawn. Fallo spawn → misma fractura. Timeout / envelope `success:false` / Gemini / LanceDB → HTTP 500 `{success:false, message}` sanitizado, **sin** fractura.
4. Última línea no vacía de stdout = envelope. Extraer `data`.
5. HTTP 200 si `success == true` y `data.response` string:

```json
{
  "success": true,
  "response": "<data.response>",
  "thought_id": "<data.thought_id>",
  "telemetry": { "duration_ms": <n|null>, "model": "<str>", "tokens": <object|omit> },
  "duration_ms": <ms del puente>
}
```

`emit_system_fracture` debe aceptar `attempted_action` (hoy hardcodea `sse_chat_stream`). Camino chat SSE no cambia su valor.

Cero parseo de bóvedas. Cero `kalma2-interact` en este handler.

## 3. Genoma (no tocar)

Proceso UUID `6c595785-e386-402f-b570-0b2aa6343051`. Handler `aiua_stimulus::run`. Inputs puente: solo `prompt`.

## 4. Tests

- `cargo check --manifest-path SddIA/interfaces/kalma2-bridge/Cargo.toml`
- Estáticos: ruta en `dispatch` antes de `serve_static`; handler no contiene `kalma2-interact`; helper genérico acepta process name.
- Unit: flatten envelope → JSON WUI; timeout ≥ Gemini env; `thought_id` 64 hex si presente.
- Grep: `SddIA/process/aiua-stimulus-processing.md` + tres acciones + `thought-graph-access` sin "kalma2"/"Kalma2".
- Smoke lab-mock (manual, no gate CI): curl POST con puente levantado + `SDDIA_LAB_MOCK_OUTBOUND=1`.
