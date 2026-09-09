---
document_id: PBI-NUCLEO-AIUA-ANTIGRAVITY-CLI-VECTOR
uuid: "d15e82ed-ccad-4e5a-80ea-c6e808afa086"
title: "[NÚCLEO] Aiúa — combustión Tormentosa vía antigravity-cli"
format: markdown
version: "1.2.0"
created: "2026-09-09"
updated: "2026-09-09"
status: "cerrado"
refinement_status: refinado
priority: alta
type: nucleo
process: feature
dispatch: false
suggested_branch: feat/aiua-antigravity-cli-vector
persist_ref: docs/features/aiua-antigravity-cli-vector
persist_ref_suggested: docs/features/aiua-antigravity-cli-vector
spawned_by: PBI-NUCLEO-PUENTE-PERCEPTIVO-KALMA2
lauds: H-AIUA-VECTOR
depends_on: []
blocks_on: []
related:
  - docs/todos/kitchen/PBI-MULTI-LLM-ROUTER.md
  - docs/todos/done/PBI_Arranque_Aiua.md
  - docs/todos/done/[NÚCLEO] Puente Perceptivo: Interacción Biológica con Tormentosa desde Kalma2 WUI.md
  - docs/todos/done/[KAIZEN] Aiúa — hallazgos auditoría live y thinking HIGH.md
  - docs/todos/done/[OPERATIVO] Forja de cápsulas nativas para integración dual con Google Antigravity (HTTP y CLI).md
  - docs/todos/done/[OPERATIVO] Kalma2 WUI — 503 Gemini sanitizado en epidermis (sin retry).md
  - docs/todos/pending/[NÚCLEO] Anatomía Motora de Aiúa — Inyección de Capacidades (Function Calling) y Orquestación EDA.md
  - docs/features/nucleo-aiua-tormentosa-motor/auditoria.md
  - SddIA/process/aiua-stimulus-processing.md
  - SddIA/process/kalma2-interact.md
  - SddIA/engine/execute-process/src/engine/handlers/aiua_stimulus.rs
  - SddIA/engine/execute-process/src/engine/capsules.rs
  - SddIA/interfaces/kalma2-bridge/src/main.rs
  - interfaces/kalma2/app.js
  - SddIA/skills/antigravity-cli-executor.md
  - SddIA/skills/antigravity-cli-executor/src/main.rs
  - SddIA/tools/gemini-http-infer.md
  - SddIA/conscience/aiua_core.md
refinement_notes: >-
  v1.2.0 Filtro A (2026-09-09). Contrastado contra genoma y crates. Correcciones:
  (1) ruta handler §4.2 era SddIA/engine/handlers/… (no existe); SSOT =
  execute-process/src/engine/handlers/aiua_stimulus.rs. (2) argv vigente de la
  skill NO emite --print (tests lo prohíben); genoma skill.md desfasado — este
  PBI no lo muta. (3) SDDIA_AGY_TIMEOUT_SECS no existe en la skill; timeout =
  DEFAULT 300s o parameters.print_timeout; bridge default 120s = hazard real.
  Var nueva de instancia; handler inyecta print_timeout; no mutar skill. (4)
  result.usage es passthrough de agy, no contrato de cuatro claves; lab-mock
  usage {}. (5) effort siempre inyectado en Aiúa (no solo si hay model);
  SDDIA_GEMINI_THINKING_LEVEL es fallback case-insensitive, no alias canónico.
  (6) skill no emite result.model. (7) EM process: markdown_body_replacements
  corta antes de process_phases → dos updates. (8) CA-CI obligatorio (patrón
  v1.2.1) antes de global APTO y accept-pr.
---

# [NÚCLEO] Aiúa — combustión Tormentosa vía antigravity-cli

## 0. Laudo

Cierra **H-AIUA-VECTOR** (`PBI-MULTI-LLM-ROUTER` kitchen §4). La combustión del latido ontológico de Tormentosa transmuta a `skill:antigravity-cli-executor` (`agy` headless: `--output-format json` + `-p`; **sin** `--print` en argv vigente), abandonando `tool:gemini-http-infer` **en este proceso**.

El modelo en la instancia sigue siendo **Gemini 3.8 Flash** (`gemini-3.8-flash` inyectado por bóveda, cero hardcode). `agy` exige `--effort` para ese slug (`requires --effort (available: low, medium, high)`). Mandato biológico: **Gemini 3.8 Flash High** (`--effort high`). Identidad ontológica = `aiua_core.md`, no la marca del actuador.

Se reutiliza la skill vigente `antigravity-cli-executor` (uuid `d8b07e6f-1cc0-4b6f-a789-02ade10471f5`). **No** se forja cápsula nueva. **No** se muta el crate de la skill salvo defecto de I/O bloqueante descubierto en forja.

## 1. Superficie real (Kalma2 ≠ combustión)

Kalma2 es la epidermis sensorial. El canal de Tormentosa ya está desacoplado:

```text
WUI #aiua-pulse
  → POST /api/aiua/interact          kalma2-bridge
  → execute-process                  --process aiua-stimulus-processing
  → Combustion-Inferencia            HOY:  tool:gemini-http-infer
                                     LAUDO: skill:antigravity-cli-executor
```

* `kalma2-interact` (`POST /api/chat`) atiende exclusivamente a Mayeuta (`mayeuta-llm` / `llm:interact`). **Queda fuera.** El puente no infiere: spawnea el proceso. El test `process_genome_has_no_kalma2_ui_coupling` permanece inalterado.
* `interfaces/kalma2/app.js` permanece intacto: contrato de salida del latido (`response`, `thought_id`, `telemetry.model`, `telemetry.duration_ms`, `telemetry.tokens?` objeto) no muta de forma. Las claves internas de `tokens` sí pueden pasar de `usageMetadata` Gemini REST a `usage` de `agy` (passthrough).
* `kalma2-bridge` se muta únicamente en epidermis de error (`sanitize_bridge_message`) y alineación de timeout. Analogía: sanitización 503 HTTP ya entregada (PR #282).

## 2. Filtro A — trampas y hechos contrastados

| Tentación / Suposición | Veredicto | Realidad SSOT contrastada |
|------------------------|-----------|---------------------------|
| Cambiar el Chat Kalma2 / `mayeuta-llm` | *Conflación de canales* | Chat = Mayeuta. Tormentosa = `aiua-stimulus-processing`. Tuberías independientes. |
| Cablear `agy` en `kalma2-bridge` | *Órgano inventado* | El puente despacha el proceso. Ceguera espacial. |
| Retirar o mutar `gemini-http-infer` | *Alcance excesivo* | Argos `notify-humanized-pr-merged` sigue en HTTP. Tool intacta. |
| Rebind `llm:interact` → antigravity | *Violación DI* | Binding canónico = `skill:mayeuta-llm`. Estas cápsulas **no** declaran `provides llm:interact`. |
| Hardcodear `gemini-3.8-flash` en genoma o Rust | *Putrefacción de catálogo* | L-MODEL / `--model` se resuelve desde input `model` ≻ `SDDIA_GEMINI_MODEL`. Starter-kit comentado y vacío. |
| Tratar `--effort` como opcional | *Inexactitud crítica live* | `agy --model gemini-3.8-flash` sin `--effort` → exit 1. Handler **siempre** inyecta `parameters.effort` (`high` por defecto, normalizado `low\|medium\|high`). |
| Autenticar `agy` con `GEMINI_API_KEY` | *Falso* | Sesión cacheada en el host. Fallo: `authentication required` / `not logged into antigravity`. |
| Este PBI = router multi-LLM | *Kitchen ajeno* | `PBI-MULTI-LLM-ROUTER` permanece incubado. Aquí: un vector, un consumidor (Aiúa). |
| Extraer error solo de `message` de la skill | *Diagnóstico ciego* | `message` = `"agy-failed"`. Causa en `"error"` y `"feedback"` (mismo string). Exit≠0 sin map: `"agy exit=N; stderr=…"`. |
| Telemetría desde `usageMetadata` | *Mapeo fantasma* | REST Gemini. Skill entrega `result.usage` (passthrough del JSON `agy`). Lab-mock: `usage: {}`. |
| Inventar contrato `input_tokens`/`output_tokens`/`thinking_tokens`/`total_tokens` | *Schema fantasma* | Fixture de la skill usa `input_tokens`. Claves live de `agy` **no** están contratadas. Propagar el objeto `usage` íntegro si es object y no vacío. |
| `SDDIA_AGY_TIMEOUT_SECS` ya leída por la skill | *Var inventada* | Skill: default 300s **o** `parameters.print_timeout` (`"30s"`). Bridge: default 120s (`SDDIA_CLIENT_TIMEOUT_SECONDS`). Hazard: puente aborta a 120s mientras `agy` sigue hasta 300s. |
| `--print` en argv de `agy` | *Fósil del genoma skill.md* | Crate vigente: `--output-format json` + `-p`. Test `argv_default_uses_sandbox_not_skip` aserta ausencia de `--print`. No mutar skill para «restaurar» el fósil. |
| Retry ante saturación o timeout del CLI | *Violación DA-5* | Un solo intento. Falla limpia. Cero `sleep`/bucles en motor. |
| Mutar genoma process a mano | *Violación DA-2* | Solo `entity-manager`. Dos updates: phases+inputs, luego `markdown_body_replacements` (el segundo corta antes de phases). |
| `result.model` eco de la skill | *Campo ausente* | Skill no emite `model`. `telemetry.model` = modelo resuelto en handler (input ≻ env) o vacío. |

## 3. Estado as-is (2026-09-09)

| Órgano | Estado físico y contractual |
|--------|------------------------------|
| Proceso `aiua-stimulus-processing` v1.0.0 uuid `6c595785-e386-402f-b570-0b2aa6343051` | Fase `Combustion-Inferencia` → `tool:gemini-http-infer`. Input `model` opcional ≻ `SDDIA_GEMINI_MODEL`. Cuerpo: «única inferencia Gemini». Cero Kalma2. |
| Handler `SddIA/engine/execute-process/src/engine/handlers/aiua_stimulus.rs` | `infer_gemini` → `invoke_tool_capsule_json(..., "gemini-http-infer")` con `request.prompt` + `request.model` (no `parameters`). Telemetría `raw_response/usageMetadata`. Reporte `"handler": "gemini-http-infer"`. Lab-mock copia binario HTTP y espera prefijo `lab-mock:`. Limpia `SDDIA_LAB_MOCK_GEMINI_URL`. |
| Skill `antigravity-cli-executor` v1.0.0 uuid `d8b07e6f-1cc0-4b6f-a789-02ade10471f5` | Lee `request.prompt`, `request.parameters.{model,effort,print_timeout,add_dir,skip_permissions}`. Lab-mock: `lab-mock-agy:{prompt[0..80]}`, `usage: {}`. Éxito: `result.text`, `result.usage`, `result.durationMs`, `result.raw_response`. Default `--sandbox`. Timeout default 300s. Env reales: `SDDIA_AGY_PATH`, `SDDIA_AGY_ALLOW_SKIP_PERMISSIONS`. Auth map: `authentication required` \| `not logged into antigravity` → `"agy authentication required"`. Timeout hijo → `"agy-timeout"`. |
| Instancia | `SDDIA_GEMINI_MODEL=gemini-3.8-flash` en `.dev/.env`. `agy` en PATH del host. HTTP AI Studio 503 recurrente. |
| WUI / puente | `#aiua-pulse` vía PR #276. `flatten_aiua_wui` aplana respuesta. Sanitización 503 HTTP (PR #282). Timeout puente: `max(SDDIA_CLIENT_TIMEOUT_SECONDS\|\|120, SDDIA_GEMINI_HTTP_TIMEOUT_SECS)`. Errores CLI no tipificados. |

## 4. Alcance de la Forja (ciclo `feature`)

### 4.1 Genoma del Proceso (DA-2)

`./sddia-run.sh --process entity-manager` (`entity_class: process`, `lifecycle_operation: update`, `entity_name: aiua-stimulus-processing`). Prohibido Write/StrReplace sobre `SddIA/process/`.

**Dos invocaciones** (factory: `markdown_body_replacements` retorna antes de `process_phases`):

1. **Frontmatter:** `process_phases` (intent de `Combustion-Inferencia` + `delegates_to: skill:antigravity-cli-executor`) + `process_inputs` (añadir `effort`) + `process_version: 1.1.0`. UUID inmutable. `process_contract_version` vigente del artefacto (`1.4.0`); **no** reenviar el default EM `1.3.0`.
2. **Cuerpo:** `markdown_body_replacements` sustituyendo «única inferencia Gemini» → combustión Antigravity CLI.

* `delegates_to` fase `Combustion-Inferencia`: `skill:antigravity-cli-executor`. Cero `tool:gemini-http-infer` en ese genoma.
* `intent`: `Única llamada al LLM vía agy; Peaje Termodinámico del CLI.`
* `inputs`: conservar `model: string opcional; else SDDIA_GEMINI_MODEL`. Añadir `effort: string opcional; default=high`.

### 4.2 Handler nativo (`SddIA/engine/execute-process/src/engine/handlers/aiua_stimulus.rs`)

Sustituir `infer_gemini` por `infer_antigravity_cli`. Payload de skill (**no** el shape HTTP):

```rust
invoke_capsule_json(repo, "antigravity-cli-executor", &json!({
  "request": {
    "prompt": assembled_prompt,
    "parameters": {
      "model": model,   // omitir si vacío
      "effort": effort, // siempre; "high" por defecto
      "print_timeout": print_timeout // "{N}s" si se resolvió N
    }
  }
}), false)
```

1. **Resolución:**
   * `model`: `inputs.model` ≻ `env(SDDIA_GEMINI_MODEL)`. Si no vacío → `parameters.model`.
   * `effort`: `inputs.effort` ≻ `env(SDDIA_AGY_EFFORT)` ≻ `env(SDDIA_GEMINI_THINKING_LEVEL)` ≻ `"high"`. Normalizar a `low\|medium\|high` (case-insensitive; `HIGH`→`high`). Valor fuera de whitelist → error limpio, sin spawn. **Siempre** inyectar `parameters.effort`.
   * `print_timeout`: `env(SDDIA_AGY_TIMEOUT_SECS)` ≻ `env(SDDIA_CLIENT_TIMEOUT_SECONDS)` ≻ `"300s"` (default skill). Formato `{N}s`. Inyectar en `parameters.print_timeout` para alinear techo skill↔puente **sin** mutar la skill.
2. **Invocación:** `invoke_capsule_json` (label `skill`). Conservar `invoke_tool_capsule_json` para `thought-graph-access`. Extraer `result.text`.
3. **Telemetría:**
   * `telemetry.tokens` ← `result.usage` **si** es objeto no vacío. No `usageMetadata`. No exigir claves concretas.
   * `telemetry.model` ← modelo resuelto en handler (la skill no ecoa `model`).
   * `telemetry.duration_ms` ← `result.durationMs` o elapsed.
4. **Reporte:** fase `"Combustion-Inferencia"` → `"handler": "antigravity-cli-executor"`.
5. **Fallos:** `cap.exit_code != 0` o `success == false` → diagnóstico `cap.body["error"]` ≻ `cap.body["feedback"]` ≻ `"agy-failed"`. `Err(...)` limpio. Cero reintentos (DA-5).
6. **Lab-mock / tests:**
   * Copiar binario `antigravity-cli-executor` (`workspace_debug_bin`), no `gemini-http-infer`.
   * Aserto de respuesta: prefijo `lab-mock-agy:`.
   * Dejar de tocar `SDDIA_LAB_MOCK_GEMINI_URL` (irrelevante).
   * Tests unitarios de `resolve_effort` / timeout si se extraen helpers.

### 4.3 Bóveda de instancia

| Variable | ¿Existe hoy? | Rol en este vector |
|----------|--------------|-------------------|
| `SDDIA_GEMINI_MODEL` | Sí | Fallback de modelo. Se transmite a `parameters.model`. |
| `SDDIA_AGY_EFFORT` | **Nueva** | `high\|medium\|low`. Default Aiúa: `high`. |
| `SDDIA_GEMINI_THINKING_LEVEL` | Sí (HTTP) | Fallback de effort, normalizado a minúsculas. No es alias canónico. |
| `SDDIA_AGY_PATH` | Sí (skill) | Binario `agy`. Handler no la lee; la skill sí. |
| `SDDIA_AGY_TIMEOUT_SECS` | **Nueva** | Entero segundos. Handler → `print_timeout`. Bridge → `max(...)`. |
| `SDDIA_CLIENT_TIMEOUT_SECONDS` | Sí (bridge) | Fallback de timeout si `SDDIA_AGY_TIMEOUT_SECS` vacío. |
| `SDDIA_AGY_ALLOW_SKIP_PERMISSIONS` | Sí (skill) | **No** activar para el latido. Default `--sandbox`. |
| `GEMINI_API_KEY` | Sí | Ya no requerida para Tormentosa. Sigue para Argos HTTP. |

Starter-kit **comentado, sin slugs**: `SddIA/scripts/starter-kit/.dev/.env.example` y `SddIA/scripts/starter-kit/.SddIA/.dev/.env.example`. Documentar `SDDIA_AGY_EFFORT` y `SDDIA_AGY_TIMEOUT_SECS` vacías.

### 4.4 Epidermis Kalma2 (`SddIA/interfaces/kalma2-bridge/src/main.rs`)

1. **`sanitize_bridge_message`:**
   * Auth (`agy authentication required`, `not logged into antigravity`, `please sign in`, `authentication required`) →
     `Tormentosa no disponible: se requiere autenticación en el CLI de Antigravity (agy).`
   * Timeout (`agy-timeout`) →
     `Tormentosa no disponible: tiempo de espera agotado al consultar el CLI de Antigravity.`
   * No reutilizar `AIUA_PROVIDER_UNAVAILABLE_MSG` (503 HTTP). No enmascarar timeouts genéricos (`timeout motor`) ni `gemini-model-unavailable`.
   * Truncar resto técnico (ya: 240 chars, sin CR/LF). Tests herméticos nuevos.
2. **Timeout:** `resolve_client_timeout_secs` pasa a considerar tres techos: client, Gemini HTTP, `SDDIA_AGY_TIMEOUT_SECS`. `max` de los definidos (>0). Si agy unset, usar 300 (default skill) **solo en la ruta `/api/aiua/interact`** si se puede acotar; si la función es global al bridge, no subir el default 120 de `/api/chat` — preferir `max(client, gemini, agy)` y que el operador fije `SDDIA_AGY_TIMEOUT_SECS` o `SDDIA_CLIENT_TIMEOUT_SECONDS`. Alineación mínima: si `SDDIA_AGY_TIMEOUT_SECS` está set, el monitor no aborta por debajo de ese techo.
3. **Frontend:** `interfaces/kalma2/app.js` **cero** cambios.

## 5. Fuera de alcance

* Chat Mayeuta (`/api/chat`), `kalma2-interact.md`, rebind `llm:interact`.
* Crate `gemini-http-infer` y crate `antigravity-cli-executor` (salvo I/O bloqueante en forja).
* Restaurar `--print` en la skill ni «arreglar» el genoma skill.md desfasado.
* Router multi-LLM (`PBI-MULTI-LLM-ROUTER` kitchen).
* Anatomía motora / function calling (`PBI-NUCLEO-AIUA-ANATOMIA-MOTORA-EDA`).
* Red / keyring Google en CI (lab-mock exclusivo).
* Activar `SDDIA_AGY_ALLOW_SKIP_PERMISSIONS` para Aiúa.

## 6. Criterios de Aceptación (Protocolo de Acero)

- [ ] **AIUA-AGY-CA1 (Genoma):** `aiua-stimulus-processing.md` mutado vía `entity-manager`. `Combustion-Inferencia` delega en `skill:antigravity-cli-executor`. Cero `tool:gemini-http-infer` en ese genoma. UUID inmutable.
- [ ] **AIUA-AGY-CA2 (Despacho):** `aiua_stimulus.rs` invoca `invoke_capsule_json(..., "antigravity-cli-executor", ...)`. `parameters.effort` **siempre** (`high` por defecto). `parameters.model` si hay modelo resuelto. Shape `request.parameters`, no `request.model` HTTP.
- [ ] **AIUA-AGY-CA3 (Telemetría):** `telemetry.tokens` ← `result.usage` si objeto no vacío. Cero `usageMetadata`. Lab-mock: usage vacío → tokens omitidos (paridad actual).
- [ ] **AIUA-AGY-CA4 (Lab-mock):** `SDDIA_LAB_MOCK_OUTBOUND=1` → `success: true`, `thought_id` 64 hex, `response` prefijo `lab-mock-agy:`, `telemetry.duration_ms`. Test `lab_mock_empty_memories_yields_duration_and_thought_id` copia `antigravity-cli-executor`, no el binario HTTP.
- [ ] **AIUA-AGY-CA5 (Epidermis):** Auth CLI / `agy-timeout` → mensajes castellanos canónicos en el puente. Cero JSON crudo. Cero reintentos. Tests herméticos. 503 HTTP sigue mapeando a `AIUA_PROVIDER_UNAVAILABLE_MSG`.
- [ ] **AIUA-AGY-CA6 (Agnosticismo):** Cero literal `gemini-3.8-flash` en Rust de producción ni genoma.
- [ ] **AIUA-AGY-CA7 (Argos):** `gemini-http-infer` y `notify-humanized-pr-merged` intactos.
- [ ] **AIUA-AGY-CA8 (Ceguera Kalma2):** `process_genome_has_no_kalma2_ui_coupling` APTO. WUI sin marcas Google/CLI. `app.js` intacto.
- [ ] **AIUA-AGY-CA9 (Live instancia):** Con `agy` autenticado y `SDDIA_GEMINI_MODEL` de bóveda, latido CLI o `#aiua-pulse` responde en primera persona, con `thought_id` y telemetría, sin 503 AI Studio. **No es gate de CI.**
- [ ] **AIUA-AGY-CA10 (Cierre documental):** Ciclo `feature`, PBI en `docs/todos/done/` en la rama del PR, `validacion.md` con `pbi_archived: true`.
- [ ] **AIUA-AGY-CA-CI:** Checks GitHub Actions del PR verdes con `run_id` **antes** de `global: APTO` definitivo y `accept-pr`. Si rojo: un parche + un push (DA-6). Sin `run_id` → `PENDIENTE-CI`, no APTO.

## 7. Orden de Forja y Ejecución

1. **Apertura:** `./sddia-run.sh --process feature` (`persist_ref`: `docs/features/aiua-antigravity-cli-vector`). Prefijo Raw Kernel. Relé IDE. Commit planificación (`clarify`/`objectives`/`spec`/`plan`) **antes** de mutar genoma/handler.
2. **Genoma:** `entity-manager` update ×2 (phases+inputs, luego body).
3. **Handler:** `aiua_stimulus.rs` + tests lab-mock.
4. **Epidermis:** `sanitize_bridge_message` + timeout + tests puente.
5. **Starter-kit:** claves AGY comentadas.
6. **Verificación local:** `cargo test -p execute-process --lib -- aiua_stimulus` y `cargo test --bin kalma2-bridge`.
7. **Live instancia** (CA9): opcional en ciclo; no bloquea CI.
8. **Cierre:** `validacion.md` con CA-CI `PENDIENTE-CI` → DCC → PR → `run_id` verde → sellar APTO → `accept-pr`.

## 8. Relación con Deuda Técnica y PBIs Hermanos

| Documento | Relación y frontera |
|-----------|---------------------|
| Kitchen `PBI-MULTI-LLM-ROUTER` | Este PBI lauda **H-AIUA-VECTOR**. El router multi-proveedor no se promueve. |
| `PBI-NUCLEO-AIUA-ANATOMIA-MOTORA-EDA` | Pending. Asumirá combustión CLI o adaptará tool-calling a `agy`. No bloquea este hito. |
| Kaizen `PBI-AIUA-AUDIT-FINDINGS-20260908` | Done (PR #275). Prefacio 1ª persona + `aiua_core.md` ya as-is. |
| Operativo `PBI-OPERATIVO-KALMA2-AIUA-503-SANITIZE` | Done (PR #282). Epidermis HTTP 503; este PBI replica sobriedad para errores CLI. |
| Operativo conectores Antigravity | Done. Skill vigente; este PBI **consume**, no re-forja. Genoma skill.md con `--print` queda como deuda de catálogo **fuera**. |
