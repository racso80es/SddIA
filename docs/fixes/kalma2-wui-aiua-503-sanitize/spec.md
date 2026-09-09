---
feature_name: kalma2-wui-aiua-503-sanitize
created: "2026-09-09"
process: bug-fix
base: main
scope: kalma2-bridge-epidermal-503-sanitize
version_spec: "1.0.0"
branch_name: fix/kalma2-wui-aiua-503-sanitize
persist_ref: docs/fixes/kalma2-wui-aiua-503-sanitize
pbi_ref: docs/todos/pending/[OPERATIVO] Kalma2 WUI — 503 Gemini sanitizado en epidermis (sin retry).md
document_id: PBI-OPERATIVO-KALMA2-AIUA-503-SANITIZE
execution_id: "8abff97c-9ce3-4d83-bdd8-4b08ac0d75d0"
incident_ref: "Auditoría empírica WUI 2026-09-09 — 503 Gemini UNAVAILABLE crudo en WUI post-recycle"
---

# Especificación — sanitización 503 Gemini en epidermis `kalma2-bridge`

## Diagnóstico

| Campo | Valor |
|-------|-------|
| Síntoma | 2ª pulsación `#aiua-pulse`: `#output` con blob `http-status-503: {…UNAVAILABLE…}` |
| Puente HTTP | `POST /api/aiua/interact` → **500** (no 404). El 503 es texto en `message`, no status reenviado |
| PID / ELF | `1244544` / release mtime 15:17:48 CEST (post-recycle; ruta viva) |
| Cadena | Gemini 503 → `map_http_error_body` → `emit_v2` `error`/`feedback` → `infer_gemini` → `OrchestratorEnvelope.failure` → `flatten_aiua_wui` → `sanitize_bridge_message` |
| Defecto | `sanitize_bridge_message` solo `\n`/`\r` → espacio y `take(240)`. Cadena incidente **188** chars (< 240) |
| Cifras (v1.2.0) | JSON Google compacto **171**; prefijo `http-status-503: ` **17**; canónico **91** |

**No** es 404 de ELF fósil. **No** es fractura Kintsugi (`L-FRACTURE`: Gemini/`success:false` → 5xx sanitizado, sin evento). **No** autoriza retry (DA-5; `aiua-audit-findings/clarify.md`).

## Corrección

### H1 — Clasificador en `sanitize_bridge_message`

Único cuello epidermal (`flatten_aiua_wui` y colapso protésico ya la llaman).

| Señal | Regla |
|-------|--------|
| Primaria | substring `http-status-503` → canónico (suficiente) |
| Secundaria | sin prefijo: `"code":503` o `"code": 503` **y** marcador (`unavailable`, `high demand`, `temporarily unavailable`, `service unavailable`) |
| Exclusión | `503` suelto; `http-status-404`/`429`; `gemini-model-unavailable`; literales `timeout motor` / `subproceso falló` |

Constante única:

```text
Tormentosa no disponible temporalmente por alta demanda del proveedor. Inténtalo más tarde.
```

HTTP del puente permanece **500** + `{success:false, message}`. Cero `System_Fracture_Detected`.

### H2 — Tests herméticos

Binario `kalma2-bridge`. Fixture del incidente; prefijo sin tokens UNAVAILABLE; no-regresión `"gemini 503"` (`flatten_aiua_wui_rejects_business_failure`); `gemini-model-unavailable`. Cero red.

### H3 — Intactos

`interfaces/kalma2/app.js`, `SddIA/tools/gemini-http-infer` (DA-2). Cero `sleep`/retry/backoff/polling.

### H4 — Cierre documental

PBI → `docs/todos/done/`. `validacion.md` con `pbi_archived: true`. CA-CI con `run_id` verde antes de `global: APTO` y `accept-pr`.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| KALMA-503-CA1 | Envelope 503 Google → `message` canónico, sin `{` ni blob |
| KALMA-503-CA2 | Texto exacto, 91 chars, ≤120 |
| KALMA-503-CA3 | Cero sleep/retry/backoff/polling nuevos. `for`/`loop` preexistente no es veto |
| KALMA-503-CA4 | No-regresión `/api/chat` y 404 `ruta desconocida`; no enmascarar `gemini-model-unavailable`, timeout/join, colapso, `"gemini 503"` |
| KALMA-503-CA5 | Tests herméticos en `--bin kalma2-bridge` |
| KALMA-503-CA6 | Un PR: código + tests + PBI `done/` + `validacion.md`. Cero diffs app.js / gemini-http-infer |
| KALMA-503-CA-CI | Checks GitHub verdes con `run_id` antes de APTO definitivo y `accept-pr` |

## Alcance prohibido

| Prohibido | Motivo |
|-----------|--------|
| Mutar `gemini-http-infer` | DA-2; telemetría técnica debe permanecer cruda |
| Mutar `interfaces/kalma2/app.js` | Ya consume `body.message` |
| Retry / backoff / sleep | DA-5 |
| Failover multi-LLM | `PBI-MULTI-LLM-ROUTER` kitchen |
| Fractura por 503 | `L-FRACTURE` |
| Casar `503` suelto | Test `flatten_aiua_wui_rejects_business_failure` |
| Reenviar HTTP 503 del proveedor | Contrato actual del puente: fallos de flatten → 500 |
