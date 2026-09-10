---
document_id: PBI-FIX-AIUA-AGY-EMPTY-RESPONSE-TEXT
uuid: "9d4898e6-d960-4dff-95b0-ce7d807c00c1"
title: "[FIX] Aiúa — combustión agy SUCCESS sin texto persistible"
format: markdown
version: "1.0.0"
created: "2026-09-10"
updated: "2026-09-10"
status: "cerrado"
closed: "2026-09-10"
fix_ref: docs/fixes/aiua-agy-empty-response-text
refinement_status: refinado
refined: true
priority: alta
type: fix
process: bug-fix
dispatch: false
suggested_branch: fix/aiua-agy-empty-response-text
persist_ref: docs/fixes/aiua-agy-empty-response-text
persist_ref_suggested: docs/fixes/aiua-agy-empty-response-text
spawned_by: PBI-NUCLEO-AIUA-ANTIGRAVITY-CLI-VECTOR
incident_ref: "Kalma2 WUI #aiua-pulse 2026-09-10 — [error] response_text obligatorio post-login agy"
architectural_constraints:
  - A-NO-SKILL-MUTATION-DA2
  - A-INTACT-FRONTEND-APPJS
  - A-NO-MULTI-LLM-FAILOVER-SCOPE
  - A-NO-RETRY-BACKOFF-DA5
  - A-HERMETIC-TESTS-NO-LIVE
related:
  - docs/todos/done/[NÚCLEO] Aiúa — combustión Tormentosa vía antigravity-cli.md
  - SddIA/engine/execute-process/src/engine/handlers/aiua_stimulus.rs
  - SddIA/actions/persist-thought-record.md
  - SddIA/skills/antigravity-cli-executor/src/main.rs
  - SddIA/interfaces/kalma2-bridge/src/main.rs
---

# [FIX] Aiúa — combustión agy SUCCESS sin texto persistible

## Síntoma

Tras login `agy` en host, `#aiua-pulse` → `[error] response_text obligatorio`. No es auth, DNS ni `http-post-failed`.

## Causa

`aiua-stimulus-processing::run` extraía solo `result.text` como string. `persist_thought_record` usa `str_opt` (trim; vacío = ausente) → `"response_text obligatorio"`.

`map_agy_result` (skill, **fuera de este fix**) hace `response.as_str()`. Si `agy` SUCCESS trae `response` objeto/array o `text` en blanco, el handler persiste `""` y tumba el latido **después** de inferir. El WUI no ve el cuerpo.

## Corrección

Solo handler nativo `aiua_stimulus.rs` (no genoma, no `app.js`, no skill):

1. `extract_infer_text`: `text` ≻ `response` ≻ `raw_response.{response,text,result}` (string, objeto `text|content|response`, array).
2. Vacío real → error de combustión `"agy respuesta vacía"` (no el de persistencia).
3. Test hermético `extract_infer_text_prefers_text_then_raw_response_object`.

## Fuera

Mutar `antigravity-cli-executor` (DA-2). `app.js`. Failover kitchen. Retry/backoff (DA-5). Live `agy` como gate CI.

## CA

| ID | Criterio |
|----|----------|
| AIUA-TXT-CA1 | `result.text` string no vacío se conserva |
| AIUA-TXT-CA2 | `text` vacío + `raw_response.response.text` objeto → extrae el cuerpo |
| AIUA-TXT-CA3 | Vacío real → `"agy respuesta vacía"`, no `"response_text obligatorio"` en `run` |
| AIUA-TXT-CA4 | Cero diffs skill/process/app.js |
| AIUA-TXT-CA5 | `cargo test -p execute-process --lib -- aiua_stimulus` verde |
| AIUA-TXT-CA6 | Un PR: handler + tests + persist_ref + PBI `done/` |
| AIUA-TXT-CA-CI | Checks GitHub verdes con `run_id` antes de `global: APTO` y `accept-pr` |
