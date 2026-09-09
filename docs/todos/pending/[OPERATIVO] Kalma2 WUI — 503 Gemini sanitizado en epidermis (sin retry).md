---
document_id: PBI-OPERATIVO-KALMA2-AIUA-503-SANITIZE
uuid: "40510b90-f1f9-4bab-82be-052de6a080e4"
title: "[OPERATIVO] Kalma2 WUI — 503 Gemini sanitizado en epidermis (sin retry)"
format: markdown
version: "1.2.0"
created: "2026-09-09"
updated: "2026-09-09"
status: "abierto"
refinement_status: refinado
refined: true
priority: media
type: operativo
process: bug-fix
dispatch: false
suggested_branch: fix/kalma2-wui-aiua-503-sanitize
persist_ref_suggested: docs/fixes/kalma2-wui-aiua-503-sanitize
spawned_by: PBI-FIX-KALMA2-BRIDGE-AIUA-ROUTE-STALE-ELF
incident_ref: "Auditoría empírica WUI 2026-09-09 — 503 Gemini UNAVAILABLE crudo en WUI post-recycle"
source_audit: >-
  2026-09-09T15:25 CEST auditoría empírica kalma2-wui-tormentosa-chat-20260909.md §6.
  Host: kalma2-bridge PID 1244544, ELF release mtime 15:17:48 CEST.
  1ª pulsación #aiua-pulse: latido OK. 2ª pulsación: HTTP 500 en puente por 503 upstream de Gemini.
  Output WUI: '[error] http-status-503: {"error":{"code":503,"message":"This model is currently experiencing high demand. Spikes in demand are usually temporary. Please try again later.","status":"UNAVAILABLE"}}'.
  Causa: sanitize_bridge_message solo normaliza newlines a espacios y recorta a 240 caracteres; no clasifica 503 y deja pasar el JSON crudo del proveedor a la WUI.
review_notes: >-
  v1.0.0 borrador: deslinde correcto (no 404 ni colapso de orquestador) y veto DA-5, con citas y heurística débiles.
  v1.1.0: rutas a done/, L-FRACTURE vs DA-5, mensaje canónico, app.js intacto, gates KALMA-503-CA*.
  v1.2.0 Filtro A contra código y SSOT: (1) longitudes 151/169/92 eran inexactas — JSON Google compacto 171, prefijo 17, cadena completa 188, canónico 91;
  (2) el puente escribe el campo JSON `message`, no `response.message`;
  (3) la cita exacta «503 Gemini ≠ CA de Core. Sin retry en handler.» vive en aiua-audit-findings/clarify.md, no en el Kaizen §2;
  (4) tools-contract vigente es 1.5.0; el mapeo http-status-{code} es de gemini-http-infer, no del contrato;
  (5) heurística: prefijo http-status-503 es suficiente; 503 suelto no casa (test flatten_aiua_wui_rejects_business_failure);
  (6) grep de `loop` como veto es trampa (Rust ya usa loop/for); (7) cargo test --bin kalma2-bridge;
  (8) timeout motor / subproceso falló no pasan por sanitize_bridge_message.
architectural_constraints:
  - A-NO-RETRY-BACKOFF-DA5
  - A-EPIDERMAL-SANITIZATION-BRIDGE
  - A-INTACT-GENOME-TOOLS-DA2
  - A-INTACT-FRONTEND-APPJS
  - A-NO-MULTI-LLM-FAILOVER-SCOPE
  - A-NO-FRACTURE-OVERLOAD-UPSTREAM
  - A-HERMETIC-TESTS-NO-LIVE
  - A-LOGICAL-PATHS-NOT-FILE-URI
  - A-PHYSICAL-VS-DOCUMENTAL-CLOSURE
  - A-PREFIX-503-SUFFICIENT
  - A-NO-BARE-503-MATCH
gates_this_wave:
  - KALMA-503-CA1
  - KALMA-503-CA2
  - KALMA-503-CA3
  - KALMA-503-CA4
  - KALMA-503-CA5
  - KALMA-503-CA6
  - KALMA-503-CA-CI
related:
  - docs/audits/kalma2-wui-tormentosa-chat-20260909.md
  - docs/todos/done/[FIX] kalma2-bridge ELF release fósil — POST api-aiua-interact 404.md
  - docs/todos/done/[NÚCLEO] Puente Perceptivo: Interacción Biológica con Tormentosa desde Kalma2 WUI.md
  - docs/todos/done/[KAIZEN] Aiúa — hallazgos auditoría live y thinking HIGH.md
  - docs/features/kalma2-aiua-perceptive-bridge/clarify.md
  - docs/features/aiua-audit-findings/clarify.md
  - docs/todos/kitchen/PBI-MULTI-LLM-ROUTER.md
  - SddIA/interfaces/kalma2-bridge/src/main.rs
  - SddIA/interfaces/kalma2-bridge/Cargo.toml
  - SddIA/tools/gemini-http-infer/src/main.rs
  - interfaces/kalma2/app.js
related_pbis:
  - id: PBI-FIX-KALMA2-BRIDGE-AIUA-ROUTE-STALE-ELF
    rol: "Predecesor directo y origen del incidente. Resolvió físicamente el 404 reciclado del daemon (PID 1244544). La segunda pulsación reveló la combustión saturada y el blob JSON en epidermis."
  - id: PBI-NUCLEO-PUENTE-PERCEPTIVO-KALMA2
    rol: "Origen de la ruta POST /api/aiua/interact, flatten_aiua_wui y sanitize_bridge_message. Estableció el principio de no acoplar el genoma del latido a la epidermis WUI."
  - id: PBI-AIUA-AUDIT-FINDINGS-20260908
    rol: "SSOT de la cita «503 Gemini ≠ CA de Core. Sin retry en handler.» en docs/features/aiua-audit-findings/clarify.md (Filtro A). El Kaizen done formula la misma política con otra redacción."
  - id: PBI-MULTI-LLM-ROUTER
    rol: "Kitchen: failover multi-proveedor ante 503/429. Fuera del alcance de este bug-fix de epidermis."
  - id: PBI-FIX-FRACTURE-64f37c7f7b34
    rol: "Incidente contemporáneo en el mismo daemon: fractura por ausencia de mayeuta-llm en /api/chat. Ortogonal."
---

# [OPERATIVO] Kalma2 WUI — 503 Gemini sanitizado en epidermis (sin retry)

> **Refinamiento v1.2.0 (Filtro A).** Incidente residual de la auditoría biológica post-recycle (PID `1244544`): `POST /api/aiua/interact` despachó con éxito a través de `handle_aiua_interact` hacia `aiua-stimulus-processing` y `gemini-http-infer`. **No** es 404 de ruta (cerrado en `PBI-FIX-KALMA2-BRIDGE-AIUA-ROUTE-STALE-ELF`). **No** es colapso protésico de orquestador ni fractura Kintsugi. Se trata de saturación upstream de Gemini (`HTTP 503 UNAVAILABLE` / high demand). Defecto actual: `sanitize_bridge_message` en `kalma2-bridge` solo reemplaza saltos de línea y corta en 240 caracteres; la cadena `http-status-503: {json}` del incidente mide **188 caracteres** (< 240), así que llega cruda al Vértice en `#output` y ensucia `#status`.
>
> **Solución:** clasificar y transformar el error 503 en la epidermis del puente (`kalma2-bridge`), devolviendo un mensaje canónico limpio y estable en castellano (91 caracteres, umbral ≤120, sin llaves JSON). Veto estricto DA-5: **cero** `sleep`/`retry`/polling. `interfaces/kalma2/app.js` y `gemini-http-infer` permanecen **intactos**.

---

## 1. Incidente y evidencia empírica

Durante la prueba biológica post-recycle (~15:25 CEST, [kalma2-wui-tormentosa-chat-20260909.md](../../audits/kalma2-wui-tormentosa-chat-20260909.md) §6), tras iniciar el daemon `kalma2-bridge` con el nuevo ELF release (PID `1244544`, mtime 15:17:48 CEST):

1. **Primera pulsación `#aiua-pulse`:** latido exitoso (`HTTP 200`, respuesta de Tormentosa renderizada en el DOM).
2. **Segunda pulsación `#aiua-pulse`:** fallo de combustión en proveedor upstream. `#output` mostró:

```text
[error] http-status-503: {"error":{"code":503,"message":"This model is currently experiencing high demand. Spikes in demand are usually temporary. Please try again later.","status":"UNAVAILABLE"}}
```

Y el elemento `#status` recibió el recorte de `app.js` (`String(msg).slice(0, 120)`):

```text
failed · http-status-503: {"error":{"code":503,"message":"This model is currently experiencing high demand. Spikes in demand are 
```

### Diagnóstico de capas

- **Ruta y socket:** `POST /api/aiua/interact` resolvió correctamente (status HTTP del puente `500`, no `404`). El `503` vive en el *texto* de `message`, no como status HTTP reenviado.
- **Orquestador y proceso:** `handle_aiua_interact` ejecutó `execute-process --process aiua-stimulus-processing`. El handler nativo corre RAG (`thought-graph-access`) antes de inferir; en este incidente la falla fue la combustión, no LanceDB.
- **Herramienta:** `gemini-http-infer` contactó a Google AI Studio; `map_http_error_body(503, &body)` formateó `http-status-503: {body}` (`body.to_string()` compacto, sin espacios). `emit_v2` coloca esa cadena en `feedback` y `error`. `infer_gemini` propaga `body.error` → `OrchestratorEnvelope.failure` → stdout `{success:false, error:"http-status-503: …"}`.
- **Defecto localizado:** `flatten_aiua_wui` extrae `envelope.error` (fallback `message`) y lo pasa a `sanitize_bridge_message`. Sin clasificación de 503, el JSON crudo se inyectó en el campo HTTP **`message`** del puente y emergió en la WUI.

---

## 2. Cuadro forense (Filtro A) — Hechos vs Tentaciones e Inexactitudes

| Tentación / Inexactitud | Clasificación | Realidad SSOT | Resolución v1.2.0 |
| :--- | :--- | :--- | :--- |
| **Reintentar 503 en handler/cápsula** | *Veto DA-5* | `aiua-audit-findings/clarify.md` Filtro A: «503 Gemini ≠ CA de Core. Sin retry en handler.» Kaizen done §2: «No es CA de Core. No reintentar en bucle (DA-5).» Clarify puente `L-TIMEOUT`: cero `sleep`/retry post-acuse. | **Prohibido** `sleep`, bucles de reintento, colas o polling en el backend o en el puente. |
| **Atribución de `L-FRACTURE` a `aiua-audit-findings`** | *Cita normativa* | `L-FRACTURE` está en `docs/features/kalma2-aiua-perceptive-bridge/clarify.md`: fractura solo si falta ELF orquestador o el spawn falla; Gemini/`success:false` → HTTP 5xx sanitizado, sin evento. | Cita intacta. Filtro A de Aiúa aporta el veto DA-5; no mezclar con Kintsugi. |
| **Citar el Kaizen como sede de la frase exacta** | *Paráfrasis* | La frase literal está en `aiua-audit-findings/clarify.md`. El Kaizen es política equivalente, otra redacción. | `related_pbis` PBI-AIUA-AUDIT-FINDINGS apunta al clarify; el Kaizen queda como correlato. |
| **Failover a otro proveedor en este PBI** | *Alcance kitchen* | `PBI-MULTI-LLM-ROUTER` está en `docs/todos/kitchen/`. | Mantener como `related`. Cero failover. |
| **Mutar `gemini-http-infer` para humanizar 503** | *Forja de genoma innecesaria (DA-2)* | Tool bajo `directories.tools`. El formato `http-status-{code}: {body}` lo implementa `map_http_error_body` en la tool (contrato vigente `tools-contract` **v1.5.0**; v1.2.0 era cita fósil). Debe seguir reportando el payload técnico. | Tool **intacta**. Humanización = epidermis `kalma2-bridge`. |
| **Mutar `interfaces/kalma2/app.js`** | *Redundancia* | `enviarAiuaStimulus`: `const msg = body.message \|\| body.error \|\| …`; `setStatus("failed · " + String(msg).slice(0, 120))`; `out.value = '[error] ' + msg`. | Frontend **intacto**. |
| **Campo `response.message`** | *Inexactitud v1.1.0* | Éxito: JSON con `response`. Fallo: `{success:false, message:…}` sin clave `response`. | Hablar de `message` del puente. |
| **Longitudes 151 / 169 / 92** | *Inexactitud aritmética v1.1.0* | JSON Google compacto del incidente: **171**. Prefijo `http-status-503: `: **17**. Cadena completa: **188**. Canónico: **91**. 188 < 240 → no hay truncado. | Cifras corregidas. El argumento cualitativo (cabe entero) se sostiene. |
| **Emitir `System_Fracture_Detected` por 503** | *Sobrecarga de Kintsugi* | `L-FRACTURE`: saturación de LLM externo ≠ fractura local. | HTTP 500 + `{success:false, message:"…"}` **sin** evento. |
| **Conjunción `http-status-503` ∧ `UNAVAILABLE` ∧ `high demand`** | *Falso negativo* | Google puede variar el inglés. El prefijo es el contrato de la tool para cualquier 503 no-catálogo. | **Señal primaria suficiente:** substring `http-status-503`. **No** casar `503` suelto (preserva el test `error: "gemini 503"`). **Señal secundaria:** `"code":503` / `"code": 503` **y** marcador de indisponibilidad, solo si falta el prefijo. |
| **Veto de la palabra `loop` en el diff** | *Trampa de verificación* | `main.rs` ya contiene `loop`/`for`. CA-3 no prohíbe iteración de código. | Auditar ausencia de `sleep`, retry/backoff y polling de reintento. No grep de `loop`. |
| **Tests de flatten cubren timeout / join** | *Alcance de test* | `OrchestratorOutcome::Timeout` y `JoinFail` responden JSON cableado **sin** pasar por `flatten_aiua_wui` ni `sanitize_bridge_message`. | CA-4: no-regresión de esos literales. CA-5: envelopes vía flatten. |

---

## 3. Causa raíz y anatomía del derrame en epidermis

### Cadena de propagación del error

```
┌─────────────────────────────────────────────────────────────┐
│ 1. Upstream Google Gemini AI Studio                         │
│    HTTP 503 UNAVAILABLE {"error": {"code": 503, ...}}       │
└──────────────────────────────┬──────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────┐
│ 2. Tool: gemini-http-infer (map_http_error_body)            │
│    Devuelve: "http-status-503: {\"error\":{...}}"  (188)    │
└──────────────────────────────┬──────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────┐
│ 3. Orquestador: aiua-stimulus-processing                    │
│    infer_gemini Err → OrchestratorEnvelope.failure          │
│    stdout: {"success": false, "error": "http-status-..."}   │
└──────────────────────────────┬──────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────┐
│ 4. Epidermis: kalma2-bridge                                 │
│    handle_aiua_interact -> flatten_aiua_wui                 │
│    -> sanitize_bridge_message(raw)                          │
│                                                             │
│    FALLO: raw mide 188 chars < límite de 240 chars.        │
│    Solo normaliza '\n'/'\r' -> ' '.                         │
│    No clasifica 503 -> entrega JSON crudo en `message`.     │
└──────────────────────────────┬──────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────┐
│ 5. Frontend WUI: interfaces/kalma2/app.js                   │
│    Pinta body.message en #output y recorta 120 en #status   │
└─────────────────────────────────────────────────────────────┘
```

El JSON compacto de Google en el incidente medía **171 caracteres**. Prefijo `http-status-503: ` = **17**. Total **188**. `sanitize_bridge_message` solo trunca >240 y sustituye saltos de línea; el payload completo pasó al HTTP 500 del puente.

---

## 4. Solución técnica y mensaje canónico

### 4.1 Comportamiento en `kalma2-bridge`

En `SddIA/interfaces/kalma2-bridge/src/main.rs`, clasificar **dentro** de `sanitize_bridge_message` (único cuello epidermal; `flatten_aiua_wui` y el colapso protésico ya la llaman):

1. **Heurística (A-PREFIX-503-SUFFICIENT / A-NO-BARE-503-MATCH):**
   - **Primaria (suficiente):** el mensaje contiene `http-status-503`.
   - **Secundaria:** ausente el prefijo, y contiene `"code":503` o `"code": 503`, **y** un marcador (`UNAVAILABLE`, `high demand`, `temporarily unavailable`, `service unavailable`) — case-insensitive en los marcadores de texto.
   - **Exclusiones:** `503` suelto; `http-status-404` / `429`; `gemini-model-unavailable`; literales `timeout motor` y `subproceso falló`.
2. **Mensaje canónico (castellano, constante única compartida con tests):**
   ```text
   Tormentosa no disponible temporalmente por alta demanda del proveedor. Inténtalo más tarde.
   ```
   - **Longitud:** 91 caracteres (≤120).
   - **Caracteres JSON:** 0 llaves (`{`, `}`).
3. **Preservación de otros errores:**
   - Colapso protésico (`orquestador nativo no encontrado`, spawn fail): fractura Kintsugi intacta; mensaje sanitizado **sin** remapear a 503.
   - `gemini-model-unavailable` / 404 de modelo: intacto.
   - `timeout motor` / `subproceso falló`: JSON cableado en el handler; **no** atraviesan el clasificador.
   - Test existente `flatten_aiua_wui_rejects_business_failure` (`"gemini 503"`): debe seguir conteniendo `gemini 503`.

### 4.2 Frontend (`interfaces/kalma2/app.js`)

**Intacto (cero líneas modificadas).**
Tras el canónico, `#output`:

`[error] Tormentosa no disponible temporalmente por alta demanda del proveedor. Inténtalo más tarde.`

`#status` (`failed · ` + msg, msg ≤120):

`failed · Tormentosa no disponible temporalmente por alta demanda del proveedor. Inténtalo más tarde.`

### 4.3 Herramienta (`gemini-http-infer`)

**Intacta.** Cero diffs en `SddIA/tools/gemini-http-infer/src/main.rs` ni `{name}.md`.

---

## 5. Deslinde y fronteras

```
                      Error upstream de Gemini (503)
                                    │
           ┌────────────────────────┴────────────────────────┐
           ▼                                                 ▼
    En este PBI (Epidermis)                         Fuera de este PBI
    ───────────────────────                         ─────────────────
    • kalma2-bridge:                                • Cero sleep / retry / polling (DA-5)
      - Clasificar 503 upstream                     • Cero failover multi-LLM (Kitchen)
      - Retornar mensaje canónico                   • Cero mutación en gemini-http-infer (DA-2)
      - HTTP 500 limpio en JSON                     • Cero mutación en interfaces/kalma2/app.js
      - Tests unitarios herméticos                  • Cero fractura Kintsugi por 503
```

1. **Deslinde con `PBI-FIX-KALMA2-BRIDGE-AIUA-ROUTE-STALE-ELF`:** aquél selló el 404 de ELF fósil. Aquí la ruta y el handler ya operan; se atiende la higiene epidermal del `message`.
2. **Deslinde con `PBI-FIX-FRACTURE-64f37c7f7b34`:** fractura `mayeuta-llm` en `POST /api/chat`. Ortogonal.
3. **Deslinde con `PBI-MULTI-LLM-ROUTER`:** cocina. Este PBI no añade enrutadores.

---

## 6. Criterios de aceptación (Gates)

- [ ] **KALMA-503-CA1 — Sanitización del payload en el puente:** envelope de orquestador con `error` `http-status-503: {"error":{"code":503,…,"status":"UNAVAILABLE"}}` → JSON del puente: `message` = canónico, sin blob Google ni `{`.
- [ ] **KALMA-503-CA2 — Mensaje canónico:** exactamente `Tormentosa no disponible temporalmente por alta demanda del proveedor. Inténtalo más tarde.` (91 chars, ≤120).
- [ ] **KALMA-503-CA3 — Veto DA-5:** no se introduce `sleep`, retry, backoff ni polling de reintento en `kalma2-bridge` ni en la cadena de esta petición. Respuesta inmediata y terminal. Iteración `for`/`loop` preexistente no es veto.
- [ ] **KALMA-503-CA4 — No-regresión y no-enmascaramiento:**
  - `POST /api/chat` y 404 `ruta desconocida` intactos.
  - `gemini-model-unavailable`, `timeout motor`, `subproceso falló`, colapso protésico y el error genérico `"gemini 503"` **no** se sustituyen por el canónico 503.
- [ ] **KALMA-503-CA5 — Tests herméticos** en el binario `kalma2-bridge`:
  1. Envelope 503 del incidente → canónico.
  2. Envelope `http-status-503` **sin** tokens UNAVAILABLE/high demand → canónico (prefijo suficiente).
  3. Envelopes no-503 (404 modelo, `"gemini 503"`, timeout no aplica a flatten) → sin remapear.
  4. Cero red (`A-HERMETIC-TESTS-NO-LIVE`).
- [ ] **KALMA-503-CA6 — Ciclo documental:** un único PR con código `kalma2-bridge`, tests, `validacion.md` y PBI en `docs/todos/done/` (`status: cerrado`). Cero diffs en `interfaces/kalma2/app.js` ni `SddIA/tools/gemini-http-infer`.
- [ ] **KALMA-503-CA-CI — Checks GitHub Actions del PR verdes con `run_id` antes de `global: APTO` definitivo y `accept-pr`.**

---

## 7. Plan de verificación empírica

### Paso 1: Pruebas unitarias de Rust (binario `kalma2-bridge`)

```bash
cargo test --manifest-path SddIA/interfaces/kalma2-bridge/Cargo.toml --bin kalma2-bridge
```

*Esperado:* suite verde, incluidos los tests nuevos de 503 y el existente `flatten_aiua_wui_rejects_business_failure`.

### Paso 2: Verificación estática anti-retry (DA-5)

Inspeccionar **solo el hunk añadido** (no el archivo entero) en busca de `sleep`, `retry`, `backoff`, `poll`:

```bash
git diff SddIA/interfaces/kalma2-bridge/src/main.rs
```

*Esperado:* clasificación síncrona. Cero primitivas de reintento nuevas. No usar `grep loop` como gate.

### Paso 3: No-mutación externa

```bash
git diff -- interfaces/kalma2/app.js SddIA/tools/gemini-http-infer
```

*Esperado:* vacío.

### Paso 4: Contrato flatten (fixture del incidente)

```rust
let envelope_503 = serde_json::json!({
    "success": false,
    "error": "http-status-503: {\"error\":{\"code\":503,\"message\":\"This model is currently experiencing high demand. Spikes in demand are usually temporary. Please try again later.\",\"status\":\"UNAVAILABLE\"}}"
});
let err = flatten_aiua_wui(&envelope_503, 50).unwrap_err();
assert_eq!(err, "Tormentosa no disponible temporalmente por alta demanda del proveedor. Inténtalo más tarde.");
assert!(!err.contains('{'));
assert_eq!(err.chars().count(), 91);
```

---

## 8. Lecciones aprendidas y ciclo de vida

1. **Epidermis vs Genoma:** `gemini-http-infer` reporta telemetría técnica. `kalma2-bridge` traduce contingencia de proveedor a mensaje biológico.
2. **Veto DA-5:** 503 upstream no autoriza retener el socket con `sleep`/retry. Respuesta inmediata; el Vértice decide el siguiente latido.
3. **Higiene de cifras:** longitudes de payload se cuentan sobre la cadena compacta real, no a ojo.
4. **Higiene de cierres:** PBI en `pending/` mientras `abierto`; a `done/` con `status: cerrado` solo en el PR de implementación. `global: APTO` exige `run_id` CI verde (CA-CI).
