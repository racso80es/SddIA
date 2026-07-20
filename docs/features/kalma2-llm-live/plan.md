---
feature_name: kalma2-llm-live
created: "2026-07-20"
process: feature
---

# Plan — kalma2-llm-live

## Línea de montaje

| Fase | Trabajo | Gate |
|------|---------|------|
| **1** | `kalma2-bridge`: `/api/chat` SSE + watchdog + emisión fractura; esqueleto `/api/execute` (reutilizar emisión EDA); política `/api/interact` | Tests bridge + smoke curl SSE con echo |
| **2** | Evolucionar `mayeuta-llm`: stream stdout; cablear bóveda; tests con simulador; dejar CLASSIFY fuera de aduana en handler kalma2 | Tests skill + handler |
| **3** | Dual-mode `kalma2-agent-runtime-cursor.py` (chat-stream); deps aisladas; preservar AGENT_PHASE | Smoke mock AGENT_PHASE + chat echo |
| **4** | UI: Botón Chat / Forjar Proceso | Smoke UI |
| **5** | E2E bus: execute → evento → status; chat SSE lab | AC1–AC5 |

## Orden de riesgo

1. SSE + watchdog (mayor valor perceptual).
2. Separación execute (corta regresión de intención LLM).
3. Prótesis dual-mode (no romper full-cycle B — AC5 primero en smoke).
4. UI al final (consumo de contratos estables).

## Skips lab (init)

`SDDIA_LAB_SKIP_PBI_ARCHIVE=1` · `SDDIA_LAB_SKIP_DELIVERY_CLOSE=1` hasta Argos APTO en rama.
