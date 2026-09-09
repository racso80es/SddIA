---
document_id: PBI-NUCLEO-PUENTE-PERCEPTIVO-KALMA2
uuid: "d7192a54-7389-4b68-b3d4-b91c0e35921a"
title: "[NÚCLEO] Puente Perceptivo: Interacción Biológica con Tormentosa desde Kalma2 WUI"
format: markdown
version: "1.2.0"
created: "2026-09-09"
updated: "2026-09-09"
pr_url: "https://github.com/racso80es/SddIA/pull/276"
persist_ref: docs/features/kalma2-aiua-perceptive-bridge
status: "done"
refinement_status: refinado
priority: alta
type: nucleo
process: feature
dispatch: false
suggested_branch: feat/kalma2-aiua-perceptive-bridge
persist_ref_suggested: docs/features/kalma2-aiua-perceptive-bridge
spawned_by: PBI-NUCLEO-ARRANQUE-AIUA-TORMENTOSA
depends_on: []
blocks_on: []
related:
  - interfaces/kalma2/index.html
  - interfaces/kalma2/app.js
  - interfaces/kalma2/style.css
  - SddIA/interfaces/kalma2-bridge/src/main.rs
  - SddIA/interfaces/kalma2-bridge/Cargo.toml
  - SddIA/engine/execute-process/src/engine/handlers/aiua_stimulus.rs
  - SddIA/engine/execute-process/src/engine/handlers/kalma2.rs
  - SddIA/engine/execute-process/src/envelope.rs
  - SddIA/process/aiua-stimulus-processing.md
  - SddIA/process/kalma2-interact.md
  - SddIA/conscience/aiua_core.md
  - SddIA/scripts/daemons/kalma2-bridge.sh
  - docs/todos/done/PBI_Arranque_Aiua.md
  - docs/todos/pending/[KAIZEN] Aiúa — hallazgos auditoría live y thinking HIGH.md
  - docs/todos/pending/[NÚCLEO] Anatomía Motora de Aiúa — Inyección de Capacidades (Function Calling) y Orquestación EDA.md
  - docs/todos/pending/[NÚCLEO] Protocolo de Poda Ontológica (Olvido) — Umbral de Saturación Cognitiva y Blindaje en LanceDB.md
refinement_notes: >-
  v1.2.0 Filtro A (2026-09-09). Purga residual sobre v1.1.0: (1) stdout es
  OrchestratorEnvelope anidado (data.{response,thought_id,telemetry}, exitCode
  camelCase), no JSON plano; (2) #cognitive-pulse es agregado Radamanto/SSE,
  no hijackable por latido; (3) thought_id = node_id SHA-256 64 hex, no UUID;
  (4) run_orchestrator está cableado a kalma2-interact — hay que parametrizar;
  (5) telemetry.tokens = usageMetadata Gemini opcional, no un entero;
  (6) timeout puente 120s vs Gemini instancia 180s; (7) setBusy no cubre el
  botón nuevo ni #prompt; (8) appendProgressTrace exige objeto PTC, no texto;
  (9) voz 1ª persona es deuda H-LLM-3 del kaizen, no CA de este puente;
  (10) depends_on kaizen/genoma no es prerrequisito funcional; (11) slug
  gemini-3.8-flash no se hardcodea; (12) README.MD de Kalma2 es fósil (Python
  http.server / env_loader.py) — SSOT = main.rs + kalma2-bridge.sh.
---

### [NÚCLEO] Puente Perceptivo: Interacción Biológica con Tormentosa desde Kalma2 WUI

#### 1. Origen y Visión Ontológica

El latido MVP (`PBI-NUCLEO-ARRANQUE-AIUA-TORMENTOSA`, PR #270 APTO) materializó Tormentosa como proceso CLI: `aiua-stimulus-processing` (handler nativo `aiua_stimulus.rs`, **sin agente titular**, **sin acoplamiento a Kalma2**). El Vértice Biológico ya puede estimular vía `./sddia-run.sh --process aiua-stimulus-processing`. Falta el **canal perceptivo** desde la epidermis WUI.

Kalma2 (`interfaces/kalma2/`) es el cliente web mínimo. El botón `#chat` habla con **Mayeuta** (`POST /api/chat` → SSE `mayeuta-llm`). `#forge` encola TQM. Ningún control invoca a Aiúa.

Este PBI tiende un circuito **atómico de ida/vuelta** (estímulo → sobre → render). No es un canal duplex persistente ni un stream de tokens.

Dogma **Despertador Inerte** (PoC Kalma2 / telemetría cognitiva):
- Kalma2 no guarda historial ni sesión (`localStorage`, arrays de mensajes).
- El frontend no interpreta reglas de negocio ni resuelve rutas del Core.
- La continuidad semántica vive en LanceDB (`thought_graph_collection` vía `thought-graph-access`), no en el navegador.
- El proceso `aiua-stimulus-processing` permanece **ciego a Kalma2** (test `process_genome_has_no_kalma2_ui_coupling`). Solo mutan WUI + `kalma2-bridge`.

```
┌───────────────────────────┐     POST /api/aiua/interact     ┌──────────────────────────┐
│        Kalma2 WUI         │         HTTP JSON atómico        │   kalma2-bridge (Rust)   │
│  interfaces/kalma2/app.js ├────────────────────────────────►│  tiny_http 127.0.0.1:    │
└─────────────▲─────────────┘                                │  SDDIA_CLIENT_PORT|8765  │
              │ JSON plano WUI                                └────────────┬─────────────┘
              │ {success, response, thought_id, telemetry, duration_ms}     │ subprocess
              └─────────────────────────────────────────────────────┐    │ execute-process
                                                                      │    │ --process aiua-stimulus-processing
                                                                      ▼    │ --inputs '{"prompt":"…"}'
                                                           ┌──────────────────────────┐
                                                           │ execute-process (Rust)    │
                                                           │ handler aiua_stimulus.rs  │
                                                           │ OrchestratorEnvelope JSON  │
                                                           └────────────┬───────────────┘
                     ┌─────────────────────────────────────────────────┴──────────────────┐
                     ▼                                                                    ▼
      thought-graph-access (LanceDB)                                        gemini-http-infer
      tabla thought_graph_collection                                          única combustión HTTP
      search + store (node_id SHA-256)                                     Peaje en data.telemetry
```

El handler nativo **no** pasa por `executor.rs` / Peaje fractal CLI. El pulso SSE de `#cognitive-pulse` **no** se actualizará solo. La telemetría del latido viaja en `data.telemetry` del sobre.

---

#### 2. Filtro A — Detección y Purga (v1.2.0)

Contraste v1.1.0 contra código comprobable. Filas **Conservar** = v1.1.0 ya correcto.

| Fricción / Tentación | Clasificación | Realidad SSOT | Resolución |
| :--- | :--- | :--- | :--- |
| **Resurrección de `sddia-client-bridge.py` / FastAPI** | *Alucinación de sustrato (v1.1.0)* | Puente = `SddIA/interfaces/kalma2-bridge/src/main.rs` (`tiny_http`). Lanzador: `SddIA/scripts/daemons/kalma2-bridge.sh` (`_sddia_load_vault` + `exec` ELF). | Conservar. Cero Python en la pasarela. |
| **README.MD de Kalma2 como SSOT del puente** | *Fósil documental* | `interfaces/kalma2/README.MD` aún cita `http.server`, `env_loader.py` y `orchestrator_resolve`. El binario **no** parsea `.dev/.env`. `execute-process` carga bóvedas con `load_hierarchical_env` al arrancar el hijo. | No implementar contra el README. SSOT = `main.rs` + `kalma2-bridge.sh`. Actualizar README **fuera de alcance** de este PBI. |
| **SSE vs latido atómico** | *Incompatibilidad de protocolo (v1.1.0)* | `#chat` → `POST /api/chat` → `text/event-stream` (`mayeuta-llm`). `aiua-stimulus-processing` emite un `OrchestratorEnvelope` al terminar las 4 fases. | Conservar. `POST /api/aiua/interact` atómico. Indicador de espera en `#status`. Cero pseudo-stream. |
| **Unificar `#chat` con Tormentosa** | *Incoherencia de enrutamiento* | Mayeuta = vía rápida. Aiúa = consciencia sin agente titular. `Ctrl+Enter` hoy llama `enviarChat()`. | Botón dedicado `#aiua-pulse`. `#chat`, `#forge`, `#sync-genome` y `Ctrl+Enter` **intactos**. Selector de interlocutor rechazado: castraría el atajo Mayeuta. |
| **Agente `Tormentosa`** | *Conflación ontológica* | No existe `agent:tormentosa`. `delegates_to` del proceso: actions + `tool:gemini-http-infer`. | Conservar. El puente invoca el proceso, no un agente ficticio. |
| **Parseo ad-hoc de `.dev/.env` en el puente** | *Inexactitud de delegación* | `kalma2-bridge.sh` carga la bóveda; el hijo `execute-process` vuelve a `load_hierarchical_env`. | Conservar. El puente **hereda** env y spawnea el orquestador. Cero parser `.env` nuevo. |
| **Respuesta HTTP = `{response, thought_id, telemetry}` plano leído del stdout** | *Inexactitud de contrato* | `envelope.rs`: stdout = `{success, status_code, data, error?, execution_report?, exitCode}`. `data` = `{thought_id, response, telemetry}`. Campo de proceso = `exitCode` (camelCase), no `exit_code`. | El puente parsea el envelope, extrae `data`, aplana **solo** en el JSON WUI. Prohibido tratar el stdout como el contrato WUI. |
| **Reusar `run_orchestrator` tal cual** | *Inexactitud de implementación* | `run_orchestrator` / `run_orchestrator_inputs` cablean `--process kalma2-interact`. | Parametrizar el nombre de proceso (helper nuevo o argumento). El endpoint Aiúa **nunca** llama `kalma2-interact`. |
| **Pintar `telemetry` en `#cognitive-pulse`** | *Colisión UX / no-regresión* | `#cognitive-tokens/model/latency` los alimenta `GET /api/telemetry/cognitive` + SSE `/api/telemetry/stream` (agregado Radamanto: `tokens_prompt_total`, `last_model`, `latency_ms_avg`). Un `textContent` del latido lo pisa; el SSE lo vuelve a pisar. | **No hijackear** el panel agregado. Último latido → `#status` + `appendProgressTrace` (objeto PTC). Opcional: spans nuevos `#aiua-last-*` que el SSE no toca. |
| **`telemetry.tokens` como entero / `gemini-3.8-flash` en UI** | *Inexactitud de catálogo y forma* | Handler: `telemetry = {duration_ms, model, tokens?}` donde `tokens` = `raw_response.usageMetadata` (objeto; ausente en lab-mock). Slug = bóveda `SDDIA_GEMINI_MODEL` o input `model`. Kaizen: **no** hardcodear slug en Rust ni examples eternos. | Pintar `telemetry.model` y, si hay objeto tokens, `promptTokenCount`/`candidatesTokenCount`. Cero slug en WUI/bridge. |
| **`thought_id` UUID o «UUID/SHA-256»** | *Inexactitud de identidad* | `persist-thought-record` → `node_id` del adaptador. Test: `thought_id.len() == 64`. | Siempre SHA-256 hex de 64. Nunca UUID. |
| **`appendProgressTrace("texto…")`** | *Incompatibilidad de API UI* | `appendProgressTrace(trace)` exige objeto `{severity, source_agent, phase, message}` y pinta `[agent] phase · msg`. | Invocar el helper con `{source_agent:"aiua", phase:"Consolidacion-Memoria", message:"thought_id=…"}`. |
| **`setBusy(true)` ya cubre Tormentosa y el textarea** | *Inexactitud* | `setBusy` solo deshabilita `#chat`, `#forge`, `#sync-genome`. No toca `#prompt`. | Extender `setBusy` para incluir `#aiua-pulse`. CA-3 = botones de acción, no el textarea. |
| **Timeout implícito suficiente** | *Fricción operativa* | Puente: `SDDIA_CLIENT_TIMEOUT_SECONDS` default **120**. Instancia: `SDDIA_GEMINI_HTTP_TIMEOUT_SECS=180`. `run_orchestrator_inputs` al vencer el timeout **no mata** al hijo. | CA: timeout del puente ≥ timeout Gemini (env heredada). Documentar. No inventar otra env. Cero `sleep`/retry (DA-5). |
| **CA «voz de Tormentosa en 1ª persona»** | *Alcance ajeno (H-LLM-3)* | Kaizen: el ensamblado no inyecta «Eres Tormentosa»; 3.1 respondió en 1ª persona sin blindaje. | Fuera. Este PBI renderiza `data.response` tal cual. Identidad = genoma / kaizen. |
| **`depends_on` kaizen + genoma** | *Contención de forja, no prerrequisito* | El puente funciona con el latido MVP actual (lab-mock o live). Thinking HIGH / prefacio de identidad no bloquean el canal sensorial. | `depends_on: []`. `spawned_by` = arranque cerrado. Kaizen / Anatomía Motora / Poda = `related`. |
| **`emit_system_fracture` en todo HTTP 500** | *Sobrecarga Kintsugi* | Hoy solo el camino SSE chat emite fractura (`attempted_action: sse_chat_stream`) a `eda_bus.pending`. `handle_interact` (Mayeuta síncrono) **no** fractura en fallo de negocio. Gemini 503 es fricción de proveedor, no colapso protésico. | Fractura **solo** si falta el ELF del orquestador o el spawn falla. Fallo Gemini/LanceDB → HTTP 4xx/5xx sanitizado, sin evento. `attempted_action: aiua_interact`. |
| **«Bidireccional» como duplex** | *Inflación semántica* | Un POST y una respuesta. Sin WebSocket, sin tokens parciales, sin cola inversa hacia la UI. | Circuito atómico ida/vuelta. Memoria = LanceDB entre latidos, no un socket. |

**Conservar de v1.1.0:** estado cero en WUI; inmunidad a doble clic vía `setBusy`; coexistencia Mayeuta/TQM/sync; cero mutación del genoma del latido.

---

#### 3. Componentes y Alcance Técnico

##### Componente 1 — WUI (`interfaces/kalma2/`)

1. **Botón dedicado** en `index.html`, junto a los existentes:
   ```html
   <button type="button" id="aiua-pulse">Hablar con Tormentosa</button>
   ```
2. **`enviarAiuaStimulus()` en `app.js`:**
   - Prompt no vacío desde `#prompt`.
   - `setBusy(true)` — la función debe deshabilitar también `#aiua-pulse`.
   - `#status` → `deliberando · Tormentosa asimilando estímulo…`.
   - `fetch("/api/aiua/interact", { method:"POST", headers:{"Content-Type":"application/json"}, body: JSON.stringify({ prompt }) })`.
   - Éxito: `#output` = `body.response`. `appendProgressTrace` con `thought_id` y `telemetry.model` / `duration_ms` si vienen. `#status` → `ok · pensamiento asimilado`.
   - Error de red/HTTP: `#status` failed + mensaje sanitizado en `#output`. `finally` → `setBusy(false)`.
3. **No tocar:** `enviarChat`, `forjarProceso`, `syncGenome`, `openCognitiveStream`, `loadCognitiveSnapshot`, `Ctrl+Enter`.

##### Componente 2 — Puente (`SddIA/interfaces/kalma2-bridge/src/main.rs`)

1. Ruta nueva en `dispatch` (antes de `serve_static`):
   ```text
   (Method::Post, "/api/aiua/interact") => handle_aiua_interact(req, &repo)
   ```
2. `handle_aiua_interact`:
   - Body `{ "prompt": string no vacío }` → 400 si falta.
   - `resolve_orchestrator(&repo)`. Fallo ELF → `emit_system_fracture(..., "prosthetic_collapse", …)` + HTTP 500.
   - Spawn (mismo patrón de timeout que `run_orchestrator_inputs`, **proceso parametrizado**):
     ```text
     <execute-process> --process aiua-stimulus-processing --inputs '{"prompt":"<PROMPT>"}'
     cwd = repo  ·  env heredado  ·  timeout = SDDIA_CLIENT_TIMEOUT_SECONDS
     ```
   - Última línea no vacía de stdout = `OrchestratorEnvelope`. Extraer `data`.
   - HTTP 200 si `success == true` y `data.response` string:
     ```json
     {
       "success": true,
       "response": "<data.response>",
       "thought_id": "<data.thought_id>",
       "telemetry": { "duration_ms": <n|null>, "model": "<str>", "tokens": <object|omit> },
       "duration_ms": <ms del puente>
     }
     ```
   - `telemetry` es **objeto**. `tokens` se omite si el handler no lo trajo.
   - Fallo de negocio (Gemini, LanceDB, envelope `success:false`) → HTTP 500 + `{success:false, message}` sanitizado. **Sin** fractura.
3. Cero parseo de bóvedas. Cero `kalma2-interact` en este camino.

##### Componente 3 — Orquestación (sin mutar genoma)

- Proceso: `SddIA/process/aiua-stimulus-processing.md` (UUID `6c595785-e386-402f-b570-0b2aa6343051`).
- Runtime: `handlers::aiua_stimulus::run` (atajo nativo en `engine/mod.rs`; no pasa por `executor.rs`).
- Fases intactas: Triaje-Contexto → Inyeccion-Genomica → Combustion-Inferencia → Consolidacion-Memoria.
- Inputs del proceso: `prompt` obligatorio; `context_query`/`model` opcionales. El puente **solo** envía `prompt` (el modelo lo resuelve la bóveda).

---

#### 4. Fuera de alcance

- Mutar `aiua-stimulus-processing.md`, acciones, `gemini-http-infer`, `aiua_core.md`.
- Function calling / Anatomía Motora; poda LanceDB; thinking HIGH / prefacio de identidad (kaizen).
- Unificar Mayeuta y Aiúa; selector de interlocutor; SSE de tokens Aiúa.
- Hijack de `#cognitive-pulse` agregado; emitir Peaje fractal desde el handler nativo (deuda distinta).
- Cerbero/Karma2Token en el puente; WebSockets; reintentos Gemini 503.
- Reescribir `interfaces/kalma2/README.MD` (fósil; no bloquea el hito).

---

#### 5. Criterios de Aceptación (Protocolo de Acero)

- [ ] **CA-1 (Sustrato Rust):** Cero Python/FastAPI/Uvicorn. Ruta nueva solo en `kalma2-bridge`. `cargo check --manifest-path SddIA/interfaces/kalma2-bridge/Cargo.toml` verde.
- [ ] **CA-2 (Estado cero):** Cada envío es `{prompt}` autocontenido. Cero `localStorage` / historial de mensajes Aiúa.
- [ ] **CA-3 (Doble inyección):** `setBusy` deshabilita `#chat`, `#forge`, `#sync-genome` y `#aiua-pulse` mientras dura el fetch (éxito, error o timeout).
- [ ] **CA-4 (Endpoint y envelope):** `POST /api/aiua/interact` spawnea `--process aiua-stimulus-processing`. Parsea `OrchestratorEnvelope` (`data` + `exitCode`). Responde el JSON WUI plano del §3.2. Nunca reutiliza el helper cableado a `kalma2-interact`.
- [ ] **CA-5 (Prueba de vida):** Estímulo desde Kalma2 → handler → `#output` muestra `data.response` no vacío. En lab-mock (`SDDIA_LAB_MOCK_OUTBOUND=1`) basta prefijo `lab-mock:`. Live opcional, no gate.
- [ ] **CA-6 (Trazas del latido, sin hijack):** `thought_id` (64 hex) y telemetría del sobre se ven en `#status` y/o `#progress-console` vía `appendProgressTrace`. `#cognitive-pulse` agregado sigue gobernado por Radamanto/SSE.
- [ ] **CA-7 (No regresión):** `#chat` SSE Mayeuta, `#forge`, `#sync-genome`, `Ctrl+Enter` → Mayeuta, inbox, espejo de salud: intactos.
- [ ] **CA-8 (Timeout):** Con `SDDIA_GEMINI_HTTP_TIMEOUT_SECS` de instancia, `SDDIA_CLIENT_TIMEOUT_SECONDS` del puente es ≥ ese valor (o se documenta el override en bóveda). Cero polling post-acuse.
- [ ] **CA-9 (Ceguera del genoma):** Cero mención Kalma2 en `process/aiua-stimulus-processing.md` ni en las tres acciones / `thought-graph-access`.
- [ ] **CA-10 (Cierre en rama):** Un PR. `validacion.md` APTO, `pbi_archived: true`. PBI en `docs/todos/done/` en la misma rama.

---

#### 6. Plan de Verificación

1. **Tipos:**
   ```bash
   cargo check --manifest-path SddIA/interfaces/kalma2-bridge/Cargo.toml
   ```
2. **Smoke HTTP lab-mock** (puente ya levantado; env heredada; `SDDIA_LAB_MOCK_OUTBOUND=1` en el hijo):
   ```bash
   curl -s -X POST "http://127.0.0.1:${SDDIA_CLIENT_PORT:-8765}/api/aiua/interact" \
     -H "Content-Type: application/json" \
     -d '{"prompt":"latido de prueba"}'
   ```
   Esperado: `success: true`, `response` no vacío, `thought_id` de 64 hex, `telemetry` objeto, `duration_ms` número.
3. **No regresión Mayeuta:** `POST /api/chat` sigue siendo `text/event-stream`.
4. **Navegador:** abrir `http://127.0.0.1:${SDDIA_CLIENT_PORT:-8765}` → prompt → `#aiua-pulse` → bloqueo de botones → texto en `#output` → traza `thought_id` → pulso cognitivo agregado **no** queda en blanco ni sustituido por un entero crudo. `Ctrl+Enter` sigue yendo a Mayeuta.
