---
feature_name: poc-interface-comunicacion
created: "2026-06-18"
process: feature
branch_name: feat/poc-interface-comunicacion
items_applied:
  - T1-bridge
  - T2-index-html
  - T3-app-js
  - T4-style-css
  - T6-smoke-fixture
---

# Ejecución — PoC Interface Comunicación (Kalma2)

## Ola W1 aplicada

| ID | Artefacto | Operación |
|----|-----------|-----------|
| T1 | `.SddIA/client/sddia-client-bridge.py` | creado |
| T2 | `interfaces/kalma2/index.html` | creado |
| T3 | `interfaces/kalma2/app.js` | creado |
| T4 | `interfaces/kalma2/style.css` | creado |
| T6 | `docs/features/poc-interface-comunicacion/_smoke-kalma2-interact.json` | creado |

## Comandos verificados

```bash
# Arrancar puente (localhost:8765)
python3 .SddIA/client/sddia-client-bridge.py

# Smoke API (otra terminal)
curl -s -X POST http://127.0.0.1:8765/api/interact \
  -H 'Content-Type: application/json' \
  -d @docs/features/poc-interface-comunicacion/_smoke-kalma2-interact.json

# UI en navegador
# http://127.0.0.1:8765/
```

## Resultados smoke (2026-06-18)

| AC | Evidencia |
|----|-----------|
| AC2 | `POST /api/interact` → `{"success": true, "response": "[eco PoC] ping de prueba kalma2", ...}` |
| AC3 | Log `[CONFIG] Jerarquía detectada` — bóvedas cargadas al arranque |
| AC5 | Eco visible en respuesta JSON; hilo servidor estable tras petición |
| AC1 | `GET /` → HTTP 200 (index.html servido) |

## Operador

1. Levantar puente: `python3 .SddIA/client/sddia-client-bridge.py`
2. Abrir `http://127.0.0.1:8765/` en navegador.
3. Escribir prompt → **Forjar** → ver respuesta eco en panel inferior.
4. Puerto alternativo: `SDDIA_CLIENT_PORT=9000` en bóveda o entorno.

## Deuda explícita (W3)

- `invoke_engine()` aún es **stub eco**; motor real `kalma2-interact` requiere forja genoma vía `entity-manager`.
- Puente en instancia (`.SddIA/client/`); promoción a genoma pendiente de estabilización.
- Sin Cerbero/RBAC — solo bind `127.0.0.1`.
