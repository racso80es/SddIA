---
feature_name: aiua-agy-empty-response-text
created: "2026-09-10"
process: bug-fix
version_implementation: "1.0.0"
items:
  - extract-infer-text
  - hermetic-test
  - intact-skill-appjs
---

# Implementación — extract_infer_text

## Cambios

| Archivo | Cambio |
|---------|--------|
| `SddIA/engine/execute-process/src/engine/handlers/aiua_stimulus.rs` | `nonempty_infer_str` + `extract_infer_text`; `run` aborta con `agy respuesta vacía` si no hay cuerpo; test hermético |
| `SddIA/skills/antigravity-cli-executor/` | **Intacto** |
| `interfaces/kalma2/app.js` | **Intacto** |
| `SddIA/process/aiua-stimulus-processing.md` | **Intacto** |

## Detalle

1. No mutar `map_agy_result` (genoma skill). El handler lee `raw_response` que la skill ya copia.
2. Persist sigue exigiendo texto; el vacío se clasifica en combustión, no en memoria.
3. Cero retry/sleep.
