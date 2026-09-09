---
feature_name: kalma2-wui-aiua-503-sanitize
created: "2026-09-09"
process: bug-fix
version_implementation: "1.0.0"
items:
  - classify-sanitize-503
  - hermetic-tests
  - intact-appjs-gemini
---

# Implementación — sanitización 503 Gemini en epidermis

## Cambios

| Archivo | Cambio |
|---------|--------|
| `SddIA/interfaces/kalma2-bridge/src/main.rs` | Constante `AIUA_PROVIDER_UNAVAILABLE_MSG`; `is_gemini_upstream_unavailable`; clasificación al inicio de `sanitize_bridge_message`; 5 tests nuevos + refuerzo del test `"gemini 503"` |
| `interfaces/kalma2/app.js` | **Intacto** |
| `SddIA/tools/gemini-http-infer/` | **Intacto** |

## Detalle

1. **Señal primaria:** substring `http-status-503` → canónico (91 chars).
2. **Señal secundaria:** `"code":503` / `"code": 503` + marcador de indisponibilidad, si falta el prefijo.
3. **Exclusión:** `503` suelto (test `flatten_aiua_wui_rejects_business_failure`).
4. **HTTP:** sigue 500 + `{success:false, message}`. Cero fractura. Cero sleep/retry.

## Tests

`cargo test --manifest-path SddIA/interfaces/kalma2-bridge/Cargo.toml --bin kalma2-bridge` → 41 passed.
