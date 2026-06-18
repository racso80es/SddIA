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
  - T7-process-kalma2-interact
  - T8-kalma2-interact-core
  - T9-bridge-motor-integration
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

## Ola W3 aplicada (2026-06-18)

| ID | Artefacto | Operación |
|----|-----------|-----------|
| T7 | `SddIA/process/kalma2-interact.md` | forjado vía `entity-manager` |
| T8 | `SddIA/scripts/qa/kalma2_interact_core.py` | creado |
| T9 | `.SddIA/client/sddia-client-bridge.py` | `invoke_engine` → `execute-process kalma2-interact` |

## Comandos verificados

```bash
python3 -m unittest SddIA.scripts.qa.test_kalma2_interact -v

python3 .SddIA/client/sddia-client-bridge.py

curl -s -X POST http://127.0.0.1:8765/api/interact \
  -H 'Content-Type: application/json' \
  -d @docs/features/poc-interface-comunicacion/_smoke-kalma2-interact.json

# UI → http://127.0.0.1:8765/
```

## Resultados smoke (2026-06-18)

| AC | Evidencia |
|----|-----------|
| AC2 | `POST /api/interact` → `success: true` |
| AC3 | Bóvedas cargadas al arranque |
| AC5 | Respuesta Mayeuta lab (`Tormentosa/Aiúa`) en JSON |
| AC1 | `GET /` → HTTP 200 |
| O7 | 3 tests `test_kalma2_interact` OK |

## Operador

1. Levantar puente: `python3 .SddIA/client/sddia-client-bridge.py`
2. Abrir `http://127.0.0.1:8765/`
3. Prompt → **Forjar** → respuesta síntesis Mayeuta (≤2 líneas)
4. `SDDIA_CLIENT_PORT` / `SDDIA_CLIENT_TIMEOUT_SECONDS` opcionales en bóveda

## Prueba funcional navegador (2026-06-18)

Script: `_browser-func-test-kalma2.py` (Playwright headless).

| Paso | Resultado |
|------|-----------|
| Abrir `http://127.0.0.1:8765` | Título «Kalma2 — Cliente SddIA» |
| Prompt cierre (commit/PR/merge #94) | Enviado vía UI |
| Respuesta motor | Síntesis Mayeuta con «Tormentosa/Aiúa» — **success** |

```bash
.venv/bin/python docs/features/poc-interface-comunicacion/_browser-func-test-kalma2.py
```


- Síntesis **lab determinista** (sin LLM externo).
- Puente en instancia; promoción a genoma pendiente.
- Sin Cerbero/RBAC — bind `127.0.0.1`.
