---
feature_name: kalma2-wui-agy-network-sanitize
created: "2026-09-10"
process: bug-fix
version_implementation: "1.0.0"
items:
  - classify-agy-network
  - hermetic-tests
  - intact-skill-appjs
---

# Implementación — sanitización red agy

## Cambios

| Archivo | Cambio |
|---------|--------|
| `SddIA/interfaces/kalma2-bridge/src/main.rs` | `AIUA_AGY_NETWORK_MSG`; `is_agy_network_issue`; clasificación en `sanitize_bridge_message`; test `sanitize_maps_agy_network_issue` |
| `interfaces/kalma2/app.js` | **Intacto** |
| `SddIA/skills/antigravity-cli-executor/` | **Intacto** |

## Detalle

1. Señal: `network issue connecting` ∪ `dial tcp` (minúsculas).
2. Canónico 82 chars. No reutiliza el 503.
3. Cero sleep/retry.
