---
feature_name: aiua-agy-empty-response-text
created: "2026-09-10"
process: bug-fix
base: main
scope: aiua-stimulus-extract-infer-text
version_spec: "1.0.0"
branch_name: fix/aiua-agy-empty-response-text
persist_ref: docs/fixes/aiua-agy-empty-response-text
pbi_ref: docs/todos/done/[FIX] Aiúa — combustión agy SUCCESS sin texto persistible.md
document_id: PBI-FIX-AIUA-AGY-EMPTY-RESPONSE-TEXT
execution_id: "7310a29a-ad26-4022-81c5-b6bbc9900165"
incident_ref: "Kalma2 WUI #aiua-pulse 2026-09-10 — [error] response_text obligatorio post-login agy"
---

# Especificación — texto de combustión agy antes de persistir

## Diagnóstico

| Campo | Valor |
|-------|-------|
| Síntoma | `#aiua-pulse` → `[error] response_text obligatorio` |
| Cadena | `agy` SUCCESS → skill `result.text` (solo `response.as_str()`) → `run` lee `result.text` string → `persist_thought_record` / `str_opt` |
| Defecto | `str_opt` trata `""`/whitespace como ausente. Si `response` no es string, el handler persiste vacío y falla **después** de inferir |
| No es | auth `agy`, DNS IPv6, `http-post-failed`, 404 de ruta, Kintsugi |

## Corrección

### H1 — `extract_infer_text` en `aiua_stimulus.rs`

Precedencia: `text` ≻ `response` ≻ `raw_response.{response,text,result}`.

Cada nodo: string no vacío; objeto `text|content|response`; array concatenado con `\n`.

Vacío tras extraer → `Err("agy respuesta vacía")` en `run`, sin llamar a persist.

### H2 — Test hermético

`extract_infer_text_prefers_text_then_raw_response_object`: string gana; objeto anidado; whitespace+vacío → empty.

### H3 — Intactos

Skill `antigravity-cli-executor` (DA-2). Genoma process. `app.js`. Bridge. Kitchen router.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| AIUA-TXT-CA1 | `text` string no vacío se conserva |
| AIUA-TXT-CA2 | `text` vacío + `raw_response.response.text` → cuerpo |
| AIUA-TXT-CA3 | Vacío real → `"agy respuesta vacía"` en `run` |
| AIUA-TXT-CA4 | Cero diffs skill/process/`app.js` |
| AIUA-TXT-CA5 | `cargo test -p execute-process --lib -- aiua_stimulus` verde |
| AIUA-TXT-CA6 | Un PR: handler + tests + persist_ref + PBI `done/` |
| AIUA-TXT-CA-CI | Checks GitHub verdes con `run_id` antes de `global: APTO` y `accept-pr` |
